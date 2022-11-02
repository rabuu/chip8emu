use minifb::Key;
use phf::phf_map;

static KEYMAP: phf::Map<u8, u8> = phf_map! {
    01u8 => 0x1, // 1
    02u8 => 0x2, // 2
    03u8 => 0x3, // 3
    04u8 => 0xc, // 4

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

    pub fn on_key_down(&mut self, key: Key) {
        let key_code = KEYMAP[&(key as u8)];
        self.keys_pressed[key_code as usize] = true;

        // TODO: next key press stuff missing
    }

    pub fn on_key_up(&mut self, key: Key) {
        let key_code = KEYMAP[&(key as u8)];
        self.keys_pressed[key_code as usize] = false;
    }
}
