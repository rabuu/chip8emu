#![allow(unused)]

use std::env;
use std::path::Path;

use minifb::Key;

use chip::Chip;
use renderer::{Renderer, TimeStep};

mod chip;
mod keyboard;
mod renderer;
mod speaker;
mod sprites;

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("Expected path to ROM");

    let mut chip = Chip::new(10);

    chip.load_sprites();
    chip.load_rom(Path::new(&path));

    const HERTZ: f32 = 60.;
    const MS_PER_UPDATE: f32 = (1. / HERTZ) * 1000.;

    let mut timestep = TimeStep::new();
    let mut dt = 0.;

    while chip.is_running() {
        dt += timestep.delta();

        chip.handle_input();

        while dt >= MS_PER_UPDATE {
            chip.cycle();
            dt -= MS_PER_UPDATE;
        }

        chip.update();
    }
}
