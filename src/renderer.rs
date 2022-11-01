use minifb::{Window, WindowOptions};

const ROWS: usize = 32;
const COLS: usize = 64;

const ON: u32 = 0x000000;
const OFF: u32 = 0xFFFFFF;

pub struct Renderer {
    scale: usize,
    display: Box<[bool; ROWS * COLS]>,
    width: usize,
    height: usize,
    pub window: Window,
}

impl Renderer {
    pub fn new(scale: usize) -> Self {
        let width = COLS * scale;
        let height = ROWS * scale;

        let window = Window::new("CHIP8EMU", width, height, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{e}"));

        Self {
            scale,
            display: Box::new([false; ROWS * COLS]),
            width,
            height,
            window,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) -> bool {
        let px = (y % ROWS) * COLS + (x % COLS);
        self.display[px] ^= true;

        !self.display[px]
    }

    pub fn clear(&mut self) {
        self.display = Box::new([false; ROWS * COLS]);
    }

    pub fn render(&mut self) {
        let mut buf = Vec::with_capacity(self.width * self.height);

        for h in 0..self.height {
            let y = h / self.scale;

            for w in 0..self.width {
                let x = w / self.scale;
                let px = y * COLS + x;

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
