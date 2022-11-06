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
                }

                self.keyboard.key_pressed(k);
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

                let nnn = opcode & 0xfff;
                let x = ((opcode & 0x0f00) >> 8) as u8;
                let y = ((opcode & 0x00f0) >> 4) as u8;
                let kk = (opcode & 0xff) as u8;
                let n = (opcode & 0xf) as u8;

                match opcode & 0xf000 {
                    0x0000 => match opcode & 0x0fff {
                        0x00e0 => self.cls(),
                        0x00ee => self.ret(),
                        _ => (), // ignore legacy instruction
                    },
                    0x1000 => self.jp(nnn),
                    0x2000 => self.call(nnn),
                    0x3000 => self.se_vb(x, kk),
                    0x4000 => self.sne_vb(x, kk),
                    0x5000 => match opcode & 0x000f {
                        0x0000 => self.se_vv(x, y),
                        unknown => panic!("Unknown instruction with opcode {:x}", unknown),
                    },
                    0x6000 => self.ld_vb(x, kk),
                    0x7000 => self.add_vb(x, kk),
                    // TODO: match all the other opcodes
                    unknown => panic!("Unknown instruction with opcode {:x}", unknown),
                }
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
    fn jp(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /// Call subroutine at `nnn`
    fn call(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    /// Skip next instruction if `Vx` == `kk`
    fn se_vb(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] == kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if `Vx` != `kk`
    fn sne_vb(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] != kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if `Vx` == `Vy`
    fn se_vv(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
    }

    /// Set `Vx` = `kk`
    fn ld_vb(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
    }

    /// Set `Vx` = `Vx` + `kk`
    fn add_vb(&mut self, x: u8, kk: u8) {
        self.v[x as usize] += kk;
    }

    /// Set `Vx` = `Vy`
    fn ld_vv(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
    }

    /// Set `Vx` = `Vx` OR `Vy`
    fn or(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` AND `Vy`
    fn and(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` xor `Vy`
    fn xor(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` + `Vy`, set `VF` = carry
    fn add_vv(&mut self, x: u8, y: u8) {
        let res = self.v[x as usize] as u16 + self.v[y as usize] as u16;

        self.v[0xf] = u8::from(res > 0xff);
        self.v[x as usize] = (res & 0xff) as u8;
    }

    /// Set `Vx` = `Vx - Vy`, set `VF` = NOT borrow
    fn sub(&mut self, x: u8, y: u8) {
        self.v[0xf] = u8::from(self.v[x as usize] > self.v[y as usize]);
        self.v[x as usize] -= self.v[y as usize];
    }

    /// Set `Vx` = `Vx` SHR 1
    fn shr(&mut self, x: u8) {
        self.v[0xf] = u8::from(self.v[x as usize] & 1 == 1);
        self.v[x as usize] /= 2;
    }

    /// Set `Vx` = `Vy - Vx`, set `VF` = NOT borrow
    fn subn(&mut self, x: u8, y: u8) {
        self.v[0xf] = u8::from(self.v[y as usize] > self.v[x as usize]);
        self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
    }

    /// Set `Vx` = `Vx` SHL 1
    fn shl(&mut self, x: u8) {
        self.v[0xf] = u8::from(self.v[x as usize] >> 7 == 1);
        self.v[x as usize] *= 2;
    }

    /// Skip next instruction if `Vx` != `Vy`
    fn sne_vv(&mut self, x: u8, y: u8) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
    }

    /// Set `I` = `nnn`
    fn ld_i(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// Jump to location `nnn` + `V0`
    fn jp_v0(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
    }

    /// Set `Vx` = random byte AND `kk`
    fn rnd(&mut self, x: u8, kk: u8) {
        let r: u8 = rand::random();
        self.v[x as usize] = r & kk;
    }

    /// Display `n`-byte sprite starting at memory location `I` at (`Vx`, `Vy`), set `VF` = collision
    fn drw(&mut self, x: u8, y: u8, n: u8) {
        for (row, &(mut sprite)) in self
            .memory
            .iter()
            .skip(self.i as usize - 1)
            .take(n as usize)
            .enumerate()
        {
            for col in 0..8 {
                if sprite & 0x80 != 0 {
                    self.v[0xf] = u8::from(
                        self.renderer
                            .xor_pixel(self.v[x as usize] + col, self.v[y as usize] + row as u8),
                    );
                }

                sprite <<= 1;
            }
        }
    }

    /// Skip next instruction if key with the value of `Vx` is pressed
    fn skp(&mut self, x: u8) {
        let key_code = self.v[x as usize];
        if self.keyboard.is_key_pressed(key_code) {
            self.pc += 2;
        }
    }

    /// Skip next instruction if key with the value of `Vx` is not pressed
    fn sknp(&mut self, x: u8) {
        let key_code = self.v[x as usize];
        if !self.keyboard.is_key_pressed(key_code) {
            self.pc += 2;
        }
    }

    /// Set `Vx` = *delay timer* value
    fn ld_vdt(&mut self, x: u8) {
        self.v[x as usize] = self.delay_timer;
    }

    /// Wait for a key press, store the value of the key in `Vx`
    /// The second part in handled in [Self::handle_input]
    fn ld_k(&mut self, x: u8) {
        self.wait_for_key = Some(x);
    }

    /// Set *delay timer* = `Vx`
    fn ld_dtv(&mut self, x: u8) {
        self.delay_timer = self.v[x as usize];
    }

    /// Set *sound timer* = `Vx`
    fn ld_stv(&mut self, x: u8) {
        self.sound_timer = self.v[x as usize];
    }

    /// Set `I` = `I` + `Vx`
    fn add_i(&mut self, x: u8) {
        self.i += self.v[x as usize] as u16;
    }

    /// Set `I` = location of sprite for digit `Vx`
    fn ld_f(&mut self, x: u8) {
        self.i = self.v[x as usize] as u16 * 5;
    }

    /// Store BCD representation of `Vx` in memory location `I`, `I + 1`, and `I + 2`
    fn ld_b(&mut self, x: u8) {
        let vx = self.v[x as usize];
        self.memory[self.i as usize] = vx / 100;
        self.memory[self.i as usize + 1] = (vx % 100) / 10;
        self.memory[self.i as usize + 2] = vx % 10;
    }

    /// Store registers `V0` through `Vx` in memory starting at location `I`
    fn ld_vstomem(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.memory[self.i as usize + i] = self.v[i];
        }
    }

    /// Read registers `V0` through `Vx` from memory starting at location `I`
    fn ld_vsfrommem(&mut self, x: u8) {
        for i in 0..=x as usize {
            self.v[i] = self.memory[self.i as usize + i];
        }
    }
}
