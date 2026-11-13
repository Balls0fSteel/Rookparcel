//! The Dock icon: a click with the window hidden in the status item (or minimized into it)
//! brings it back (`applicationShouldHandleReopen:hasVisibleWindows:`). The Dock menu is
//! `menu::dock`, built by `RPAppDelegate` each time it opens, and runs its commands through
//! `RPTarget` like every other menu.
//!
//! D69: while the windows are locked a click asks for the PIN; with Option held and a PIN set
//! it asks the lock's reset question first (`lock::reopen`).

/// `applicationShouldHandleReopen:hasVisibleWindows:`.
pub fn reopen(has_visible_windows: bool) -> bool {
    if crate::lock::reopen() {
        return true;
    }
    if !has_visible_windows {
        super::lifecycle::show_window();
    }
    true
}
