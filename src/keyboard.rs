use minifb::Key;
use phf::phf_map;

pub static KEYMAP: phf::Map<u8, u8> = phf_map! {
    1u8 => 0x1, // 1
    2u8 => 0x2, // 2
    3u8 => 0x3, // 3
    4u8 => 0xc, // 4

    26u8 => 0x4, // Q
    32u8 => 0x5, // W
    14u8 => 0x6, // E
    27u8 => 0xd, // R

    10u8 => 0x7, // A
    28u8 => 0x8, // S
    13u8 => 0x9, // D
    15u8 => 0xe, // F

    35u8 => 0xa, // Z
    33u8 => 0x0, // X
    12u8 => 0xb, // C
    31u8 => 0xf, // V
};

pub struct Keyboard {
    keys_pressed: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 16],
        }
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keys_pressed[key_code as usize]
    }

    pub fn key_pressed(&mut self, key: Key) {
        let Some(&key_code) = KEYMAP.get(&(key as u8)) else {
            return;
        };

        self.keys_pressed[key_code as usize] = true;
    }

    pub fn key_released(&mut self, key: Key) {
        let Some(&key_code) = KEYMAP.get(&(key as u8)) else {
            return;
        };

        self.keys_pressed[key_code as usize] = false;
    }
}
