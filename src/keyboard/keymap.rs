use super::keycodes::KeyCode;

pub const NUM_COLS: u8 = 15;
pub const NUM_ROWS: u8 = 5;

pub const N_LAYER_MAP: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Tilde, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0, Minus, Equals, Empty, Backspace],
    [Tab, Empty, Q, W, E, R, T, Y, U, I, O, P, LeftBracket, RightBracket, BackSlash],
    [CapsLock, Empty, A, S, D, F, G, H, J, K, L, Semicolon, SingleQuote, Enter, Empty],
    [Empty, LeftShift, Z, X, C, V, B, N, M, Comma, Period, ForwardSlash, Empty, RightShift, Empty],
    [LeftCtrl, LeftSuper, Empty, LeftAlt, Empty, Empty, Space, Empty, Empty, Empty, RightAlt, Fn, Empty, Menu, RightCtrl],
];


// TODO: Add more FN Buttons as I add functionality on user end. This may be RGB controls,
// direct shortcuts on computer, idk, but stuff will be added.
pub const FN_LAYER_MAP: [[KeyCode; NUM_COLS]; NUM_ROWS] = [
    [Escape, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, Empty, Delete],
    [Tab, Empty, Q, Up, E, PrevTrack, PlayPause, NextTrack, U, I, O, P, LeftBracket, RightBracket, BackSlash],
    [CapsLock, Empty, Left, Down, Right, F, G, H, VolDec, VolInc, L, Semicolon, SingleQuote, Enter, Empty],
    [Empty, LeftShift, PowerOff, X, C, V, B, N, Mute, Comma, Period, ForwardSlash, Empty, RightShift, Empty],
    [LeftCtrl, LeftSuper, Empty, LeftAlt, Empty, Empty, Space, Empty, Empty, Empty, RightAlt, Fn, Empty, Menu, RightCtrl],
];
