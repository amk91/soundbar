use anyhow::{Result, bail};

#[derive(Debug, Clone)]
pub enum KeyCode {
    SPACE = 0x20,
    
    PAGE_UP = 0x21,
    PAGE_DOWN = 0x22,
    END = 0x23,
    HOME = 0x24,
    
    LEFT_ARROW = 0x25,
    UP_ARROW = 0x26,
    RIGHT_ARROW = 0x27,
    DOWN_ARROW = 0x28,
    
    SELECT = 0x29,
    PRINT = 0x2A,
    SNAPSHOT = 0x2C,
    INSERT = 0x2D,
    DELETE = 0x2E,

    SEMICOLOMN = 0xBA,
    EQUAL = 0xBB,
    COMMA = 0xBC,
    MINUS = 0xBD,
    FORSLASH = 0xBF,
    ACCENT = 0xC0,
    OPEN_PAR = 0xDB,
    BACKSLASH = 0xDC,
    CLOSE_PAR = 0xDD,
    QUOTE = 0xDE,
    
    NUM_0 = 0x30,
    NUM_1 = 0x31,
    NUM_2 = 0x32,
    NUM_3 = 0x33,
    NUM_4 = 0x34,
    NUM_5 = 0x35,
    NUM_6 = 0x36,
    NUM_7 = 0x37,
    NUM_8 = 0x38,
    NUM_9 = 0x39,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    NUMPAD_0 = 0x60,
    NUMPAD_1 = 0x61,
    NUMPAD_2 = 0x62,
    NUMPAD_3 = 0x63,
    NUMPAD_4 = 0x64,
    NUMPAD_5 = 0x65,
    NUMPAD_6 = 0x66,
    NUMPAD_7 = 0x67,
    NUMPAD_8 = 0x68,
    NUMPAD_9 = 0x69,
    MULTIPLY = 0x6A,
    ADD = 0x6B,
    SEPARATOR = 0x6C,
    SUBTRACT = 0x6D,
    DECIMAL = 0x6E,
    DIVIDE = 0x6F,

    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,

    NUM_LOCK = 0x90,
    SCROLL_LOCK = 0x91,
}

impl TryFrom<u32> for KeyCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x20 => Ok(KeyCode::SPACE),
            0x21 => Ok(KeyCode::PAGE_UP),
            0x22 => Ok(KeyCode::PAGE_DOWN),
            0x23 => Ok(KeyCode::END),
            0x24 => Ok(KeyCode::HOME),
            0x25 => Ok(KeyCode::LEFT_ARROW),
            0x26 => Ok(KeyCode::UP_ARROW),
            0x27 => Ok(KeyCode::RIGHT_ARROW),
            0x28 => Ok(KeyCode::DOWN_ARROW),
            0x29 => Ok(KeyCode::SELECT),
            0x2A => Ok(KeyCode::PRINT),
            0x2C => Ok(KeyCode::SNAPSHOT),
            0x2D => Ok(KeyCode::INSERT),
            0x2E => Ok(KeyCode::DELETE),
            0xBA => Ok(KeyCode::SEMICOLOMN),
            0xBB => Ok(KeyCode::EQUAL),
            0xBC => Ok(KeyCode::COMMA),
            0xBD => Ok(KeyCode::MINUS),
            0xBF => Ok(KeyCode::FORSLASH),
            0xC0 => Ok(KeyCode::ACCENT),
            0xDB => Ok(KeyCode::OPEN_PAR),
            0xDC => Ok(KeyCode::BACKSLASH),
            0xDD => Ok(KeyCode::CLOSE_PAR),
            0xDE => Ok(KeyCode::QUOTE),
            0x30 => Ok(KeyCode::NUM_0),
            0x31 => Ok(KeyCode::NUM_1),
            0x32 => Ok(KeyCode::NUM_2),
            0x33 => Ok(KeyCode::NUM_3),
            0x34 => Ok(KeyCode::NUM_4),
            0x35 => Ok(KeyCode::NUM_5),
            0x36 => Ok(KeyCode::NUM_6),
            0x37 => Ok(KeyCode::NUM_7),
            0x38 => Ok(KeyCode::NUM_8),
            0x39 => Ok(KeyCode::NUM_9),
            0x41 => Ok(KeyCode::A),
            0x42 => Ok(KeyCode::B),
            0x43 => Ok(KeyCode::C),
            0x44 => Ok(KeyCode::D),
            0x45 => Ok(KeyCode::E),
            0x46 => Ok(KeyCode::F),
            0x47 => Ok(KeyCode::G),
            0x48 => Ok(KeyCode::H),
            0x49 => Ok(KeyCode::I),
            0x4A => Ok(KeyCode::J),
            0x4B => Ok(KeyCode::K),
            0x4C => Ok(KeyCode::L),
            0x4D => Ok(KeyCode::M),
            0x4E => Ok(KeyCode::N),
            0x4F => Ok(KeyCode::O),
            0x50 => Ok(KeyCode::P),
            0x51 => Ok(KeyCode::Q),
            0x52 => Ok(KeyCode::R),
            0x53 => Ok(KeyCode::S),
            0x54 => Ok(KeyCode::T),
            0x55 => Ok(KeyCode::U),
            0x56 => Ok(KeyCode::V),
            0x57 => Ok(KeyCode::W),
            0x58 => Ok(KeyCode::X),
            0x59 => Ok(KeyCode::Y),
            0x5A => Ok(KeyCode::Z),
            0x60 => Ok(KeyCode::NUMPAD_0),
            0x61 => Ok(KeyCode::NUMPAD_1),
            0x62 => Ok(KeyCode::NUMPAD_2),
            0x63 => Ok(KeyCode::NUMPAD_3),
            0x64 => Ok(KeyCode::NUMPAD_4),
            0x65 => Ok(KeyCode::NUMPAD_5),
            0x66 => Ok(KeyCode::NUMPAD_6),
            0x67 => Ok(KeyCode::NUMPAD_7),
            0x68 => Ok(KeyCode::NUMPAD_8),
            0x69 => Ok(KeyCode::NUMPAD_9),
            0x6A => Ok(KeyCode::MULTIPLY),
            0x6B => Ok(KeyCode::ADD),
            0x6C => Ok(KeyCode::SEPARATOR),
            0x6D => Ok(KeyCode::SUBTRACT),
            0x6E => Ok(KeyCode::DECIMAL),
            0x6F => Ok(KeyCode::DIVIDE),
            0x70 => Ok(KeyCode::F1),
            0x71 => Ok(KeyCode::F2),
            0x72 => Ok(KeyCode::F3),
            0x73 => Ok(KeyCode::F4),
            0x74 => Ok(KeyCode::F5),
            0x75 => Ok(KeyCode::F6),
            0x76 => Ok(KeyCode::F7),
            0x77 => Ok(KeyCode::F8),
            0x78 => Ok(KeyCode::F9),
            0x79 => Ok(KeyCode::F10),
            0x7A => Ok(KeyCode::F11),
            0x7B => Ok(KeyCode::F12),
            0x7C => Ok(KeyCode::F13),
            0x7D => Ok(KeyCode::F14),
            0x7E => Ok(KeyCode::F15),
            0x7F => Ok(KeyCode::F16),
            0x80 => Ok(KeyCode::F17),
            0x81 => Ok(KeyCode::F18),
            0x82 => Ok(KeyCode::F19),
            0x83 => Ok(KeyCode::F20),
            0x84 => Ok(KeyCode::F21),
            0x85 => Ok(KeyCode::F22),
            0x86 => Ok(KeyCode::F23),
            0x87 => Ok(KeyCode::F24),
            0x90 => Ok(KeyCode::NUM_LOCK),
            0x91 => Ok(KeyCode::SCROLL_LOCK),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SysKeyCode {
    SHIFT = 0x10,
    CTRL = 0x11,
    ALT = 0x12,
    LSHIFT = 0xA0,
    RSHIFT = 0xA1,
    LCTRL = 0xA2,
    RCTRL = 0xA3,
    LALT = 0xA4,
    RALT = 0xA5,
}

impl TryFrom<u32> for SysKeyCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(SysKeyCode::SHIFT),
            0x11 => Ok(SysKeyCode::CTRL),
            0x12 => Ok(SysKeyCode::ALT),
            0xA0 => Ok(SysKeyCode::LSHIFT),
            0xA1 => Ok(SysKeyCode::RSHIFT),
            0xA2 => Ok(SysKeyCode::LCTRL),
            0xA3 => Ok(SysKeyCode::RCTRL),
            0xA4 => Ok(SysKeyCode::LALT),
            0xA5 => Ok(SysKeyCode::RALT),
            _ => Err(()),
        }
    }
}

pub type KeyTaskCode = u32;

#[derive(Default, Debug)]
pub struct KeyTask {
    pub key: Option<KeyCode>,
    pub sys_key: Option<SysKeyCode>,
}

impl KeyTask {
    pub fn new(key: KeyCode, sys_key: Option<SysKeyCode>) -> KeyTask {
        KeyTask { key: Some(key), sys_key }
    }

    pub fn try_new(key: u32, sys_key: u32) -> Result<KeyTask> {
        let key = if let Ok(x) = KeyCode::try_from(key) {
            x
        } else {
            bail!("Unable to generate KeyCode from key {} ({:#04X})", key, key);
        };

        let sys_key = if let Ok(x) = SysKeyCode::try_from(sys_key) {
            x
        } else {
            bail!("Unable to generate KeyCode from sys_key {} ({:#04X})", sys_key, sys_key);
        };

        Ok(KeyTask {
            key: Some(key),
            sys_key: Some(sys_key),
        })
    }

    pub fn try_from(key_code: KeyTaskCode) -> Result<KeyTask> {
        let sys_key = key_code >> 2;
        let key = key_code & 0x0F;

        KeyTask::try_new(key, sys_key)
    }

    pub fn get_code(&self) -> KeyTaskCode {
        if let Some(key) = &self.key {
            let mut code = if let Some(sys_key) = &self.sys_key {
                (sys_key.clone() as u32) << 2
            } else {
                0
            };
            code |= key.clone() as u32;
            code
        } else {
            0
        }
    }
}
