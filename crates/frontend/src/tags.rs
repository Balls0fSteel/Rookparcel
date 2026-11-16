//! Tags (D76): any number of names on a torrent, beside its one label, as qBittorrent has them.
//! The known tags exist on their own (one can be made with no torrent, and stays when its last
//! torrent loses it) until deleted; deleting one takes it off every torrent.
//!
//! A name is trimmed, not empty, without `,` and control characters, and at most
//! [`MAX_TAG_BYTES`] long (longer ones are cut at a character boundary); names compare
//! case-sensitively, as qBittorrent's do. [`normalize`] is the one check every interface and
//! the API use; [`parse_list`] reads a comma-separated list. At most [`MAX_TAGS`] are known.
//!
//! The preferences keep a torrent's tags as `torrent.<key>.tags` ([`torrent_key`], `key` being
//! `InfoHashes::key_hex`) and the known ones as `tags.list` ([`KNOWN_KEY`]), both comma-separated
//! in [`cmp`] order. A name can break out of neither its line nor its key: it holds no line
//! break (no control character) and no comma, and it is only ever a value.
//!
//! The free functions below the model's ([`TorrentList`]'s tag methods) are what a menu or the
//! sidebar does to a selection, shared by the GUIs that own their `App` (Win32, GTK) and the
//! Mac's [`crate::app::Core`]: they change the list and the preferences; the caller saves the
//! preferences.

use std::cmp::Ordering;

use rookparcel_core::TorrentId;

use crate::columns::natural_cmp;
use crate::model::TorrentList;
use crate::prefs::Prefs;

/// The longest name in bytes; a longer one is cut at a character boundary.
pub const MAX_TAG_BYTES: usize = 128;

/// The most tags that can be known.
pub const MAX_TAGS: usize = 1024;

/// The preferences key of the known tags.
pub const KNOWN_KEY: &str = "tags.list";

/// The preferences key of a torrent's tags; `key` is its `InfoHashes::key_hex`.
pub fn torrent_key(key: &str) -> String {
    format!("torrent.{key}.tags")
}

/// The torrent's `key` of a [`torrent_key`] (a preferences change the Web UI hands on).
pub fn key_of(pref_key: &str) -> Option<&str> {
    pref_key.strip_prefix("torrent.")?.strip_suffix(".tags").filter(|k| !k.is_empty() && !k.contains('.'))
}

/// Why a name or a new tag is refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagError {
    /// Empty (or only spaces), or with a comma or a control character.
    Invalid,
    /// [`MAX_TAGS`] are known already.
    TooMany,
}

impl TagError {
    /// The message to show (translated).
    pub fn text(self) -> String {
        match self {
            TagError::Invalid => crate::tr!(TagInvalid).to_owned(),
            TagError::TooMany => crate::tr!(TagTooMany, max = MAX_TAGS),
        }
    }
}

impl std::fmt::Display for TagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text())
    }
}

/// `name` as a tag: trimmed and cut to [`MAX_TAG_BYTES`] (at a character boundary, then trimmed
/// again); `Err` when it is empty or holds a comma or a control character.
pub fn normalize(name: &str) -> Result<String, TagError> {
    let name = name.trim();
    if name.is_empty() || name.contains(',') || name.chars().any(char::is_control) {
        return Err(TagError::Invalid);
    }
    let mut end = name.len().min(MAX_TAG_BYTES);
    while !name.is_char_boundary(end) {
        end -= 1;
    }
    let name = name[..end].trim_end();
    if name.is_empty() {
        return Err(TagError::Invalid);
    }
    Ok(name.to_owned())
}

/// The valid names of a comma-separated list, each once, in [`cmp`] order; invalid ones are
/// dropped.
pub fn parse_list(text: &str) -> Vec<String> {
    let mut v: Vec<String> = text.split(',').filter_map(|n| normalize(n).ok()).collect();
    sort(&mut v);
    v
}

/// The order tags are shown and kept in: natural and ignoring case first (as labels sort), then
/// case-sensitively, so `tv` and `TV` (two tags) always come in the same order.
pub fn cmp(a: &str, b: &str) -> Ordering {
    natural_cmp(a, b).then_with(|| a.cmp(b))
}

/// Sorts `tags` by [`cmp`] and drops duplicates.
pub fn sort(tags: &mut Vec<String>) {
    tags.sort_by(|a, b| cmp(a, b));
    tags.dedup();
}

/// Two tag lists in order: tag by tag, a shorter list (none at all first) before a longer one
/// it begins.
pub fn cmp_lists(a: &[String], b: &[String]) -> Ordering {
    for (x, y) in a.iter().zip(b) {
        let o = cmp(x, y);
        if o != Ordering::Equal {
            return o;
        }
    }
    a.len().cmp(&b.len())
}

/// The stored form: the names joined by `,`.
pub fn join(tags: &[String]) -> String {
    tags.join(",")
}

/// The Tags column's text: the names joined by `, `.
pub fn display(tags: &[String]) -> String {
    tags.join(", ")
}

/// A tag's text as a menu item's (`&` marks a mnemonic there, so a name's own `&` is doubled;
/// GTK takes it through [`crate::lang::gtk_mnemonic`]).
pub fn menu_text(name: &str) -> String {
    name.replace('&', "&&")
}

/// A tag's state across a selection: the check mark of its item in Tags ▸.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagState {
    /// Every selected torrent has it.
    On,
    /// None has it.
    Off,
    /// Some have it.
    Mixed,
}

// ---- the preferences --------------------------------------------------------------------------

/// The known tags kept in the preferences.
pub fn load_known(prefs: &Prefs) -> Vec<String> {
    let mut v = parse_list(prefs.get(KNOWN_KEY).unwrap_or_default());
    v.truncate(MAX_TAGS);
    v
}

/// Gives `list` the known tags kept in `prefs`: at start, before the torrents come.
pub fn load_into(list: &mut TorrentList, prefs: &Prefs) {
    list.set_known_tags(load_known(prefs));
}

/// Writes `list`'s known tags to `prefs` (removes the key when there are none).
pub fn store_known(list: &TorrentList, prefs: &mut Prefs) {
    let tags = list.tags();
    if tags.is_empty() {
        prefs.remove(KNOWN_KEY);
    } else {
        prefs.set(KNOWN_KEY, join(tags));
    }
}

/// A torrent's first sight (its first status, before `list.update`): the tags the preferences
/// keep for it go into `list`; a name not known yet becomes known (an import or the Web UI
/// wrote it), and `tags.list` follows. `key` is its `InfoHashes::key_hex`. True when `prefs`
/// changed (the caller saves them).
pub fn on_first_sight(list: &mut TorrentList, prefs: &mut Prefs, id: TorrentId, key: &str) -> bool {
    let Some(text) = prefs.get(&torrent_key(key)).map(str::to_owned) else { return false };
    let tags = parse_list(&text);
    let known = list.tags().len();
    list.set_tags(id, &tags);
    let mut changed = false;
    if list.tags().len() != known {
        store_known(list, prefs);
        changed = true;
    }
    // a name that was not valid, or did not fit, is dropped from the file too
    if list.tags_of(id) != tags.as_slice() || join(&tags) != text {
        store_torrent(list, prefs, id, key);
        changed = true;
    }
    changed
}

/// Makes `names` known in the preferences' `tags.list` while fewer than [`MAX_TAGS`] are (an
/// import, before its torrents come); invalid names are dropped.
pub fn learn(prefs: &mut Prefs, names: &[String]) {
    let mut list = TorrentList::new();
    load_into(&mut list, prefs);
    let before = list.tags().len();
    for n in names {
        let _ = list.create_tag(n);
    }
    if list.tags().len() != before {
        store_known(&list, prefs);
    }
}

/// Writes torrent `id`'s tags from `list` to `prefs` under `key` (its `InfoHashes::key_hex`);
/// none removes the key.
pub fn store_torrent(list: &TorrentList, prefs: &mut Prefs, id: TorrentId, key: &str) {
    let tags = list.tags_of(id);
    if tags.is_empty() {
        prefs.remove(&torrent_key(key));
    } else {
        prefs.set(&torrent_key(key), join(tags));
    }
}

/// A removed torrent's tags leave the preferences (the tags stay known).
pub fn forget_torrent(prefs: &mut Prefs, key: &str) {
    prefs.remove(&torrent_key(key));
}

/// Writes the tags of `ids` (those in `list`) to `prefs`.
fn store_all(list: &TorrentList, prefs: &mut Prefs, ids: &[TorrentId]) {
    for &id in ids {
        if let Some(key) = list.get(id).map(|s| s.info_hashes.key_hex()) {
            store_torrent(list, prefs, id, &key);
        }
    }
}

/// A preferences change made elsewhere (the Web UI a GUI hosts hands its changes to the window:
/// `torrent.<key>.tags` or `tags.list`) applied to `list`; `id_of` finds a torrent by its key
/// (`None`: not seen yet, it takes its tags at its first sight). True when the list changed.
pub fn apply_change(
    list: &mut TorrentList,
    key: &str,
    value: Option<&str>,
    id_of: impl Fn(&str) -> Option<TorrentId>,
) -> bool {
    if key == KNOWN_KEY {
        let before = list.generation();
        list.set_known_tags(parse_list(value.unwrap_or_default()));
        return list.generation() != before;
    }
    let Some(id) = key_of(key).and_then(id_of) else { return false };
    list.set_tags(id, &parse_list(value.unwrap_or_default()))
}

// ---- a selection's menu ----------------------------------------------------------------------

/// Tag `tag`'s state across `ids`.
pub fn state(list: &TorrentList, ids: &[TorrentId], tag: &str) -> TagState {
    let have = ids.iter().filter(|&&id| list.tags_of(id).iter().any(|t| t == tag)).count();
    match have {
        0 => TagState::Off,
        n if n == ids.len() => TagState::On,
        _ => TagState::Mixed,
    }
}

/// Every known tag with its state across `ids`, in order: Tags ▸'s items.
pub fn states(list: &TorrentList, ids: &[TorrentId]) -> Vec<(String, TagState)> {
    list.tags().iter().map(|t| (t.clone(), state(list, ids, t))).collect()
}

/// Some torrent of `ids` has a tag: Remove All Tags is enabled.
pub fn any_tagged(list: &TorrentList, ids: &[TorrentId]) -> bool {
    ids.iter().any(|&id| !list.tags_of(id).is_empty())
}

/// Gives `ids` the tag `name` (made known when it is new). `Err` for an invalid name, or a new
/// one with [`MAX_TAGS`] known.
pub fn add(list: &mut TorrentList, prefs: &mut Prefs, ids: &[TorrentId], name: &str) -> Result<(), TagError> {
    let tag = create(list, prefs, name)?;
    for &id in ids {
        list.add_tag(id, &tag);
    }
    store_all(list, prefs, ids);
    Ok(())
}

/// Takes the tag `name` off `ids` (it stays known).
pub fn remove(list: &mut TorrentList, prefs: &mut Prefs, ids: &[TorrentId], name: &str) {
    for &id in ids {
        list.remove_tag(id, name);
    }
    store_all(list, prefs, ids);
}

/// A tag's item in Tags ▸: on for all unless every torrent has it, then off for all (a mixed
/// tag goes on for all), as the flags' items toggle.
pub fn toggle(list: &mut TorrentList, prefs: &mut Prefs, ids: &[TorrentId], name: &str) {
    if state(list, ids, name) == TagState::On {
        remove(list, prefs, ids, name);
    } else {
        // a known tag: only an unknown name could fail, and then nothing changes
        let _ = add(list, prefs, ids, name);
    }
}

/// Remove All Tags: `ids` lose every tag (the tags stay known).
pub fn clear(list: &mut TorrentList, prefs: &mut Prefs, ids: &[TorrentId]) {
    for &id in ids {
        list.set_tags(id, &[]);
    }
    store_all(list, prefs, ids);
}

/// New Tag: makes `name` known (with no torrent); its normalized name. An existing tag is no
/// error. `Err` for an invalid name, or a new one with [`MAX_TAGS`] known.
pub fn create(list: &mut TorrentList, prefs: &mut Prefs, name: &str) -> Result<String, TagError> {
    let before = list.tags().len();
    let tag = list.create_tag(name)?;
    if list.tags().len() != before {
        store_known(list, prefs);
    }
    Ok(tag)
}

/// Delete Tag: `name` is no longer known and every torrent loses it, in the list and in the
/// preferences (a `torrent.<key>.tags` of a torrent not in the list too, so the name cannot come
/// back at that torrent's first sight). False when it was not known.
pub fn delete(list: &mut TorrentList, prefs: &mut Prefs, name: &str) -> bool {
    let Some(lost) = list.delete_tag(name) else { return false };
    store_all(list, prefs, &lost);
    forget_everywhere(prefs, name);
    store_known(list, prefs);
    true
}

/// Takes `name` out of every `torrent.<key>.tags` in `prefs` (a key left empty goes).
pub fn forget_everywhere(prefs: &mut Prefs, name: &str) {
    forget_all_everywhere(prefs, |t| t == name);
}

/// Takes every tag `gone` says of out of every `torrent.<key>.tags` in `prefs`, in one pass
/// however many names go (a key left empty goes).
pub fn forget_all_everywhere(prefs: &mut Prefs, gone: impl Fn(&str) -> bool) {
    let changed: Vec<(String, Vec<String>)> = prefs
        .entries()
        .filter(|(k, _)| key_of(k).is_some())
        .filter_map(|(k, v)| {
            let tags = parse_list(v);
            tags.iter().any(|t| gone(t)).then(|| (k.to_owned(), tags))
        })
        .collect();
    for (k, mut tags) in changed {
        tags.retain(|t| !gone(t));
        if tags.is_empty() {
            prefs.remove(&k);
        } else {
            prefs.set(&k, join(&tags));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rookparcel_core::{InfoHashes, Sha1Hash, TorrentStatus};

    fn st(raw: u64) -> TorrentStatus {
        TorrentStatus {
            id: TorrentId::from_raw(raw),
            info_hashes: InfoHashes::v1(Sha1Hash([raw as u8; 20])),
            name: format!("t{raw}"),
            ..TorrentStatus::default()
        }
    }

    fn id(raw: u64) -> TorrentId {
        TorrentId::from_raw(raw)
    }

    fn key(raw: u64) -> String {
        InfoHashes::v1(Sha1Hash([raw as u8; 20])).key_hex()
    }

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn names_are_checked_and_normalized() {
        assert_eq!(normalize("  tv  "), Ok("tv".to_owned()));
        assert_eq!(normalize("TV"), Ok("TV".to_owned()), "case is kept");
        assert_eq!(normalize(" a b "), Ok("a b".to_owned()));
        for bad in ["", "   ", "a,b", ",", "a\nb", "a\rb", "a\tb", "\u{7f}", "x\u{0}"] {
            assert_eq!(normalize(bad), Err(TagError::Invalid), "{bad:?}");
        }
        // cut at 128 bytes, on a character boundary
        assert_eq!(normalize(&"a".repeat(200)).unwrap().len(), MAX_TAG_BYTES);
        let long = format!("a{}", "é".repeat(100));
        let cut = normalize(&long).unwrap();
        assert!(cut.len() <= MAX_TAG_BYTES && cut.len() == 127, "{}", cut.len());
        assert!(long.starts_with(&cut));
        // the cut leaves trailing spaces: trimmed again
        let spaced = format!("{} {}", "a".repeat(127), "b");
        assert_eq!(normalize(&spaced).unwrap(), "a".repeat(127));
        assert_eq!(TagError::Invalid.text(), crate::tr!(TagInvalid));
        assert!(TagError::TooMany.text().contains("1024"));
    }

    #[test]
    fn lists_parse_sorted_and_without_duplicates() {
        assert_eq!(parse_list(" b , A,a,,b, ,x\ny, ep10,ep2"), names(&["A", "a", "b", "ep2", "ep10"]));
        assert!(parse_list("").is_empty());
        assert_eq!(cmp("tv", "TV"), Ordering::Greater);
        assert_eq!(cmp("Movies", "tv"), Ordering::Less);
        assert_eq!(cmp_lists(&[], &names(&["a"])), Ordering::Less);
        assert_eq!(cmp_lists(&names(&["a"]), &names(&["a", "b"])), Ordering::Less);
        assert_eq!(cmp_lists(&names(&["b"]), &names(&["a", "b"])), Ordering::Greater);
        assert_eq!(display(&names(&["a", "b"])), "a, b");
        assert_eq!(menu_text("R&B"), "R&&B");
        assert_eq!(key_of("torrent.abc.tags"), Some("abc"));
        assert_eq!(key_of("torrent.abc.label"), None);
        assert_eq!(key_of("torrent..tags"), None);
        assert_eq!(key_of(KNOWN_KEY), None);
    }

    #[test]
    fn menu_helpers_on_a_selection() {
        let mut l = TorrentList::new();
        let mut p = Prefs::default();
        l.update([st(1), st(2), st(3)]);
        let all = [id(1), id(2), id(3)];
        assert!(!any_tagged(&l, &all));
        add(&mut l, &mut p, &[id(1)], " tv ").unwrap();
        assert_eq!(p.get(KNOWN_KEY), Some("tv"));
        assert_eq!(p.get(&torrent_key(&key(1))), Some("tv"));
        assert_eq!(state(&l, &all, "tv"), TagState::Mixed);
        assert_eq!(state(&l, &[id(1)], "tv"), TagState::On);
        assert_eq!(state(&l, &[id(2)], "tv"), TagState::Off);
        assert!(any_tagged(&l, &all));
        // mixed: on for all; then off for all
        toggle(&mut l, &mut p, &all, "tv");
        assert!(all.iter().all(|&i| l.tags_of(i) == ["tv"]));
        toggle(&mut l, &mut p, &all, "tv");
        assert!(all.iter().all(|&i| l.tags_of(i).is_empty()));
        assert!(p.get(&torrent_key(&key(2))).is_none());
        assert_eq!(l.tags(), ["tv"], "still known");

        assert_eq!(add(&mut l, &mut p, &all, "a,b"), Err(TagError::Invalid));
        create(&mut l, &mut p, "Movies").unwrap();
        assert_eq!(create(&mut l, &mut p, "Movies"), Ok("Movies".to_owned()), "an existing one is fine");
        assert_eq!(p.get(KNOWN_KEY), Some("Movies,tv"));
        add(&mut l, &mut p, &[id(2)], "Movies").unwrap();
        add(&mut l, &mut p, &[id(2), id(3)], "tv").unwrap();
        assert_eq!(p.get(&torrent_key(&key(2))), Some("Movies,tv"));
        assert_eq!(
            states(&l, &[id(2), id(3)]),
            [("Movies".to_owned(), TagState::Mixed), ("tv".to_owned(), TagState::On)]
        );
        clear(&mut l, &mut p, &[id(2)]);
        assert!(l.tags_of(id(2)).is_empty() && p.get(&torrent_key(&key(2))).is_none());
        assert_eq!(l.tags_of(id(3)), ["tv"]);
        remove(&mut l, &mut p, &[id(3)], "tv");
        assert!(l.tags_of(id(3)).is_empty());

        // delete: gone everywhere
        add(&mut l, &mut p, &all, "tv").unwrap();
        assert!(delete(&mut l, &mut p, "tv"));
        assert!(!delete(&mut l, &mut p, "tv"));
        assert_eq!(l.tags(), ["Movies"]);
        assert_eq!(p.get(KNOWN_KEY), Some("Movies"));
        assert!(
            all.iter().all(|&i| l.tags_of(i).is_empty() && p.get(&torrent_key(&key(i.to_raw()))).is_none())
        );
        assert!(delete(&mut l, &mut p, "Movies"));
        assert!(p.get(KNOWN_KEY).is_none());
    }

    #[test]
    fn at_most_max_tags_are_known() {
        let mut l = TorrentList::new();
        let mut p = Prefs::default();
        l.update([st(1)]);
        for i in 0..MAX_TAGS {
            create(&mut l, &mut p, &format!("t{i}")).unwrap();
        }
        assert_eq!(create(&mut l, &mut p, "one more"), Err(TagError::TooMany));
        assert_eq!(add(&mut l, &mut p, &[id(1)], "one more"), Err(TagError::TooMany));
        assert!(l.tags_of(id(1)).is_empty());
        // a known one still goes on
        add(&mut l, &mut p, &[id(1)], "t5").unwrap();
        assert_eq!(l.tags_of(id(1)), ["t5"]);
        // set_tags drops the names that do not fit
        l.set_tags(id(1), &names(&["t5", "new"]));
        assert_eq!(l.tags_of(id(1)), ["t5"]);
        assert_eq!(load_known(&p).len(), MAX_TAGS);
    }

    /// The tags go through the preferences file and back, hostile names included: none breaks
    /// out of its line or key.
    #[test]
    fn tags_survive_the_file_and_leave_with_the_torrent() {
        let mut l = TorrentList::new();
        let mut p = Prefs::default();
        let hostile = ["x\ny=1", "a\r\ntorrent.00.label=evil", "#comment", "=", "k=v", " sp ", "é ü", "[tv]"];
        let mut accepted = Vec::new();
        for h in hostile {
            if let Ok(t) = create(&mut l, &mut p, h) {
                accepted.push(t);
            }
        }
        assert_eq!(accepted, names(&["#comment", "=", "k=v", "sp", "é ü", "[tv]"]));
        l.update([st(1), st(2)]);
        for t in &accepted {
            add(&mut l, &mut p, &[id(1)], t).unwrap();
        }
        add(&mut l, &mut p, &[id(2)], "only on two").unwrap();
        let text = p.to_text();
        assert_eq!(text.lines().count(), 4, "a comment line and three keys:\n{text}");
        assert!(!text.contains("evil") && !text.contains("y=1"));

        // a new start: the file read back, the torrents seen again
        let mut q = Prefs::from_text(&text);
        let mut l2 = TorrentList::new();
        load_into(&mut l2, &q);
        assert_eq!(l2.tags(), l.tags());
        assert!(!on_first_sight(&mut l2, &mut q, id(1), &key(1)), "nothing to write back");
        on_first_sight(&mut l2, &mut q, id(2), &key(2));
        l2.update([st(1), st(2)]);
        assert_eq!(l2.tags_of(id(1)), l.tags_of(id(1)));
        assert_eq!(l2.tags_of(id(2)), ["only on two"]);
        assert_eq!(q.to_text(), text);

        // removed with the torrent: its key goes, the tags stay known
        l2.remove(id(2));
        forget_torrent(&mut q, &key(2));
        assert!(q.get(&torrent_key(&key(2))).is_none());
        assert!(l2.tags().iter().any(|t| t == "only on two"));
        assert_eq!(l2.tag_counts().iter().find(|(t, _)| t == "only on two").map(|x| x.1), Some(0));
    }

    /// A tag written by an import or the Web UI before the torrent came, unknown and untidy.
    #[test]
    fn first_sight_makes_new_tags_known_and_tidies_the_value() {
        let mut l = TorrentList::new();
        let mut p = Prefs::default();
        p.set(&torrent_key(&key(1)), " tv,,Movies,tv ");
        assert!(on_first_sight(&mut l, &mut p, id(1), &key(1)));
        assert_eq!(l.tags(), ["Movies", "tv"]);
        assert_eq!(p.get(KNOWN_KEY), Some("Movies,tv"));
        assert_eq!(p.get(&torrent_key(&key(1))), Some("Movies,tv"));
        assert!(!on_first_sight(&mut l, &mut p, id(2), &key(2)), "no key, nothing");
        assert!(l.tags_of(id(2)).is_empty());
    }

    /// Deleting a tag takes it out of a torrent's key the list does not hold (a torrent removed
    /// while the program was closed, or not seen yet), so it cannot come back at a first sight.
    #[test]
    fn delete_reaches_torrents_not_in_the_list() {
        let mut l = TorrentList::new();
        let mut p = Prefs::default();
        l.update([st(1)]);
        add(&mut l, &mut p, &[id(1)], "tv").unwrap();
        p.set(&torrent_key(&key(7)), "Movies,tv");
        p.set(&torrent_key(&key(8)), "tv");
        p.set("torrent.ab.label", "tv");
        assert!(delete(&mut l, &mut p, "tv"));
        assert!(p.get(&torrent_key(&key(1))).is_none());
        assert_eq!(p.get(&torrent_key(&key(7))), Some("Movies"));
        assert!(p.get(&torrent_key(&key(8))).is_none(), "left empty: gone");
        assert_eq!(p.get("torrent.ab.label"), Some("tv"), "a label is not a tag");
        assert!(p.get(KNOWN_KEY).is_none());
        // the torrent seen later does not bring it back
        l.update([st(7)]);
        assert!(on_first_sight(&mut l, &mut p, id(7), &key(7)), "Movies becomes known");
        assert_eq!(l.tags(), ["Movies"], "tv does not come back");
        assert_eq!(l.tags_of(id(7)), ["Movies"]);
    }

    /// Hosted, `tags.list` travels whole: an action that leaves the known tags alone must not
    /// write it, or a tag a browser made meanwhile would be overwritten.
    #[test]
    fn the_known_list_is_written_only_when_it_changes() {
        let mut l = TorrentList::new();
        let mut p = Prefs::default();
        l.update([st(1), st(2)]);
        create(&mut l, &mut p, "tv").unwrap();
        create(&mut l, &mut p, "Movies").unwrap();
        p.track_changes();
        add(&mut l, &mut p, &[id(1), id(2)], "tv").unwrap();
        toggle(&mut l, &mut p, &[id(1)], "Movies");
        remove(&mut l, &mut p, &[id(2)], "tv");
        clear(&mut l, &mut p, &[id(1)]);
        create(&mut l, &mut p, " tv ").unwrap();
        let changes = p.take_changes();
        assert!(!changes.is_empty());
        assert!(changes.iter().all(|(k, _)| k != KNOWN_KEY), "{changes:?}");
        create(&mut l, &mut p, "new").unwrap();
        assert_eq!(p.take_changes(), [(KNOWN_KEY.to_owned(), Some("Movies,new,tv".to_owned()))]);
    }

    #[test]
    fn changes_from_the_web_ui_apply_to_the_list() {
        let mut l = TorrentList::new();
        l.update([st(1)]);
        let of = |k: &str| (k == key(1)).then_some(id(1));
        assert!(apply_change(&mut l, &torrent_key(&key(1)), Some("b,a"), of));
        assert_eq!(l.tags_of(id(1)), ["a", "b"]);
        assert_eq!(l.tags(), ["a", "b"]);
        assert!(!apply_change(&mut l, &torrent_key("ffff"), Some("c"), of), "not seen yet");
        assert!(apply_change(&mut l, KNOWN_KEY, Some("a,b,c"), of));
        assert_eq!(l.tags(), ["a", "b", "c"]);
        assert!(apply_change(&mut l, &torrent_key(&key(1)), None, of));
        assert!(l.tags_of(id(1)).is_empty());
        assert!(!apply_change(&mut l, "torrent.x.label", Some("y"), of));
    }
}
