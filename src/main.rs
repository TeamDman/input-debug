use std::{thread, time::Duration};
use std::io::{stdout, Write};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_LBUTTON, VK_RBUTTON, VK_CANCEL, VK_MBUTTON,
    VK_XBUTTON1, VK_XBUTTON2, VK_BACK, VK_TAB, VK_CLEAR, VK_RETURN, VK_SHIFT,
    VK_CONTROL, VK_MENU, VK_PAUSE, VK_CAPITAL, VK_KANA, VK_HANGUL, VK_JUNJA,
    VK_FINAL, VK_HANJA, VK_KANJI, VK_ESCAPE, VK_CONVERT, VK_NONCONVERT,
    VK_ACCEPT, VK_MODECHANGE, VK_SPACE, VK_PRIOR, VK_NEXT, VK_END, VK_HOME,
    VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN, VK_SELECT, VK_PRINT, VK_EXECUTE,
    VK_SNAPSHOT, VK_INSERT, VK_DELETE, VK_HELP, VK_LWIN, VK_RWIN, VK_APPS,
    VK_SLEEP, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4,
    VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_MULTIPLY,
    VK_ADD, VK_SEPARATOR, VK_SUBTRACT, VK_DECIMAL, VK_DIVIDE, VK_F1, VK_F2,
    VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
    VK_F13, VK_F14, VK_F15, VK_F16, VK_F17, VK_F18, VK_F19, VK_F20, VK_F21,
    VK_F22, VK_F23, VK_F24, VK_NUMLOCK, VK_SCROLL, VK_LSHIFT, VK_RSHIFT,
    VK_LCONTROL, VK_RCONTROL, VK_LMENU, VK_RMENU, VK_BROWSER_BACK,
    VK_BROWSER_FORWARD, VK_BROWSER_REFRESH, VK_BROWSER_STOP,
    VK_BROWSER_SEARCH, VK_BROWSER_FAVORITES, VK_BROWSER_HOME, VK_VOLUME_MUTE,
    VK_VOLUME_DOWN, VK_VOLUME_UP, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PREV_TRACK,
    VK_MEDIA_STOP, VK_MEDIA_PLAY_PAUSE, VK_LAUNCH_MAIL,
    VK_LAUNCH_MEDIA_SELECT, VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_OEM_1,
    VK_OEM_PLUS, VK_OEM_COMMA, VK_OEM_MINUS, VK_OEM_PERIOD, VK_OEM_2,
    VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_102,
    VK_PROCESSKEY, VK_PACKET, VK_ATTN, VK_CRSEL, VK_EXSEL, VK_EREOF, VK_PLAY,
    VK_ZOOM, VK_NONAME, VK_PA1, VK_OEM_CLEAR,
};

// Maps virtual key codes to human-readable strings
fn vk_to_string(vk_code: VIRTUAL_KEY) -> String {
    match vk_code {
        // Alphanumeric keys
        key if key.0 >= 0x30 && key.0 <= 0x39 => { // 0-9
            format!("{}", char::from_u32(key.0 as u32).unwrap_or('?'))
        }
        key if key.0 >= 0x41 && key.0 <= 0x5A => { // A-Z
            format!("{}", char::from_u32(key.0 as u32).unwrap_or('?'))
        }

        // Specific keys
        VK_LBUTTON => "Left Mouse".to_string(),
        VK_RBUTTON => "Right Mouse".to_string(),
        VK_CANCEL => "Cancel".to_string(),
        VK_MBUTTON => "Middle Mouse".to_string(),
        VK_XBUTTON1 => "X1 Mouse".to_string(),
        VK_XBUTTON2 => "X2 Mouse".to_string(),
        VK_BACK => "Backspace".to_string(),
        VK_TAB => "Tab".to_string(),
        VK_CLEAR => "Clear".to_string(),
        VK_RETURN => "Enter".to_string(),
        VK_SHIFT => "Shift (Any)".to_string(),
        VK_CONTROL => "Ctrl (Any)".to_string(),
        VK_MENU => "Alt (Any)".to_string(),
        VK_PAUSE => "Pause".to_string(),
        VK_CAPITAL => "Caps Lock".to_string(),
        #[allow(unreachable_patterns)]
        VK_KANA | VK_HANGUL | VK_JUNJA | VK_FINAL | VK_HANJA | VK_KANJI => {
            format!("IME specific: 0x{:X}", vk_code.0)
        }
        VK_ESCAPE => "Escape".to_string(),
        VK_CONVERT | VK_NONCONVERT | VK_ACCEPT | VK_MODECHANGE => {
            format!("IME specific: 0x{:X}", vk_code.0)
        }
        VK_SPACE => "Space".to_string(),
        VK_PRIOR => "Page Up".to_string(),
        VK_NEXT => "Page Down".to_string(),
        VK_END => "End".to_string(),
        VK_HOME => "Home".to_string(),
        VK_LEFT => "Left Arrow".to_string(),
        VK_UP => "Up Arrow".to_string(),
        VK_RIGHT => "Right Arrow".to_string(),
        VK_DOWN => "Down Arrow".to_string(),
        VK_SELECT => "Select".to_string(),
        VK_PRINT => "Print".to_string(),
        VK_EXECUTE => "Execute".to_string(),
        VK_SNAPSHOT => "Print Screen".to_string(),
        VK_INSERT => "Insert".to_string(),
        VK_DELETE => "Delete".to_string(),
        VK_HELP => "Help".to_string(),
        VK_LWIN => "Left Windows".to_string(),
        VK_RWIN => "Right Windows".to_string(),
        VK_APPS => "Apps".to_string(),
        VK_SLEEP => "Sleep".to_string(),
        VK_NUMPAD0 => "Numpad 0".to_string(),
        VK_NUMPAD1 => "Numpad 1".to_string(),
        VK_NUMPAD2 => "Numpad 2".to_string(),
        VK_NUMPAD3 => "Numpad 3".to_string(),
        VK_NUMPAD4 => "Numpad 4".to_string(),
        VK_NUMPAD5 => "Numpad 5".to_string(),
        VK_NUMPAD6 => "Numpad 6".to_string(),
        VK_NUMPAD7 => "Numpad 7".to_string(),
        VK_NUMPAD8 => "Numpad 8".to_string(),
        VK_NUMPAD9 => "Numpad 9".to_string(),
        VK_MULTIPLY => "Numpad *".to_string(),
        VK_ADD => "Numpad +".to_string(),
        VK_SEPARATOR => "Numpad Separator".to_string(),
        VK_SUBTRACT => "Numpad -".to_string(),
        VK_DECIMAL => "Numpad .".to_string(),
        VK_DIVIDE => "Numpad /".to_string(),
        VK_F1 => "F1".to_string(), VK_F2 => "F2".to_string(), VK_F3 => "F3".to_string(),
        VK_F4 => "F4".to_string(), VK_F5 => "F5".to_string(), VK_F6 => "F6".to_string(),
        VK_F7 => "F7".to_string(), VK_F8 => "F8".to_string(), VK_F9 => "F9".to_string(),
        VK_F10 => "F10".to_string(), VK_F11 => "F11".to_string(), VK_F12 => "F12".to_string(),
        VK_F13 => "F13".to_string(), VK_F14 => "F14".to_string(), VK_F15 => "F15".to_string(),
        VK_F16 => "F16".to_string(), VK_F17 => "F17".to_string(), VK_F18 => "F18".to_string(),
        VK_F19 => "F19".to_string(), VK_F20 => "F20".to_string(), VK_F21 => "F21".to_string(),
        VK_F22 => "F22".to_string(), VK_F23 => "F23".to_string(), VK_F24 => "F24".to_string(),
        VK_NUMLOCK => "Num Lock".to_string(),
        VK_SCROLL => "Scroll Lock".to_string(),
        VK_LSHIFT => "Left Shift".to_string(),
        VK_RSHIFT => "Right Shift".to_string(),
        VK_LCONTROL => "Left Ctrl".to_string(),
        VK_RCONTROL => "Right Ctrl".to_string(),
        VK_LMENU => "Left Alt".to_string(),
        VK_RMENU => "Right Alt".to_string(),
        VK_BROWSER_BACK => "Browser Back".to_string(),
        VK_BROWSER_FORWARD => "Browser Forward".to_string(),
        VK_BROWSER_REFRESH => "Browser Refresh".to_string(),
        VK_BROWSER_STOP => "Browser Stop".to_string(),
        VK_BROWSER_SEARCH => "Browser Search".to_string(),
        VK_BROWSER_FAVORITES => "Browser Favorites".to_string(),
        VK_BROWSER_HOME => "Browser Home".to_string(),
        VK_VOLUME_MUTE => "Volume Mute".to_string(),
        VK_VOLUME_DOWN => "Volume Down".to_string(),
        VK_VOLUME_UP => "Volume Up".to_string(),
        VK_MEDIA_NEXT_TRACK => "Media Next".to_string(),
        VK_MEDIA_PREV_TRACK => "Media Prev".to_string(),
        VK_MEDIA_STOP => "Media Stop".to_string(),
        VK_MEDIA_PLAY_PAUSE => "Media Play/Pause".to_string(),
        VK_LAUNCH_MAIL => "Launch Mail".to_string(),
        VK_LAUNCH_MEDIA_SELECT => "Launch Media Select".to_string(),
        VK_LAUNCH_APP1 => "Launch App1".to_string(),
        VK_LAUNCH_APP2 => "Launch App2".to_string(),
        VK_OEM_1 => ";:".to_string(),      // For US standard keyboard
        VK_OEM_PLUS => "=".to_string(),
        VK_OEM_COMMA => ",".to_string(),
        VK_OEM_MINUS => "-".to_string(),
        VK_OEM_PERIOD => ".".to_string(),
        VK_OEM_2 => "/?".to_string(),      // For US standard keyboard
        VK_OEM_3 => "`~".to_string(),      // For US standard keyboard
        VK_OEM_4 => "[{".to_string(),      // For US standard keyboard
        VK_OEM_5 => "\\|".to_string(),     // For US standard keyboard
        VK_OEM_6 => "]}".to_string(),      // For US standard keyboard
        VK_OEM_7 => "'\"".to_string(),     // For US standard keyboard
        VK_OEM_8 => "OEM_8".to_string(),
        VK_OEM_102 => "OEM_102 (Angle Bracket/Backslash)".to_string(),
        VK_PROCESSKEY => "Process Key".to_string(),
        VK_PACKET => "Packet".to_string(),
        VK_ATTN => "Attn".to_string(),
        VK_CRSEL => "CrSel".to_string(),
        VK_EXSEL => "ExSel".to_string(),
        VK_EREOF => "Erase EOF".to_string(),
        VK_PLAY => "Play".to_string(),
        VK_ZOOM => "Zoom".to_string(),
        VK_NONAME => "NoName".to_string(),
        VK_PA1 => "PA1".to_string(),
        VK_OEM_CLEAR => "OEM Clear".to_string(),
        _ => format!("Unknown(0x{:02X})", vk_code.0),
    }
}

fn main() {
    println!("Monitoring pressed keys. Press Ctrl+C to exit.");
    println!("----------------------------------------------");

    loop {
        let mut pressed_keys = Vec::new();

        // Iterate through virtual key codes from 1 to 254
        // This range is specified by Microsoft for GetAsyncKeyState.
        for i in 1..=254 {
            // GetAsyncKeyState takes an i32 for the virtual key code.
            // The high bit of the return value (a SHORT/i16) is set if the key is down.
            let state = unsafe { GetAsyncKeyState(i) };
            if state & (0x8000u16 as i16) != 0 {
                // The key is currently pressed
                pressed_keys.push(vk_to_string(VIRTUAL_KEY(i as u16)));
            }
        }

        let output_string = if !pressed_keys.is_empty() {
            format!("Pressed: {}", pressed_keys.join(", "))
        } else {
            "No keys pressed.".to_string()
        };

        // Print to the same line, clearing previous output.
        // The 120 spaces should be enough to clear most previous lines.
        print!("\r{: <120}", ""); // Clear the line
        print!("\r{}", output_string); // Print new content
        
        // Ensure the output is immediately visible
        if let Err(e) = stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
            // Optionally, decide if the program should exit or continue
        }


        // Adjust sleep duration for responsiveness vs CPU usage
        thread::sleep(Duration::from_millis(50)); // ~20 updates per second
    }
}