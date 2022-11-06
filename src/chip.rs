use std::{fs, path::Path};

use minifb::{Key, KeyRepeat};

use crate::{
    keyboard::{self, Keyboard},
    renderer::Renderer,
    speaker::Speaker,
    sprites::SPRITES,
};

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

    stack: Vec<u16>,

    wait_for_key: Option<u8>, // If waiting, store `Vx`

    speed: u32, // how many instructions are evaluated every cycle
}

impl Chip {
    pub fn new(speed: u32) -> Self {
        Self {
            renderer: Renderer::new(10),
            keyboard: Keyboard::new(),
            speaker: Speaker::new(440., 0.3),
            memory: Box::new([0; 4096]),
            v: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200, // start of most Chip-8 programs
            stack: Vec::new(),
            wait_for_key: None,
            speed,
        }
    }

    pub fn is_running(&self) -> bool {
        self.renderer.window.is_open() && !self.renderer.window.is_key_down(Key::Escape)
    }

    pub fn update(&mut self) {
        self.renderer.window.update();
    }

    pub fn handle_input(&mut self) {
        self.renderer
            .window
            .get_keys_pressed(KeyRepeat::No)
            .into_iter()
            .for_each(|k| {
                if let Some(x) = self.wait_for_key {
                    if let Some(&key) = keyboard::KEYMAP.get(&(k as u8)) {
                        self.v[x as usize] = key;
                        self.wait_for_key = None;
                    }
                } else {
                    self.keyboard.key_pressed(k)
                }
            });

        self.renderer
            .window
            .get_keys_released()
            .into_iter()
            .for_each(|k| self.keyboard.key_released(k));
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
            if self.wait_for_key.is_none() {
                let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
                    | self.memory[self.pc as usize + 1] as u16;
                // TODO: Implement instructions
            }
        }

        // update timers
        if self.wait_for_key.is_none() {
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

    /// Clear the display
    fn cls(&mut self) {
        self.renderer.clear();
    }

    /// Return from a subroutine
    fn ret(&mut self) {
        self.pc = self.stack.pop().unwrap_or(0);
    }

    /// Jump to location `nnn`
    fn jp(&mut self, opcode: u16) {
        let nnn = opcode & 0xfff;
        self.pc = nnn;
    }

    /// Call subroutine at `nnn`
    fn call(&mut self, opcode: u16) {
        self.stack.push(self.pc);

        let nnn = opcode & 0xfff;
        self.pc = nnn;
    }

    /// Skip next instruction if `Vx` == `kk`
    fn se_vb(&mut self, opcode: u16) {
        let x = opcode & 0xf;
        let kk = (opcode & 0xff) as u8;

        if self.v[x as usize] == kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if `Vx` != `kk`
    fn sne_vb(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let kk = (opcode & 0xff) as u8;

        if self.v[x as usize] != kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if `Vx` == `Vy`
    fn se_vv(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
    }

    /// Set `Vx` = `kk`
    fn ld_vb(&mut self, opcode: u16) {
        let x = opcode & 0xf;
        let kk = (opcode & 0xff) as u8;

        self.v[x as usize] = kk;
    }

    /// Set `Vx` = `Vx` + `kk`
    fn add_vb(&mut self, opcode: u16) {
        let x = opcode & 0xf;
        let kk = (opcode & 0xff) as u8;

        self.v[x as usize] += kk;
    }

    /// Set `Vx` = `Vy`
    fn ld_vv(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        self.v[x as usize] = self.v[y as usize];
    }

    /// Set `Vx` = `Vx` OR `Vy`
    fn or(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        self.v[x as usize] |= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` AND `Vy`
    fn and(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        self.v[x as usize] &= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` xor `Vy`
    fn xor(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        self.v[x as usize] ^= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` + `Vy`, set `VF` = carry
    fn add_vv(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        let res = self.v[x as usize] as u16 + self.v[y as usize] as u16;

        self.v[0xf] = if res > 0xff { 1 } else { 0 };
        self.v[x as usize] = (res & 0xff) as u8;
    }

    /// Set `Vx` = `Vx - Vy`, set `VF` = NOT borrow
    fn sub(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        self.v[0xf] = if self.v[x as usize] > self.v[y as usize] {
            1
        } else {
            0
        };

        self.v[x as usize] -= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` SHR 1
    fn shr(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;

        self.v[0xf] = if self.v[x as usize] & 1 == 1 { 1 } else { 0 };
        self.v[x as usize] /= 2;
    }

    /// Set `Vx` = `Vy - Vx`, set `VF` = NOT borrow
    fn subn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        self.v[0xf] = if self.v[y as usize] > self.v[x as usize] {
            1
        } else {
            0
        };

        self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
    }

    /// Set `Vx` = `Vx` SHL 1
    fn shl(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;

        self.v[0xf] = if self.v[x as usize] >> 7 == 1 { 1 } else { 0 };
        self.v[x as usize] *= 2;
    }

    /// Skip next instruction if `Vx` != `Vy`
    fn sne_vv(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;

        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
    }

    /// Set `I` = `nnn`
    fn ld_i(&mut self, opcode: u16) {
        let nnn = opcode & 0xfff;

        self.i = nnn;
    }

    /// Jump to location `nnn` + `V0`
    fn jp_v0(&mut self, opcode: u16) {
        let nnn = opcode & 0xfff;

        self.pc = nnn + self.v[0] as u16;
    }

    /// Set `Vx` = random byte AND `kk`
    fn rnd(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let kk = (opcode & 0xff) as u8;
        let r: u8 = rand::random();

        self.v[x as usize] = r & kk;
    }

    /// Display `n`-byte sprite starting at memory location `I` at (`Vx`, `Vy`), set `VF` = collision
    fn drw(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;
        let n = (opcode & 0xf) as u8;

        for (row, &(mut sprite)) in self
            .memory
            .iter()
            .skip(self.i as usize - 1)
            .take(n as usize)
            .enumerate()
        {
            for col in 0..8 {
                if sprite & 0x80 != 0 {
                    if self
                        .renderer
                        .xor_pixel(self.v[x as usize] + col, self.v[y as usize] + row as u8)
                    {
                        self.v[0xf] = 1;
                    } else {
                        self.v[0xf] = 0;
                    }
                }

                sprite <<= 1;
            }
        }
    }

    /// Skip next instruction if key with the value of `Vx` is pressed
    fn skp(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;

        let key_code = self.v[x as usize];

        if self.keyboard.is_key_pressed(key_code) {
            self.pc += 2;
        }
    }

    /// Skip next instruction if key with the value of `Vx` is not pressed
    fn sknp(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;

        let key_code = self.v[x as usize];

        if !self.keyboard.is_key_pressed(key_code) {
            self.pc += 2;
        }
    }

    /// Set `Vx` = *delay timer* value
    fn ld_dt(&mut self, opcode: u16) {
        let x = ((opcode & 0x0f00) >> 8) as u8;

        self.v[x as usize] = self.delay_timer;
    }
}
