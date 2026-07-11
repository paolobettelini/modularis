use bevy::prelude::KeyCode;

pub fn parse_key_code(value: &str) -> Option<KeyCode> {
    let normalized = value.trim().to_ascii_lowercase();
    Some(match normalized.as_str() {
        "space" => KeyCode::Space,
        "escape" | "esc" => KeyCode::Escape,
        "enter" | "return" => KeyCode::Enter,
        "backspace" => KeyCode::Backspace,
        "delete" => KeyCode::Delete,
        "tab" => KeyCode::Tab,
        "shift" | "shiftleft" | "left shift" => KeyCode::ShiftLeft,
        "shiftright" | "right shift" => KeyCode::ShiftRight,
        "control" | "ctrl" | "controlleft" | "left control" => KeyCode::ControlLeft,
        "controlright" | "right control" => KeyCode::ControlRight,
        "alt" | "altleft" | "left alt" => KeyCode::AltLeft,
        "altright" | "right alt" => KeyCode::AltRight,
        "arrowup" | "up" => KeyCode::ArrowUp,
        "arrowdown" | "down" => KeyCode::ArrowDown,
        "arrowleft" | "left" => KeyCode::ArrowLeft,
        "arrowright" | "right" => KeyCode::ArrowRight,
        "0" | "digit0" => KeyCode::Digit0,
        "1" | "digit1" => KeyCode::Digit1,
        "2" | "digit2" => KeyCode::Digit2,
        "3" | "digit3" => KeyCode::Digit3,
        "4" | "digit4" => KeyCode::Digit4,
        "5" | "digit5" => KeyCode::Digit5,
        "6" | "digit6" => KeyCode::Digit6,
        "7" | "digit7" => KeyCode::Digit7,
        "8" | "digit8" => KeyCode::Digit8,
        "9" | "digit9" => KeyCode::Digit9,
        "f1" => KeyCode::F1,
        "f2" => KeyCode::F2,
        "f3" => KeyCode::F3,
        "f4" => KeyCode::F4,
        "f5" => KeyCode::F5,
        "f6" => KeyCode::F6,
        "f7" => KeyCode::F7,
        "f8" => KeyCode::F8,
        "f9" => KeyCode::F9,
        "f10" => KeyCode::F10,
        "f11" => KeyCode::F11,
        "f12" => KeyCode::F12,
        "a" | "keya" => KeyCode::KeyA,
        "b" | "keyb" => KeyCode::KeyB,
        "c" | "keyc" => KeyCode::KeyC,
        "d" | "keyd" => KeyCode::KeyD,
        "e" | "keye" => KeyCode::KeyE,
        "f" | "keyf" => KeyCode::KeyF,
        "g" | "keyg" => KeyCode::KeyG,
        "h" | "keyh" => KeyCode::KeyH,
        "i" | "keyi" => KeyCode::KeyI,
        "j" | "keyj" => KeyCode::KeyJ,
        "k" | "keyk" => KeyCode::KeyK,
        "l" | "keyl" => KeyCode::KeyL,
        "m" | "keym" => KeyCode::KeyM,
        "n" | "keyn" => KeyCode::KeyN,
        "o" | "keyo" => KeyCode::KeyO,
        "p" | "keyp" => KeyCode::KeyP,
        "q" | "keyq" => KeyCode::KeyQ,
        "r" | "keyr" => KeyCode::KeyR,
        "s" | "keys" => KeyCode::KeyS,
        "t" | "keyt" => KeyCode::KeyT,
        "u" | "keyu" => KeyCode::KeyU,
        "v" | "keyv" => KeyCode::KeyV,
        "w" | "keyw" => KeyCode::KeyW,
        "x" | "keyx" => KeyCode::KeyX,
        "y" | "keyy" => KeyCode::KeyY,
        "z" | "keyz" => KeyCode::KeyZ,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_jump_bindings_case_insensitively() {
        assert_eq!(parse_key_code("Space"), Some(KeyCode::Space));
        assert_eq!(parse_key_code("ControlLeft"), Some(KeyCode::ControlLeft));
        assert_eq!(parse_key_code("enter"), Some(KeyCode::Enter));
        assert_eq!(parse_key_code("K"), Some(KeyCode::KeyK));
        assert_eq!(parse_key_code("unknown"), None);
    }
}
