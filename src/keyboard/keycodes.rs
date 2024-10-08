use defmt::Format;
use serde::{Serialize, Deserialize};

// https://www.usb.org/sites/default/files/hut1_5.pdf
/**
 * This will contain ALL values, for all 3 HID streams. These 3 streams will be for:
 * 1. Normal Layer. All options that are accessible without using FN key.
 * 2. FN Consumer Control Layer. This will contain the 'special' buttons, such as VolUp, Mute, etc.
 * 3. FN + Normal Layer. These will be the keys that are standard for keyboards such as F1 - F12,
 *    Del, Esc, etc.
 *
 */
#[repr(u16)]
#[derive(Clone, Copy, Format, PartialEq)]
pub enum KeyCode {
    /* Normal Layer Key Codes */
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
    Enter =     0x28,
    Backspace = 0x2a,
    Tab =       0x2b,
    Space =     0x2c,
    Hyphen =    0x2d,
    Equals =    0x2e,
    LeftBracket =   0x2f,
    RightBracket =  0x30,
    BackSlash =     0x31,
    Semicolon =     0x33,
    Apostrophe =    0x34,
    Tilde =         0x35,
    Comma =         0x36,
    Period =        0x37,
    ForwardSlash =  0x38,
    CapsLock =  0x39,
    Asterisk =  0x55,
    LeftCtrl =  0xe0,
    LeftShift = 0xe1,
    LeftAlt =   0xe2,
    LeftSuper = 0xe3,
    RightCtrl = 0xe4,
    RightShift =    0xe5,
    RightAlt =      0xe6,
    RightSuper =    0xe7,

    /* FN Layer Key Codes */
    // Normal Keys
    Escape = 0x29,
    F1 = 0x3a,
    F2 = 0x3b,
    F3 = 0x3c,
    F4 = 0x3d,
    F5 = 0x3e,
    F6 = 0x3f,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4a,
    PageUp = 0x4b,
    DeleteForward = 0x4c, // The Del used in KiCAD
    End = 0x4d,
    PageDown = 0x4e,
    Right = 0x4f,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
    Menu = 0x76,
    
    // System Control
    PowerOff = 0xef,

    // Consumer Page
    VolInc = 0xf8,
    VolDec = 0xf9,
    Mute = 0xfa,
    NextTrack = 0xfb,
    PreviousTrack = 0xfc,
    PlayPause = 0xfd,
    GameRecord = 0xfe,
    GameCapture = 0xff,
    // TODO: Implement feature reports if using RGB keyboard
    
    /* KeebBrightnessInc = ,
    KeebBrightnessDec = ,
    KeebBrightnessPowerToggle = ,
    */
    
}
/**
 * #[derive(Debug, Format, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
 * #[repr(u16)]
 * pub enum MacroKey {
 *     Macro1 = 0x01,
 *     Macro2 = 0x01,
 *     Macro3 = 0x01,
 *     Macro4 = 0x01,
 *     Macro5 = 0x01,
 *     Macro6 = 0x01,
 *     Macro7 = 0x01,
 *     Macro8 = 0x01,
 *     Macro9 = 0x01,
 *     Macro10 = 0x01,
 *     Macro11 = 0x01,
 *     Macro12 = 0x01,
 *     Macro13 = 0x01,
 *     Macro14 = 0x01,
 *     Macro15 = 0x01,
 *     Macro16 = 0x01,
 *     Macro17 = 0x01,
 *     Macro18 = 0x01,
 *     Macro19 = 0x01,
 *     Macro20 = 0x01,
 *     Macro21 = 0x01,
 *     Macro22 = 0x01,
 *     Macro23 = 0x01,
 *     Macro24 = 0x01,
 *     Macro25 = 0x01,
 *     Macro26 = 0x01,
 *     Macro27 = 0x01,
 *     Macro28 = 0x01,
 *     Macro29 = 0x01,
 *     Macro30 = 0x01,
 *     Macro31 = 0x01,
 *     Macro32 = 0x01,
 *     Macro33 = 0x01,
 *     Macro34 = 0x01,
 *     Macro35 = 0x01,
 *     Macro36 = 0x01,
 *     Macro37 = 0x01,
 *     Macro38 = 0x01,
 *     Macro39 = 0x01,
 *     Macro40 = 0x01,
 *     Macro41 = 0x01,
 *     Macro42 = 0x01,
 *     Macro43 = 0x01,
 *     Macro44 = 0x01,
 *     Macro45 = 0x01,
 *     Macro46 = 0x01,
 *     Macro47 = 0x01,
 *     Macro48 = 0x01,
 *     Macro49 = 0x01,
 *     Macro50 = 0x01,
 *     Macro51 = 0x01,
 *     Macro52 = 0x01,
 *     Macro53 = 0x01,
 *     Macro54 = 0x01,
 *     Macro55 = 0x01,
 *     Macro56 = 0x01,
 *     Macro57 = 0x01,
 *     Macro58 = 0x01,
 *     Macro59 = 0x01,
 *     Macro60 = 0x01,
 *     Macro61 = 0x01,
 *     Macro62 = 0x01,
 *     Macro63 = 0x01,
 *     Macro64 = 0x01,
 * }
 */

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(u16)]
pub enum SystemControlKey {
    #[num_enum(default)]
    Zero = 0x00,
    PowerOff = 0x81,
    Sleep = 0x82,
    WakeUp = 0x83,
    Restart = 0x8f,
}

#[derive(Debug, Format, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(u16)]
pub enum ConsumerKey {
    #[num_enum(default)]
    Zero = 0x00,
    // 15.5 Display Controls
    SnapShot = 0x65,
    /// <https://www.usb.org/sites/default/files/hutrr41_0.pdf>
    BrightnessUp = 0x6F,
    BrightnessDown = 0x70,
    // 15.7 Transport Controls
    Play = 0xB0,
    Pause = 0xB1,
    Record = 0xB2,
    FastForward = 0xB3,
    Rewind = 0xB4,
    NextTrack = 0xB5,
    PrevTrack = 0xB6,
    StopPlay = 0xB7,
    Eject = 0xB8,
    RandomPlay = 0xB9,
    Repeat = 0xBC,
    StopEject = 0xCC,
    PlayPause = 0xCD,
    // 15.9.1 Audio Controls - Volume
    Mute = 0xE2,
    VolumeIncrement = 0xE9,
    VolumeDecrement = 0xEA,
    Reserved = 0xEB,
    // 15.15 Application Launch Buttons
    Email = 0x18A,
    Calculator = 0x192,
    LocalBrowser = 0x194,
    Lock = 0x19E,
    ControlPanel = 0x19F,
    Assistant = 0x1CB,
    // 15.16 Generic GUI Application Controls
    New = 0x201,
    Open = 0x202,
    Close = 0x203,
    Exit = 0x204,
    Maximize = 0x205,
    Minimize = 0x206,
    Save = 0x207,
    Print = 0x208,
    Properties = 0x209,
    Undo = 0x21A,
    Copy = 0x21B,
    Cut = 0x21C,
    Paste = 0x21D,
    SelectAll = 0x21E,
    Find = 0x21F,
    Search = 0x221,
    Home = 0x223,
    Back = 0x224,
    Forward = 0x225,
    Stop = 0x226,
    Refresh = 0x227,
    Bookmarks = 0x22A,
    NextKeyboardLayoutSelect = 0x29D,
    DesktopShowAllWindows = 0x29F,
    AcSoftKeyLeft = 0x2A0,
}

impl KeyCode {
    pub(crate) fn modifier_bmask(&self) -> Option<u8> {
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
    pub(crate) fn is_mod(&self) -> bool {
        self == KeyCode::Fn || self.modifier_bmask().is_some()
    }
    pub(crate) fn is_consumer(&self) -> bool {
        self >= 0xf0
    }
    /* TODO:
     * Implement the rest of the consumer key codes
     */
    pub(crate) fn to_consumer(&self) -> ConsumerKey {
        match self {
            KeyCode::Mute => ConsumerKey::Mute,
            KeyCode::VolInc => ConsumerKey::VolumeIncrement,
            KeyCode::VolDec => ConsumerKey::VolumeDecrement,
            KeyCode::NextTrack => ConsumerKey::NextTrack,
            KeyCode::PreviousTrack => ConsumerKey::PrevTrack,
            KeyCode::PlayPause => ConsumerKey::PlayPause,
            KeyCode::GameRecord => ConsumerKey::Record,
            KeyCode::GameCapture => ConsumerKey::SnapShot,
            _ => None,
        }
    }
    pub(crate) fn is_system(&self) -> bool {
        KeyCode::PowerOff <= self <= KeyCode::PowerOff
    }
    pub(crate) fn to_system(&self) -> Option<SystemControlKey> {
        match self {
            KeyCode::PowerOff => Some(SystemControlKey::PowerOff),
            _ => None,
        }
    }
    /**
     * Requires no matching to_basic because by default it is already 'basic', or the normal
     * keycodes sent by a keyboard.
     */
    pub(crate) fn is_basic(&self) -> bool {
        // Self-imposed border.
        self <= 0xdd
    }

    /**
     * Implement later if I have time. Will also likely need support on driver side, so will have
     * to write for both windows and linux in case I want to actually be efficient. But I barely
     * even use windows anyway so who gives a fuck
     *
     * pub(crate) fn is_macro(&self) -> bool {
     *
     * }
     *
     * pub(crate) fn as_macro(&self) -> MacroKey {
     *
     * }
     */

}


