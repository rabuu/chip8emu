use std::time::Instant;

use minifb::{Window, WindowOptions};

const ROWS: u8 = 32;
const COLS: u8 = 64;

const OFF: u32 = 0x000000;
const ON: u32 = 0xFFFFFF;

pub struct Renderer {
    scale: usize,
    display: Box<[bool; ROWS as usize * COLS as usize]>,
    width: usize,
    height: usize,
    pub window: Window,
}

impl Renderer {
    pub fn new(scale: usize) -> Self {
        let width = COLS as usize * scale;
        let height = ROWS as usize * scale;

        let window = Window::new("CHIP8EMU", width, height, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{e}"));

        Self {
            scale,
            display: Box::new([false; ROWS as usize * COLS as usize]),
            width,
            height,
            window,
        }
    }

    pub fn xor_pixel(&mut self, x: u8, y: u8) -> bool {
        let px = (y as usize % ROWS as usize) * COLS as usize + (x as usize % COLS as usize);
        self.display[px] ^= true;

        !self.display[px]
    }

    pub fn clear(&mut self) {
        self.display = Box::new([false; ROWS as usize * COLS as usize]);
    }

    pub fn render(&mut self) {
        let mut buf = Vec::with_capacity(self.width * self.height);

        for h in 0..self.height {
            let y = h / self.scale;

            for w in 0..self.width {
                let x = w / self.scale;
                let px = y * COLS as usize + x;

                if self.display[px] {
                    buf.push(ON);
                } else {
                    buf.push(OFF);
                }
            }
        }

        self.window
            .update_with_buffer(&buf, self.width, self.height)
            .unwrap();
    }
}

pub struct TimeStep {
    last: Instant,
    delta: f32,
}

impl TimeStep {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
            delta: 0.,
        }
    }

    pub fn delta(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last).as_micros() as f32 * 0.001;

        self.last = now;
        self.delta = delta;

        delta
    }
}
