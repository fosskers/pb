extern crate winapi;
extern crate kernel32;

use super::{Width, Height};

/// Returns the size of the terminal, if available.
///
/// Note that this returns the size of the actual command window, and
/// not the overall size of the command window buffer
pub fn terminal_size() -> Option<(Width, Height)> {
    if let Some((_, csbi)) = get_csbi() {
        let w: Width = Width((csbi.srWindow.Right - csbi.srWindow.Left) as u16);
        let h: Height = Height((csbi.srWindow.Bottom - csbi.srWindow.Top) as u16);
        Some((w, h))
    } else {
        None
    }
}

fn move_cursor_up(n: usize) -> String {
    use self::kernel32::SetConsoleCursorPosition;
    use self::winapi::COORD;
    if let Some((hand, csbi)) = get_csbi() {
        unsafe {
            SetConsoleCursorPosition(hand, COORD {
                X: 0 as i16,
                Y: csbi.dwCursorPosition.Y as i16 - n,
            });
        }
    }
    "".to_string()
}

fn get_csbi() -> Option<(self::winapi::HANDLE, self::winapi::CONSOLE_SCREEN_BUFFER_INFO)> {
    use self::winapi::HANDLE;
    use self::kernel32::{GetStdHandle, GetConsoleScreenBufferInfo};
    use self::winapi::STD_OUTPUT_HANDLE;
    use self::winapi::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

    let hand: HANDLE = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let zc = COORD { X: 0, Y: 0 };
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: zc.clone(),
        dwCursorPosition: zc.clone(),
        wAttributes: 0,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: zc,
    };
    match unsafe { GetConsoleScreenBufferInfo(hand, &mut csbi) } {
        0 => None,
        _ => Some((hand, csbi)),
    }
}
