use defmt::Format;

#[repr(u8)]
#[derive(Clone, Copy, Format, PartialEq)]
pub enum KeyCode {
    Empty = 0x0,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0a,
    H = 0x0b,
    I = 0x0c,
    J = 0x0d,
    K = 0x0e,
    L = 0x0f,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1a,
    X = 0x1b,
    Y = 0x1c,
    Z = 0x1d,

    Num1 = 0x1e,
    Num2 = 0x1f,
    Num3 = 0x20,
    Num4 = 0x21,
    Num5 = 0x22,
    Num6 = 0x23,
    Num7 = 0x24,
    Num8 = 0x25,
    Num9 = 0x26,
    Num0 = 0x27,

    Enter = 0x28,
    Backspace = 0x2a,
    Tab = 0x2b,
    Space = 0x2c,
    Minus = 0x2d,
    Equals = 0x2e,
    LeftBracket = 0x2f,
    RightBracket = 0x30,
    BackSlash = 0x31,
    Semicolon = 0x32,
    SingleQuote = 0x33,
    Tilde = 0x34,
    Comma = 0x35,
    Period = 0x36,
    ForwardSlash = 0x37,
    CapsLock = 0x38,
    Menu = 0x39,

    // FN controls
    Up = 0x40,
    Left = 0x41,
    Down = 0x42,
    Right = 0x43,
    Mute = 0x44,
    VolUp = 0x45,
    VolDown = 0x46,
    F1 = 0x47,
    F2 = 0x48,
    F3 = 0x49,
    F4 = 0x4a,
    F5 = 0x4b,
    F6 = 0x4c,
    F7 = 0x4d,
    F8 = 0x4e,
    F9 = 0x4f,
    F10 = 0x50,
    F11 = 0x51,
    F12 = 0x52,
    Escape = 0x53,
    Delete = 0x54,

    

    Fn = 0xf0,
    LeftShift = 0xf1,
    LeftCtrl = 0xf2,
    LeftAlt = 0xf3,
    LeftSuper = 0xf4,
    RightAlt = 0xf6,
    RightCtrl = 0xf7,
    RightShift = 0xf8,
}

impl KeyCode {
    pub fn modifier_bmask(&self) -> Option<u8> {
        match *self {
            KeyCode::LeftCtrl => Some(1 << 0),
            KeyCode::LeftShift => Some(1 << 1),
            KeyCode::LeftAlt => Some(1 << 2),
            KeyCode::LeftSuper => Some(1 << 3),
            KeyCode::RightCtrl => Some(1 << 4),
            KeyCode::RightShift => Some(1 << 5),
            KeyCode::RightAlt => Some(1 << 6),
            _ => None,
        }
    }
    pub fn is_mod(&self) -> bool {
        *self == KeyCode::Fn || self.modifier_bmask().is_some()
    }
}
