//! File name patterns: Create New Torrent's "Skip Files" field and Preferences' "Do not
//! download files named" list (D74).

/// Wildcard patterns (`*`, `?`) separated by `|`, `;` or line breaks, matched without case
/// against a whole file or folder name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkipPatterns(Vec<Pattern>);

impl SkipPatterns {
    pub fn parse(text: &str) -> SkipPatterns {
        SkipPatterns(
            text.split(['|', ';', '\n', '\r'])
                .map(str::trim)
                .filter(|p| !p.is_empty())
                .map(|p| Pattern::new(&p.to_lowercase()))
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn matches(&self, name: &str) -> bool {
        let name: Vec<char> = name.to_lowercase().chars().collect();
        self.0.iter().any(|p| p.matches(&name))
    }
}

/// One pattern, `*` matching any run (also empty) and `?` one character, compiled for a
/// bit-parallel run of its automaton: a name costs its length times the pattern's length / 64
/// whatever the two hold, where matching with backtracking costs up to their product (hostile
/// file names against a long list, N3).
#[derive(Clone, Debug, PartialEq, Eq)]
struct Pattern {
    /// Positions (0..=len; `len` is the match) in 64-bit words.
    words: usize,
    len: usize,
    /// The positions holding a `*` (runs of stars are one).
    star: Vec<u64>,
    /// The positions holding a `?`.
    any: Vec<u64>,
    /// Per character of the pattern, sorted: the positions holding it.
    chars: Vec<(char, Vec<u64>)>,
    /// The characters a name needs at least (all but the stars).
    fixed: usize,
}

impl Pattern {
    fn new(p: &str) -> Pattern {
        let mut chars: Vec<char> = Vec::new();
        for c in p.chars() {
            if !(c == '*' && chars.last() == Some(&'*')) {
                chars.push(c);
            }
        }
        let (len, words) = (chars.len(), chars.len() / 64 + 1);
        let bits = |f: &dyn Fn(char) -> bool| {
            let mut v = vec![0u64; words];
            for (i, &c) in chars.iter().enumerate() {
                if f(c) {
                    v[i / 64] |= 1 << (i % 64);
                }
            }
            v
        };
        let mut distinct: Vec<char> = chars.iter().copied().filter(|&c| c != '*' && c != '?').collect();
        distinct.sort_unstable();
        distinct.dedup();
        Pattern {
            words,
            len,
            star: bits(&|c| c == '*'),
            any: bits(&|c| c == '?'),
            chars: distinct.iter().map(|&d| (d, bits(&|c| c == d))).collect(),
            fixed: chars.iter().filter(|&&c| c != '*').count(),
        }
    }

    fn matches(&self, s: &[char]) -> bool {
        if s.len() < self.fixed || (self.star.iter().all(|&w| w == 0) && s.len() != self.fixed) {
            return false;
        }
        let mut d = vec![0u64; self.words];
        d[0] = 1;
        self.close(&mut d);
        let mut next = vec![0u64; self.words];
        for c in s {
            let lit = self.chars.binary_search_by_key(c, |e| e.0).ok().map(|i| &self.chars[i].1);
            // a step on a matching character or `?`; a star stays where it is
            let mut carry = 0;
            for w in 0..self.words {
                let m = self.any[w] | lit.map_or(0, |l| l[w]);
                let stepped = d[w] & m;
                next[w] = (stepped << 1) | carry | (d[w] & self.star[w]);
                carry = stepped >> 63;
            }
            std::mem::swap(&mut d, &mut next);
            self.close(&mut d);
            if d.iter().all(|&w| w == 0) {
                return false;
            }
        }
        d[self.len / 64] & (1 << (self.len % 64)) != 0
    }

    /// A star also matches nothing: from each star to the position after it (one step, as no
    /// two stars follow each other).
    fn close(&self, d: &mut [u64]) {
        let mut carry = 0;
        for (word, &star) in d.iter_mut().zip(&self.star) {
            let at = *word & star;
            *word |= (at << 1) | carry;
            carry = at >> 63;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_patterns() {
        let s = SkipPatterns::parse("*.TXT| thumbs.db ;a?c*");
        assert!(s.matches("readme.txt"));
        assert!(s.matches("Thumbs.db"));
        assert!(s.matches("abc"));
        assert!(s.matches("axcdef"));
        assert!(!s.matches("ac"));
        assert!(!s.matches("readme.txt.bak"));
        assert!(SkipPatterns::parse(" | ").is_empty());
        assert!(SkipPatterns::parse("*").matches(""));
    }

    #[test]
    fn line_breaks_separate_patterns_too() {
        // the qBittorrent API sends the list one pattern per line
        let s = SkipPatterns::parse("*.lnk\r\nThumbs.db\n\n*sample*\n");
        assert_eq!(s, SkipPatterns::parse("*.lnk; Thumbs.db; *sample*"));
        assert!(s.matches("Setup.LNK") && s.matches("thumbs.db") && s.matches("Movie.SAMPLE.mkv"));
        assert!(!s.matches("movie.mkv") && !s.matches("Thumbs.db.txt"));
        // a whole name: no part of one, no path
        assert!(!SkipPatterns::parse("sample").matches("a sample.mkv"));
        assert!(SkipPatterns::parse("\n\r\n").is_empty());
        // case-insensitive beyond ASCII, a ? is one character, not one byte
        assert!(SkipPatterns::parse("ÄRGER?.TXT").matches("ärgerß.txt"));
    }

    /// The matcher before N3 (greedy, backtracking to the last star), as the reference.
    fn backtracking(p: &[char], s: &[char]) -> bool {
        let (mut pi, mut si) = (0, 0);
        let mut star: Option<(usize, usize)> = None;
        while si < s.len() {
            if pi < p.len() && (p[pi] == '?' || p[pi] == s[si]) {
                pi += 1;
                si += 1;
            } else if pi < p.len() && p[pi] == '*' {
                star = Some((pi, si));
                pi += 1;
            } else if let Some((sp, ss)) = star {
                pi = sp + 1;
                si = ss + 1;
                star = Some((sp, ss + 1));
            } else {
                return false;
            }
        }
        p[pi..].iter().all(|&c| c == '*')
    }

    /// Every pattern and name over a small alphabet, up to five characters each, against the
    /// old matcher; and patterns across the 64-position words.
    #[test]
    fn the_same_answers_as_backtracking() {
        let alphabet = ['a', 'b', '*', '?'];
        let all = |max: usize, of: &[char]| {
            let mut out = vec![String::new()];
            let mut level = vec![String::new()];
            for _ in 0..max {
                level = level.iter().flat_map(|s| of.iter().map(move |c| format!("{s}{c}"))).collect();
                out.extend(level.iter().cloned());
            }
            out
        };
        let names = all(5, &['a', 'b']);
        for p in all(5, &alphabet) {
            let (compiled, pc) = (Pattern::new(&p), p.chars().collect::<Vec<_>>());
            for n in &names {
                let nc: Vec<char> = n.chars().collect();
                assert_eq!(compiled.matches(&nc), backtracking(&pc, &nc), "{p:?} {n:?}");
            }
        }
        let long_name: String = "ab".repeat(100);
        for p in [
            "a*b*".repeat(40),
            format!("{}*", "?".repeat(63)),
            format!("*{}", "ab".repeat(70)),
            "*".repeat(200),
        ] {
            let (pc, nc): (Vec<char>, Vec<char>) = (p.chars().collect(), long_name.chars().collect());
            for cut in [0, 1, 63, 64, 65, 127, 128, 129, 199, 200] {
                assert_eq!(Pattern::new(&p).matches(&nc[..cut]), backtracking(&pc, &nc[..cut]), "{p} {cut}");
            }
        }
    }

    /// The worst case of backtracking (a star, then about half the name's length that matches
    /// all but at its end, tried from every place) costs a name its length times the pattern's
    /// / 64 here: a full list of such patterns (about 4 KiB) against 2,000 names of the longest
    /// kind a torrent can carry (240 characters) answers well within the bound, which leaves
    /// room for a slow debug build on a busy machine.
    #[test]
    fn a_hostile_list_against_hostile_names_stays_fast() {
        let worst =
            [format!("*{}b", "a".repeat(119)), format!("*{}?b", "a?".repeat(59)), "*a*a*a*a*a*c".into()];
        let list = worst.iter().cycle().take(33).cloned().collect::<Vec<_>>().join(";");
        assert!(list.len() <= crate::exclude::MAX_BYTES);
        let s = SkipPatterns::parse(&list);
        let name = "a".repeat(240);
        let start = std::time::Instant::now();
        for _ in 0..2_000 {
            assert!(!s.matches(&name));
        }
        assert!(start.elapsed() < std::time::Duration::from_secs(60), "{:?}", start.elapsed());
    }
}
