//! Results that worker threads post to a window. The message's `lParam` is a number naming the
//! result in a table here, not a pointer to it: any program of the same user can post the same
//! message number with a `lParam` of its choosing, and a pointer taken from that would be read
//! and freed. A number the table does not hold, for that message and type, is ignored.

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use windows_sys::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::PostMessageW;

/// The results waiting for their message: id -> (message, result).
type Parked = BTreeMap<usize, (u32, Box<dyn Any + Send>)>;

static PARKED: Mutex<Parked> = Mutex::new(BTreeMap::new());
/// The last id given; ids start at 1.
static LAST: AtomicUsize = AtomicUsize::new(0);

fn parked() -> std::sync::MutexGuard<'static, Parked> {
    PARKED.lock().unwrap_or_else(|e| e.into_inner())
}

/// Keeps `value` for `msg` and returns the `lParam` that names it.
fn park<T: Send + 'static>(msg: u32, value: T) -> LPARAM {
    let id = LAST.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
    parked().insert(id, (msg, Box::new(value)));
    id as LPARAM
}

/// Posts `msg` to `hwnd` carrying `value`, which the window takes back with [`take`]. False
/// when the post failed (the window is gone); `value` is then dropped.
pub fn post<T: Send + 'static>(hwnd: HWND, msg: u32, wparam: WPARAM, value: T) -> bool {
    let id = park(msg, value);
    if unsafe { PostMessageW(hwnd, msg, wparam, id) } == 0 {
        parked().remove(&(id as usize));
        return false;
    }
    true
}

/// The value [`post`] sent with `msg` and this `lParam`; `None` (and nothing taken) when no
/// such value of type `T` waits for `msg`: a message someone else posted.
pub fn take<T: 'static>(msg: u32, lparam: LPARAM) -> Option<T> {
    let mut parked = parked();
    let id = lparam as usize;
    match parked.get(&id) {
        Some((m, v)) if *m == msg && v.is::<T>() => {}
        _ => return None,
    }
    let (_, v) = parked.remove(&id)?;
    v.downcast::<T>().ok().map(|b| *b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MSG: u32 = 0x8000 + 900;

    #[test]
    fn a_parked_value_is_taken_once_by_its_message_and_type() {
        let id = park(MSG, String::from("result"));
        assert_eq!(take::<String>(MSG + 1, id), None, "another message");
        assert_eq!(take::<u32>(MSG, id), None, "another type");
        assert_eq!(take::<String>(MSG, id + 1_000_000), None, "an id never given");
        assert_eq!(take::<String>(MSG, 0), None);
        assert_eq!(take::<String>(MSG, -1), None, "a pointer-looking number");
        assert_eq!(take::<String>(MSG, id).as_deref(), Some("result"));
        assert_eq!(take::<String>(MSG, id), None, "taken once");
    }

    /// Through a hidden message-only window: what arrives names the value posted.
    #[test]
    fn a_posted_value_arrives_by_number() {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            CreateWindowExW, DestroyWindow, HWND_MESSAGE, MSG as Msg, PM_REMOVE, PeekMessageW,
        };
        let msg = MSG + 1;
        unsafe {
            let class = crate::util::wide("STATIC");
            let hwnd = CreateWindowExW(
                0,
                class.as_ptr(),
                std::ptr::null(),
                0,
                0,
                0,
                0,
                0,
                HWND_MESSAGE,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            );
            assert!(!hwnd.is_null());
            assert!(post(hwnd, msg, 5, vec![1u8, 2, 3]));
            let mut m: Msg = std::mem::zeroed();
            assert_ne!(PeekMessageW(&mut m, hwnd, msg, msg, PM_REMOVE), 0);
            assert_eq!(m.wParam, 5);
            assert_eq!(take::<Vec<u8>>(msg, m.lParam), Some(vec![1, 2, 3]));
            DestroyWindow(hwnd);
        }
    }

    #[test]
    fn a_failed_post_keeps_nothing() {
        // a message number of this test's own: other tests may park values meanwhile
        let msg = MSG + 2;
        let no_window = 0x7ffe_0001_isize as HWND;
        assert!(!post(no_window, msg, 0, 7u8));
        assert!(!parked().values().any(|(m, _)| *m == msg));
    }
}
