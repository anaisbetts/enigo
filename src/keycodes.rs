#[cfg(any(all(unix, not(target_os = "macos")), target_os = "windows"))]
use log::trace;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
use strum_macros::EnumIter;

// A key on the keyboard.
/// Use [`Key::Unicode`] to enter arbitrary Unicode chars.
/// If a key is missing, please open an issue in our repo and we will quickly
/// add it. In the mean time, you can simulate that key by using [`Key::Other`]
/// or the [`crate::Keyboard::raw`] function. Some of the keys are only
/// available on a specific platform. Use conditional compilation to use them.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(test, derive(EnumIter))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num0,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num3,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num4,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num5,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num6,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num7,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num8,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Num9,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    A,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    B,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    C,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    D,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    E,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    F,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    G,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    H,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    I,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    J,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    K,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    L,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    M,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    N,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    O,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    P,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Q,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    R,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    S,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    T,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    U,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    V,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    W,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    X,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Y,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Z,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    AbntC1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    AbntC2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Accept,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Add,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Apps,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Attn,
    /// backspace key
    Backspace,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    Break,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    Begin,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    BrightnessDown,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    BrightnessUp,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserBack,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserFavorites,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserForward,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserHome,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserRefresh,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserSearch,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    BrowserStop,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Cancel,
    /// caps lock key
    CapsLock,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Clear,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// command key on macOS (super key on Linux, windows key on Windows)
    #[cfg_attr(feature = "serde", serde(alias = "cmd"))]
    Command,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    ContrastUp,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    ContrastDown,
    /// control key
    #[cfg_attr(feature = "serde", serde(alias = "ctrl"))]
    Control,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Convert,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Crsel,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEAlphanumeric,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBECodeinput,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEDetermineString,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEEnterDLGConversionMode,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEEnterIMEConfigMode,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEEnterWordRegisterMode,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEFlushString,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEHiragana,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBEKatakana,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBENoCodepoint,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBENoRoman,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBERoman,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBESBCSChar,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    DBESChar,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Decimal,
    /// delete key
    Delete,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Divide,
    /// down arrow key
    DownArrow,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    Eject,
    /// end key
    End,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Ereof,
    /// escape key (esc)
    Escape,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Execute,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Exsel,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    /// F21 key
    F21,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    /// F22 key
    F22,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    /// F23 key
    F23,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    /// F24 key
    F24,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F25,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F26,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F27,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F28,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F29,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F30,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F31,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F32,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F33,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F34,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    F35,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    Function,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Final,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    Find,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadA,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadB,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadDPadDown,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadDPadLeft,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadDPadRight,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadDPadUp,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftShoulder,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftThumbstickButton,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftThumbstickDown,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftThumbstickLeft,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftThumbstickRight,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftThumbstickUp,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadLeftTrigger,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadMenu,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightShoulder,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightThumbstickButton,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightThumbstickDown,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightThumbstickLeft,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightThumbstickRight,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightThumbstickUp,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadRightTrigger,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadView,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadX,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    GamepadY,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Hangeul,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Hangul,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Hanja,
    Help,
    /// home key
    Home,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Ico00,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    IcoClear,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    IcoHelp,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    IlluminationDown,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    IlluminationUp,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    IlluminationToggle,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    IMEOff,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    IMEOn,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Insert,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Junja,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Kana,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Kanji,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    LaunchApp1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    LaunchApp2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    LaunchMail,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    LaunchMediaSelect,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    /// Opens launchpad
    Launchpad,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    LaunchPanel,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    LButton,
    LControl,
    /// left arrow key
    LeftArrow,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    Linefeed,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    LMenu,
    LShift,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    LWin,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    MButton,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    MediaFast,
    MediaNextTrack,
    MediaPlayPause,
    MediaPrevTrack,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    MediaRewind,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    MediaStop,
    /// meta key (also known as "windows", "super", and "command")
    Meta,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    /// Opens mission control
    MissionControl,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    ModeChange,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Multiply,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationAccept,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationCancel,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationDown,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationLeft,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationMenu,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationRight,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationUp,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NavigationView,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NoName,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    NonConvert,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    None,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Numlock,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad0,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad3,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad4,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad5,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad6,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad7,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad8,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Numpad9,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM102,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM3,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM4,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM5,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM6,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM7,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEM8,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMAttn,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMAuto,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMAx,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMBacktab,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMClear,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMComma,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMCopy,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMCusel,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMEnlw,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMFinish,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMFJJisho,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMFJLoya,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMFJMasshou,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMFJRoya,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMFJTouroku,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMJump,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMMinus,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMNECEqual,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMPA1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMPA2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMPA3,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMPeriod,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMPlus,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMReset,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    OEMWsctrl,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    PA1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Packet,
    /// page down key
    PageDown,
    /// page up key
    PageUp,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Pause,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Play,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    Power,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Print,
    /// Take a screenshot
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    #[doc(alias = "Print")]
    #[doc(alias = "Snapshot")]
    PrintScr,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Processkey,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    RButton,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    RCommand,
    RControl,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    Redo,
    /// return key
    Return,
    /// right arrow key
    RightArrow,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    RMenu,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    ROption,
    RShift,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    RWin,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Scroll,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    ScrollLock,
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos")), feature = "bindable"))]
    Select,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    ScriptSwitch,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Separator,
    /// shift key
    Shift,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    /// Lock shift key
    ShiftLock,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Sleep,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    #[deprecated(since = "0.2.2", note = "now renamed to PrintScr")]
    Snapshot,
    /// space key
    Space,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Subtract,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// super key on linux (command key on macOS, windows key on Windows)
    Super,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    SysReq,
    /// tab key (tabulator)
    Tab,
    #[cfg(any(all(unix, not(target_os = "macos")), feature = "bindable"))]
    Undo,
    /// up arrow key
    UpArrow,
    #[cfg(any(target_os = "macos", feature = "bindable"))]
    VidMirror,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    #[cfg(all(unix, not(target_os = "macos"), feature = "bindable"))]
    /// microphone mute toggle on linux
    MicMute,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// windows key on Windows (super key on Linux, command key on macOS)
    Windows,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    XButton1,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    XButton2,
    #[cfg(any(target_os = "windows", feature = "bindable"))]
    Zoom,
    /// Unicode character
    #[doc(alias = "Layout")]
    #[cfg_attr(feature = "serde", serde(alias = "uni"))]
    #[cfg_attr(feature = "serde", serde(alias = "Uni"))]
    #[cfg_attr(feature = "serde", serde(alias = "Char"))]
    #[cfg_attr(feature = "serde", serde(alias = "char"))]
    Unicode(char),
    /// Use this for keys that are not listed here that you know the
    /// value of. Let us know if you think the key should be listed so
    /// we can add it
    /// On Linux, this will result in a keysym,
    /// On Windows, this will result in a `Virtual_Key` and
    /// On macOS, this will yield a `KeyCode`
    Other(u32),
}

#[cfg(all(unix, not(target_os = "macos")))]
/// Converts a Key to a Keysym
impl From<Key> for xkeysym::Keysym {
    #[allow(clippy::too_many_lines)]
    fn from(key: Key) -> Self {
        use xkeysym::Keysym;

        trace!("Key::from(key: {key:?})");

        #[allow(clippy::match_same_arms)]
        match key {
            Key::Unicode(c) => xkeysym::Keysym::from_char(c),
            Key::Alt | Key::Option => Keysym::Alt_L,
            Key::Backspace => Keysym::BackSpace,
            Key::Begin => Keysym::Begin,
            Key::Break => Keysym::Break,
            Key::Cancel => Keysym::Cancel,
            Key::CapsLock => Keysym::Caps_Lock,
            Key::Clear => Keysym::Clear,
            Key::Control | Key::LControl => Keysym::Control_L,
            Key::Delete => Keysym::Delete,
            Key::DownArrow => Keysym::Down,
            Key::End => Keysym::End,
            Key::Escape => Keysym::Escape,
            Key::Execute => Keysym::Execute,
            Key::F1 => Keysym::F1,
            Key::F2 => Keysym::F2,
            Key::F3 => Keysym::F3,
            Key::F4 => Keysym::F4,
            Key::F5 => Keysym::F5,
            Key::F6 => Keysym::F6,
            Key::F7 => Keysym::F7,
            Key::F8 => Keysym::F8,
            Key::F9 => Keysym::F9,
            Key::F10 => Keysym::F10,
            Key::F11 => Keysym::F11,
            Key::F12 => Keysym::F12,
            Key::F13 => Keysym::F13,
            Key::F14 => Keysym::F14,
            Key::F15 => Keysym::F15,
            Key::F16 => Keysym::F16,
            Key::F17 => Keysym::F17,
            Key::F18 => Keysym::F18,
            Key::F19 => Keysym::F19,
            Key::F20 => Keysym::F20,
            Key::F21 => Keysym::F21,
            Key::F22 => Keysym::F22,
            Key::F23 => Keysym::F23,
            Key::F24 => Keysym::F24,
            Key::F25 => Keysym::F25,
            Key::F26 => Keysym::F26,
            Key::F27 => Keysym::F27,
            Key::F28 => Keysym::F28,
            Key::F29 => Keysym::F29,
            Key::F30 => Keysym::F30,
            Key::F31 => Keysym::F31,
            Key::F32 => Keysym::F32,
            Key::F33 => Keysym::F33,
            Key::F34 => Keysym::F34,
            Key::F35 => Keysym::F35,
            Key::Find => Keysym::Find,
            Key::Hangul => Keysym::Hangul,
            Key::Hanja => Keysym::Hangul_Hanja,
            Key::Help => Keysym::Help,
            Key::Home => Keysym::Home,
            Key::Insert => Keysym::Insert,
            Key::Kanji => Keysym::Kanji,
            Key::LeftArrow => Keysym::Left,
            Key::Linefeed => Keysym::Linefeed,
            Key::LMenu => Keysym::Menu,
            Key::ModeChange => Keysym::Mode_switch,
            Key::MediaNextTrack => Keysym::XF86_AudioNext,
            Key::MediaPlayPause => Keysym::XF86_AudioPlay,
            Key::MediaPrevTrack => Keysym::XF86_AudioPrev,
            Key::MediaStop => Keysym::XF86_AudioStop,
            Key::Numlock => Keysym::Num_Lock,
            Key::PageDown => Keysym::Page_Down,
            Key::PageUp => Keysym::Page_Up,
            Key::Pause => Keysym::Pause,
            Key::Print => Keysym::Print,
            Key::PrintScr => Keysym::Print,
            Key::RControl => Keysym::Control_R,
            Key::Redo => Keysym::Redo,
            Key::Return => Keysym::Return,
            Key::RightArrow => Keysym::Right,
            Key::RShift => Keysym::Shift_R,
            Key::ScrollLock => Keysym::Scroll_Lock,
            Key::Select => Keysym::Select,
            Key::ScriptSwitch => Keysym::script_switch,
            Key::Shift | Key::LShift => Keysym::Shift_L,
            Key::ShiftLock => Keysym::Shift_Lock,
            Key::Space => Keysym::space,
            Key::SysReq => Keysym::Sys_Req,
            Key::Tab => Keysym::Tab,
            Key::Undo => Keysym::Undo,
            Key::UpArrow => Keysym::Up,
            Key::VolumeDown => Keysym::XF86_AudioLowerVolume,
            Key::VolumeUp => Keysym::XF86_AudioRaiseVolume,
            Key::VolumeMute => Keysym::XF86_AudioMute,
            Key::MicMute => Keysym::XF86_AudioMicMute,
            Key::Command | Key::Super | Key::Windows | Key::Meta => Keysym::Super_L,
            Key::Other(v) => Keysym::from(v),
        }
    }
}

/// Converts a Key to a Virtual Key
#[cfg(target_os = "windows")]
impl TryFrom<Key> for windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY {
    type Error = &'static str;

    #[allow(clippy::too_many_lines)]
    fn try_from(key: Key) -> Result<Self, Self::Error> {
        use windows::Win32::UI::Input::KeyboardAndMouse::{
            VK__none_, VIRTUAL_KEY, VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9,
            VK_A, VK_ABNT_C1, VK_ABNT_C2, VK_ACCEPT, VK_ADD, VK_APPS, VK_ATTN, VK_B, VK_BACK,
            VK_BROWSER_BACK, VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME,
            VK_BROWSER_REFRESH, VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_C, VK_CANCEL, VK_CAPITAL,
            VK_CLEAR, VK_CONTROL, VK_CONVERT, VK_CRSEL, VK_D, VK_DBE_ALPHANUMERIC,
            VK_DBE_CODEINPUT, VK_DBE_DBCSCHAR, VK_DBE_DETERMINESTRING,
            VK_DBE_ENTERDLGCONVERSIONMODE, VK_DBE_ENTERIMECONFIGMODE, VK_DBE_ENTERWORDREGISTERMODE,
            VK_DBE_FLUSHSTRING, VK_DBE_HIRAGANA, VK_DBE_KATAKANA, VK_DBE_NOCODEINPUT,
            VK_DBE_NOROMAN, VK_DBE_ROMAN, VK_DBE_SBCSCHAR, VK_DECIMAL, VK_DELETE, VK_DIVIDE,
            VK_DOWN, VK_E, VK_END, VK_EREOF, VK_ESCAPE, VK_EXECUTE, VK_EXSEL, VK_F, VK_F1, VK_F10,
            VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17, VK_F18, VK_F19, VK_F2, VK_F20,
            VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9,
            VK_FINAL, VK_G, VK_GAMEPAD_A, VK_GAMEPAD_B, VK_GAMEPAD_DPAD_DOWN, VK_GAMEPAD_DPAD_LEFT,
            VK_GAMEPAD_DPAD_RIGHT, VK_GAMEPAD_DPAD_UP, VK_GAMEPAD_LEFT_SHOULDER,
            VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON, VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
            VK_GAMEPAD_LEFT_THUMBSTICK_LEFT, VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
            VK_GAMEPAD_LEFT_THUMBSTICK_UP, VK_GAMEPAD_LEFT_TRIGGER, VK_GAMEPAD_MENU,
            VK_GAMEPAD_RIGHT_SHOULDER, VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
            VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN, VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
            VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT, VK_GAMEPAD_RIGHT_THUMBSTICK_UP,
            VK_GAMEPAD_RIGHT_TRIGGER, VK_GAMEPAD_VIEW, VK_GAMEPAD_X, VK_GAMEPAD_Y, VK_H,
            VK_HANGEUL, VK_HANGUL, VK_HANJA, VK_HELP, VK_HOME, VK_I, VK_ICO_00, VK_ICO_CLEAR,
            VK_ICO_HELP, VK_IME_OFF, VK_IME_ON, VK_INSERT, VK_J, VK_JUNJA, VK_K, VK_KANA, VK_KANJI,
            VK_L, VK_LAUNCH_APP1, VK_LAUNCH_APP2, VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT,
            VK_LBUTTON, VK_LCONTROL, VK_LEFT, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON,
            VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_MENU,
            VK_MODECHANGE, VK_MULTIPLY, VK_N, VK_NAVIGATION_ACCEPT, VK_NAVIGATION_CANCEL,
            VK_NAVIGATION_DOWN, VK_NAVIGATION_LEFT, VK_NAVIGATION_MENU, VK_NAVIGATION_RIGHT,
            VK_NAVIGATION_UP, VK_NAVIGATION_VIEW, VK_NEXT, VK_NONAME, VK_NONCONVERT, VK_NUMLOCK,
            VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3, VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6,
            VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1, VK_OEM_102, VK_OEM_2, VK_OEM_3,
            VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8, VK_OEM_ATTN, VK_OEM_AUTO, VK_OEM_AX,
            VK_OEM_BACKTAB, VK_OEM_CLEAR, VK_OEM_COMMA, VK_OEM_COPY, VK_OEM_CUSEL, VK_OEM_ENLW,
            VK_OEM_FINISH, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA, VK_OEM_FJ_MASSHOU, VK_OEM_FJ_ROYA,
            VK_OEM_FJ_TOUROKU, VK_OEM_JUMP, VK_OEM_MINUS, VK_OEM_NEC_EQUAL, VK_OEM_PA1, VK_OEM_PA2,
            VK_OEM_PA3, VK_OEM_PERIOD, VK_OEM_PLUS, VK_OEM_RESET, VK_OEM_WSCTRL, VK_P, VK_PA1,
            VK_PACKET, VK_PAUSE, VK_PLAY, VK_PRINT, VK_PRIOR, VK_PROCESSKEY, VK_Q, VK_R,
            VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_S,
            VK_SCROLL, VK_SELECT, VK_SEPARATOR, VK_SHIFT, VK_SLEEP, VK_SNAPSHOT, VK_SPACE,
            VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V, VK_VOLUME_DOWN, VK_VOLUME_MUTE,
            VK_VOLUME_UP, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2, VK_Y, VK_Z, VK_ZOOM,
        };

        trace!("Key::try_from(key: {key:?})");
        let vk = match key {
            Key::Num0 => VK_0,
            Key::Num1 => VK_1,
            Key::Num2 => VK_2,
            Key::Num3 => VK_3,
            Key::Num4 => VK_4,
            Key::Num5 => VK_5,
            Key::Num6 => VK_6,
            Key::Num7 => VK_7,
            Key::Num8 => VK_8,
            Key::Num9 => VK_9,
            Key::A => VK_A,
            Key::B => VK_B,
            Key::C => VK_C,
            Key::D => VK_D,
            Key::E => VK_E,
            Key::F => VK_F,
            Key::G => VK_G,
            Key::H => VK_H,
            Key::I => VK_I,
            Key::J => VK_J,
            Key::K => VK_K,
            Key::L => VK_L,
            Key::M => VK_M,
            Key::N => VK_N,
            Key::O => VK_O,
            Key::P => VK_P,
            Key::Q => VK_Q,
            Key::R => VK_R,
            Key::S => VK_S,
            Key::T => VK_T,
            Key::U => VK_U,
            Key::V => VK_V,
            Key::W => VK_W,
            Key::X => VK_X,
            Key::Y => VK_Y,
            Key::Z => VK_Z,
            Key::AbntC1 => VK_ABNT_C1,
            Key::AbntC2 => VK_ABNT_C2,
            Key::Accept => VK_ACCEPT,
            Key::Add => VK_ADD,
            Key::Alt | Key::Option => VK_MENU,
            Key::Apps => VK_APPS,
            Key::Attn => VK_ATTN,
            Key::Backspace => VK_BACK,
            Key::BrowserBack => VK_BROWSER_BACK,
            Key::BrowserFavorites => VK_BROWSER_FAVORITES,
            Key::BrowserForward => VK_BROWSER_FORWARD,
            Key::BrowserHome => VK_BROWSER_HOME,
            Key::BrowserRefresh => VK_BROWSER_REFRESH,
            Key::BrowserSearch => VK_BROWSER_SEARCH,
            Key::BrowserStop => VK_BROWSER_STOP,
            Key::Cancel => VK_CANCEL,
            Key::CapsLock => VK_CAPITAL,
            Key::Clear => VK_CLEAR,
            Key::Control => VK_CONTROL,
            Key::Convert => VK_CONVERT,
            Key::Crsel => VK_CRSEL,
            Key::DBEAlphanumeric => VK_DBE_ALPHANUMERIC,
            Key::DBECodeinput => VK_DBE_CODEINPUT,
            Key::DBEDetermineString => VK_DBE_DETERMINESTRING,
            Key::DBEEnterDLGConversionMode => VK_DBE_ENTERDLGCONVERSIONMODE,
            Key::DBEEnterIMEConfigMode => VK_DBE_ENTERIMECONFIGMODE,
            Key::DBEEnterWordRegisterMode => VK_DBE_ENTERWORDREGISTERMODE,
            Key::DBEFlushString => VK_DBE_FLUSHSTRING,
            Key::DBEHiragana => VK_DBE_HIRAGANA,
            Key::DBEKatakana => VK_DBE_KATAKANA,
            Key::DBENoCodepoint => VK_DBE_NOCODEINPUT,
            Key::DBENoRoman => VK_DBE_NOROMAN,
            Key::DBERoman => VK_DBE_ROMAN,
            Key::DBESBCSChar => VK_DBE_SBCSCHAR,
            Key::DBESChar => VK_DBE_DBCSCHAR,
            Key::Decimal => VK_DECIMAL,
            Key::Delete => VK_DELETE,
            Key::Divide => VK_DIVIDE,
            Key::DownArrow => VK_DOWN,
            Key::End => VK_END,
            Key::Ereof => VK_EREOF,
            Key::Escape => VK_ESCAPE,
            Key::Execute => VK_EXECUTE,
            Key::Exsel => VK_EXSEL,
            Key::F1 => VK_F1,
            Key::F2 => VK_F2,
            Key::F3 => VK_F3,
            Key::F4 => VK_F4,
            Key::F5 => VK_F5,
            Key::F6 => VK_F6,
            Key::F7 => VK_F7,
            Key::F8 => VK_F8,
            Key::F9 => VK_F9,
            Key::F10 => VK_F10,
            Key::F11 => VK_F11,
            Key::F12 => VK_F12,
            Key::F13 => VK_F13,
            Key::F14 => VK_F14,
            Key::F15 => VK_F15,
            Key::F16 => VK_F16,
            Key::F17 => VK_F17,
            Key::F18 => VK_F18,
            Key::F19 => VK_F19,
            Key::F20 => VK_F20,
            Key::F21 => VK_F21,
            Key::F22 => VK_F22,
            Key::F23 => VK_F23,
            Key::F24 => VK_F24,
            Key::Final => VK_FINAL,
            Key::GamepadA => VK_GAMEPAD_A,
            Key::GamepadB => VK_GAMEPAD_B,
            Key::GamepadDPadDown => VK_GAMEPAD_DPAD_DOWN,
            Key::GamepadDPadLeft => VK_GAMEPAD_DPAD_LEFT,
            Key::GamepadDPadRight => VK_GAMEPAD_DPAD_RIGHT,
            Key::GamepadDPadUp => VK_GAMEPAD_DPAD_UP,
            Key::GamepadLeftShoulder => VK_GAMEPAD_LEFT_SHOULDER,
            Key::GamepadLeftThumbstickButton => VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON,
            Key::GamepadLeftThumbstickDown => VK_GAMEPAD_LEFT_THUMBSTICK_DOWN,
            Key::GamepadLeftThumbstickLeft => VK_GAMEPAD_LEFT_THUMBSTICK_LEFT,
            Key::GamepadLeftThumbstickRight => VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT,
            Key::GamepadLeftThumbstickUp => VK_GAMEPAD_LEFT_THUMBSTICK_UP,
            Key::GamepadLeftTrigger => VK_GAMEPAD_LEFT_TRIGGER,
            Key::GamepadMenu => VK_GAMEPAD_MENU,
            Key::GamepadRightShoulder => VK_GAMEPAD_RIGHT_SHOULDER,
            Key::GamepadRightThumbstickButton => VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON,
            Key::GamepadRightThumbstickDown => VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN,
            Key::GamepadRightThumbstickLeft => VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT,
            Key::GamepadRightThumbstickRight => VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT,
            Key::GamepadRightThumbstickUp => VK_GAMEPAD_RIGHT_THUMBSTICK_UP,
            Key::GamepadRightTrigger => VK_GAMEPAD_RIGHT_TRIGGER,
            Key::GamepadView => VK_GAMEPAD_VIEW,
            Key::GamepadX => VK_GAMEPAD_X,
            Key::GamepadY => VK_GAMEPAD_Y,
            Key::Hangeul => VK_HANGEUL,
            Key::Hangul => VK_HANGUL,
            Key::Hanja => VK_HANJA,
            Key::Help => VK_HELP,
            Key::Home => VK_HOME,
            Key::Ico00 => VK_ICO_00,
            Key::IcoClear => VK_ICO_CLEAR,
            Key::IcoHelp => VK_ICO_HELP,
            Key::IMEOff => VK_IME_OFF,
            Key::IMEOn => VK_IME_ON,
            Key::Insert => VK_INSERT,
            Key::Junja => VK_JUNJA,
            Key::Kana => VK_KANA,
            Key::Kanji => VK_KANJI,
            Key::LaunchApp1 => VK_LAUNCH_APP1,
            Key::LaunchApp2 => VK_LAUNCH_APP2,
            Key::LaunchMail => VK_LAUNCH_MAIL,
            Key::LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT,
            Key::LButton => VK_LBUTTON,
            Key::LControl => VK_LCONTROL,
            Key::LeftArrow => VK_LEFT,
            Key::LMenu => VK_LMENU,
            Key::LShift => VK_LSHIFT,
            Key::MButton => VK_MBUTTON,
            Key::MediaNextTrack => VK_MEDIA_NEXT_TRACK,
            Key::MediaPlayPause => VK_MEDIA_PLAY_PAUSE,
            Key::MediaPrevTrack => VK_MEDIA_PREV_TRACK,
            Key::MediaStop => VK_MEDIA_STOP,
            Key::ModeChange => VK_MODECHANGE,
            Key::Multiply => VK_MULTIPLY,
            Key::NavigationAccept => VK_NAVIGATION_ACCEPT,
            Key::NavigationCancel => VK_NAVIGATION_CANCEL,
            Key::NavigationDown => VK_NAVIGATION_DOWN,
            Key::NavigationLeft => VK_NAVIGATION_LEFT,
            Key::NavigationMenu => VK_NAVIGATION_MENU,
            Key::NavigationRight => VK_NAVIGATION_RIGHT,
            Key::NavigationUp => VK_NAVIGATION_UP,
            Key::NavigationView => VK_NAVIGATION_VIEW,
            Key::NoName => VK_NONAME,
            Key::NonConvert => VK_NONCONVERT,
            Key::None => VK__none_,
            Key::Numlock => VK_NUMLOCK,
            Key::Numpad0 => VK_NUMPAD0,
            Key::Numpad1 => VK_NUMPAD1,
            Key::Numpad2 => VK_NUMPAD2,
            Key::Numpad3 => VK_NUMPAD3,
            Key::Numpad4 => VK_NUMPAD4,
            Key::Numpad5 => VK_NUMPAD5,
            Key::Numpad6 => VK_NUMPAD6,
            Key::Numpad7 => VK_NUMPAD7,
            Key::Numpad8 => VK_NUMPAD8,
            Key::Numpad9 => VK_NUMPAD9,
            Key::OEM1 => VK_OEM_1,
            Key::OEM102 => VK_OEM_102,
            Key::OEM2 => VK_OEM_2,
            Key::OEM3 => VK_OEM_3,
            Key::OEM4 => VK_OEM_4,
            Key::OEM5 => VK_OEM_5,
            Key::OEM6 => VK_OEM_6,
            Key::OEM7 => VK_OEM_7,
            Key::OEM8 => VK_OEM_8,
            Key::OEMAttn => VK_OEM_ATTN,
            Key::OEMAuto => VK_OEM_AUTO,
            Key::OEMAx => VK_OEM_AX,
            Key::OEMBacktab => VK_OEM_BACKTAB,
            Key::OEMClear => VK_OEM_CLEAR,
            Key::OEMComma => VK_OEM_COMMA,
            Key::OEMCopy => VK_OEM_COPY,
            Key::OEMCusel => VK_OEM_CUSEL,
            Key::OEMEnlw => VK_OEM_ENLW,
            Key::OEMFinish => VK_OEM_FINISH,
            Key::OEMFJJisho => VK_OEM_FJ_JISHO,
            Key::OEMFJLoya => VK_OEM_FJ_LOYA,
            Key::OEMFJMasshou => VK_OEM_FJ_MASSHOU,
            Key::OEMFJRoya => VK_OEM_FJ_ROYA,
            Key::OEMFJTouroku => VK_OEM_FJ_TOUROKU,
            Key::OEMJump => VK_OEM_JUMP,
            Key::OEMMinus => VK_OEM_MINUS,
            Key::OEMNECEqual => VK_OEM_NEC_EQUAL,
            Key::OEMPA1 => VK_OEM_PA1,
            Key::OEMPA2 => VK_OEM_PA2,
            Key::OEMPA3 => VK_OEM_PA3,
            Key::OEMPeriod => VK_OEM_PERIOD,
            Key::OEMPlus => VK_OEM_PLUS,
            Key::OEMReset => VK_OEM_RESET,
            Key::OEMWsctrl => VK_OEM_WSCTRL,
            Key::PA1 => VK_PA1,
            Key::Packet => VK_PACKET,
            Key::PageDown => VK_NEXT,
            Key::PageUp => VK_PRIOR,
            Key::Pause => VK_PAUSE,
            Key::Play => VK_PLAY,
            Key::Print => VK_PRINT,
            Key::PrintScr | Key::Snapshot => VK_SNAPSHOT,
            Key::Processkey => VK_PROCESSKEY,
            Key::RButton => VK_RBUTTON,
            Key::RControl => VK_RCONTROL,
            Key::Return => VK_RETURN,
            Key::RightArrow => VK_RIGHT,
            Key::RMenu => VK_RMENU,
            Key::RShift => VK_RSHIFT,
            Key::RWin => VK_RWIN,
            Key::Scroll => VK_SCROLL,
            Key::Select => VK_SELECT,
            Key::Separator => VK_SEPARATOR,
            Key::Shift => VK_SHIFT,
            Key::Sleep => VK_SLEEP,
            Key::Space => VK_SPACE,
            Key::Subtract => VK_SUBTRACT,
            Key::Tab => VK_TAB,
            Key::UpArrow => VK_UP,
            Key::VolumeDown => VK_VOLUME_DOWN,
            Key::VolumeMute => VK_VOLUME_MUTE,
            Key::VolumeUp => VK_VOLUME_UP,
            Key::XButton1 => VK_XBUTTON1,
            Key::XButton2 => VK_XBUTTON2,
            Key::Zoom => VK_ZOOM,
            Key::Unicode(c) => 'unicode_handling: {
                // Handle special characters separately
                match c {
                    '\n' => break 'unicode_handling VK_RETURN,

                    '\r' => { // TODO: What is the correct key to type here?
                         // break 'unicode_handling VK_,
                    }
                    '\t' => break 'unicode_handling VK_TAB,
                    '\0' => {
                        return Err("Invalid mapping");
                    }
                    _ => (),
                }

                let layout = crate::Enigo::get_keyboard_layout();

                let mut buffer = [0; 2];
                let utf16_surrogates = c.encode_utf16(&mut buffer);
                if utf16_surrogates.len() != 1 {
                    return Err("Character can't be mapped to only one virtual key");
                }
                // Translate a character to the corresponding virtual-key code and shift state.
                // If the function succeeds, the low-order byte of the return value contains the
                // virtual-key code and the high-order byte contains the shift state, which can
                // be a combination of the following flag bits. If the function finds no key
                // that translates to the passed character code, both the low-order and
                // high-order bytes contain –1
                let vk = unsafe {
                    windows::Win32::UI::Input::KeyboardAndMouse::VkKeyScanExW(
                        utf16_surrogates[0],
                        layout,
                    )
                };
                // TODO: Check if the condition should be <=
                if vk < 0 {
                    return Err("Character can't be mapped to virtual key");
                }
                VIRTUAL_KEY(vk as u16)
            }
            Key::Other(v) => {
                let Ok(v) = u16::try_from(v) else {
                    return Err("virtual keycodes on Windows have to fit into u16");
                };
                VIRTUAL_KEY(v)
            }
            Key::Super | Key::Command | Key::Windows | Key::Meta | Key::LWin => VK_LWIN,
            #[cfg(feature = "bindable")]
            _ => return Err(()),
        };

        trace!("virtual key: {vk:?})");
        Ok(vk)
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
#[cfg(any(feature = "wayland", feature = "x11rb", feature = "libei"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Modifier {
    #[cfg_attr(feature = "serde", serde(alias = "shift"))]
    Shift,
    #[cfg_attr(feature = "serde", serde(alias = "lock"))]
    Lock,
    #[cfg_attr(feature = "serde", serde(alias = "control"))]
    #[cfg_attr(feature = "serde", serde(alias = "crtl"))]
    Control,
    #[cfg_attr(feature = "serde", serde(alias = "mod1"))]
    #[cfg_attr(feature = "serde", serde(alias = "m1"))]
    Mod1,
    #[cfg_attr(feature = "serde", serde(alias = "mod2"))]
    #[cfg_attr(feature = "serde", serde(alias = "m2"))]
    Mod2,
    #[cfg_attr(feature = "serde", serde(alias = "mod3"))]
    #[cfg_attr(feature = "serde", serde(alias = "m3"))]
    Mod3,
    #[cfg_attr(feature = "serde", serde(alias = "mod4"))]
    #[cfg_attr(feature = "serde", serde(alias = "m4"))]
    Mod4,
    #[cfg_attr(feature = "serde", serde(alias = "mod5"))]
    #[cfg_attr(feature = "serde", serde(alias = "m5"))]
    Mod5,
}

#[cfg(all(unix, not(target_os = "macos")))]
#[cfg(any(feature = "wayland", feature = "x11rb", feature = "libei"))]
impl Modifier {
    /// Returns the bitflag of the modifier that is usually associated with it
    /// on Linux
    #[must_use]
    pub(crate) fn bitflag(self) -> ModifierBitflag {
        match self {
            Self::Shift => 0x1,
            Self::Lock => 0x2,
            Self::Control => 0x4,
            Self::Mod1 => 0x8,
            Self::Mod2 => 0x10,
            Self::Mod3 => 0x20,
            Self::Mod4 => 0x40,
            Self::Mod5 => 0x80,
        }
    }

    /// Returns the number of the modifier that is usually associated with it
    /// on Linux
    #[must_use]
    pub(crate) fn no(self) -> usize {
        match self {
            Self::Shift => 0,
            Self::Lock => 1,
            Self::Control => 2,
            Self::Mod1 => 3,
            Self::Mod2 => 4,
            Self::Mod3 => 5,
            Self::Mod4 => 6,
            Self::Mod5 => 7,
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
#[cfg(any(feature = "wayland", feature = "x11rb", feature = "libei"))]
/// Converts a Key to a modifier
impl TryFrom<Key> for Modifier {
    type Error = &'static str;

    fn try_from(key: Key) -> Result<Self, &'static str> {
        match key {
            Key::Shift | Key::LShift | Key::RShift => Ok(Self::Shift),
            Key::CapsLock => Ok(Self::Lock),
            Key::Control | Key::LControl | Key::RControl => Ok(Self::Control),
            Key::Alt | Key::Option => Ok(Self::Mod1),
            Key::Numlock => Ok(Self::Mod2),
            // The Mod3 modifier is usually unmapped
            // Key::Mod3 => Ok(Self::Mod3),
            Key::Command | Key::Super | Key::Windows | Key::Meta => Ok(Self::Mod4),
            Key::ModeChange => Ok(Self::Mod5),
            _ => Err("not a modifier key"),
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
#[cfg(any(feature = "wayland", feature = "x11rb", feature = "libei"))]
pub(crate) type ModifierBitflag = u32;
