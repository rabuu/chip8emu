use std::{fs, path::Path};

use minifb::Key;

use crate::{keyboard::Keyboard, renderer::Renderer, speaker::Speaker, sprites::SPRITES};

pub struct Chip {
    renderer: Renderer,
    keyboard: Keyboard,
    speaker: Speaker,

    memory: Box<[u8; 4096]>, // 4kB
    v: [u8; 16],
    i: u16,

    delay_timer: u8,
    sound_timer: u8,

    pc: u16,
    sp: u8,

    stack: [u16; 16],

    paused: bool,
    speed: u32, // how many instructions are evaluated every cycle
}

impl Chip {
    pub fn new(speed: u32) -> Self {
        Self {
            renderer: Renderer::new(10),
            keyboard: Keyboard::new(),
            speaker: Speaker::new(660., 0.7),
            memory: Box::new([0; 4096]),
            v: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200, // start of most Chip-8 programs
            sp: 0,
            stack: [0; 16],
            paused: false,
            speed,
        }
    }

    pub fn is_running(&self) -> bool {
        self.renderer.window.is_open() && !self.renderer.window.is_key_down(Key::Escape)
    }

    pub fn update(&mut self) {
        self.renderer.window.update();
    }

    pub fn load_sprites(&mut self) {
        for (i, sprite) in SPRITES.into_iter().flatten().enumerate() {
            self.memory[i] = sprite;
        }
    }

    pub fn load_rom(&mut self, path: &Path) {
        let program = fs::read(path).expect("Couldn't read file");

        for (loc, byte) in program.into_iter().enumerate() {
            self.memory[0x200 + loc] = byte;
        }
    }

    pub fn cycle(&mut self) {
        // execute instructions
        for _ in 0..self.speed {
            if !self.paused {
                let opcode = self.memory[self.pc as usize] << 8 | self.memory[self.pc as usize + 1];
                // TODO: Implement instructions
            }
        }

        // update timers
        if !self.paused {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }

        // play sound if sound timer is non-zero
        if self.sound_timer != 0 {
            self.speaker.play();
        } else {
            self.speaker.stop();
        }
    }
}
