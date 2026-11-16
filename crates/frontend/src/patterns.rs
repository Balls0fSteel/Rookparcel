//! File name patterns: Create New Torrent's "Skip Files" field and Preferences' "Do not
//! download files named" list (D74).

/// Wildcard patterns (`*`, `?`) separated by `|`, `;` or line breaks, matched without case
/// against a whole file or folder name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkipPatterns(Vec<Vec<char>>);

impl SkipPatterns {
    pub fn parse(text: &str) -> SkipPatterns {
        SkipPatterns(
            text.split(['|', ';', '\n', '\r'])
                .map(str::trim)
                .filter(|p| !p.is_empty())
                .map(|p| p.to_lowercase().chars().collect())
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn matches(&self, name: &str) -> bool {
        let name: Vec<char> = name.to_lowercase().chars().collect();
        self.0.iter().any(|p| wildcard(p, &name))
    }
}

/// `*` matches any run (also empty), `?` one character.
fn wildcard(p: &[char], s: &[char]) -> bool {
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
}
