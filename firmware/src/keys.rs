pub enum Key {
    NONE = 0x00,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
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
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    K1 = 0x1E,
    K2 = 0x1F,
    K3 = 0x20,
    K4 = 0x21,
    K5 = 0x22,
    K6 = 0x23,
    K7 = 0x24,
    K8 = 0x25,
    K9 = 0x26,
    K0 = 0x27,
    RETURN = 0x28,
    ESC = 0x29,
    BACKSPACE = 0x2A,
    TAB = 0x2B,
    SPACE = 0x2C,
    DASH = 0x2D,
    EQUAL = 0x2E,
    OBRAKET = 0x2F,
    CBRAKET = 0x30,
    BSLASH = 0x31,
    TILDE = 0x32,
    COLON = 0x33,
    QUOTE = 0x34,
    GACC = 0x35,
    COMMA = 0x36,
    DOT = 0x37,
    SLASH = 0x38,
    CAPSLOCK = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrntScrn = 0x46,
    ScrlLck = 0x47,
    PAUSE = 0x48,
    INSERT = 0x49,
    HOME = 0x4A,
    PgUp = 0x4B,
    DELETE = 0x4C,
    END = 0x4D,
    PgDwn = 0x4E,
    ArrowR = 0x4F,
    ArrowL = 0x50,
    ArrowD = 0x51,
    ArrowU = 0x52,
    NumLck = 0x53,
    NDiv = 0x54,
    NMul = 0x55,
    NSub = 0x56,
    NAdd = 0x57,
    NEnter = 0x58,
    N1 = 0x59,
    N2 = 0x5A,
    N3 = 0x5B,
    N4 = 0x5C,
    N5 = 0x5D,
    N6 = 0x5E,
    N7 = 0x5F,
    N8 = 0x60,
    N9 = 0x61,
    N0 = 0x62,
    NDOT = 0x63,
    BsNPipe = 0x64,
    MENU = 0x65,

    // Multimedia
    //, MUTE = 0x120
    //, VOL_UP = 0x80
    //, VOL_DN = 0x81
    //, MEDIA_PLAYPAUSE = 0xe8
    //, MEDIA_STOPCD = 0xe9
    //, MEDIA_PREVIOUSSONG = 0xea
    //, MEDIA_NEXTSONG = 0xeb
    //, PLAY = 0x00
    //, NEXT = 0x00
    //, PREV = 0x00

    //  Mods
    LCtrl = 0xE0,
    LShft = 0xE1,
    LAlt = 0xE2,
    LSupr = 0xE3,
    RCtrl = 0xE4,
    RShft = 0xE5,
    RAlt = 0xE6,
    RSupr = 0xE7,

    // Layers
    Layer0,
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
    Layer6,
    Layer7,
    Layer8,
    Layer9,
}

pub fn is_mod(key: Key) -> (bool, u8) {
    match key {
        Key::LCtrl => (true, Mod::LCtrl as u8),
        Key::LShft => (true, Mod::LShft as u8),
        Key::LAlt => (true, Mod::LAlt as u8),
        Key::LSupr => (true, Mod::LSupr as u8),
        Key::RCtrl => (true, Mod::RCtrl as u8),
        Key::RShft => (true, Mod::RShft as u8),
        Key::RAlt => (true, Mod::RAlt as u8),
        Key::RSupr => (true, Mod::RSuprg as u8),
        _ => false,
    }
}

pub enum Layer {
    Layer0 = 0,
    Layer1 = 1,
    Layer2 = 2,
    Layer3 = 3,
    Layer4 = 4,
    Layer5 = 5,
    Layer6 = 6,
    Layer7 = 7,
    Layer8 = 8,
    Layer9 = 9,
}

pub enum Mod {
    LCtrl = 0b00000001,
    LShft = 0b00000010,
    LAlt = 0b00000100,
    LSupr = 0b00001000,
    RCtrl = 0b00010000,
    RShft = 0b00100000,
    RAlt = 0b01000000,
    RSupr = 0b10000000,
}
