#![allow(unused)]

use minifb::Key;

use renderer::{Renderer, TimeStep};

use crate::chip::Chip;

mod chip;
mod keyboard;
mod renderer;
mod speaker;
mod sprites;

fn main() {
    let mut chip = Chip::new(10);

    const HERTZ: f32 = 60.;
    const MS_PER_UPDATE: f32 = (1. / HERTZ) * 1000.;

    let mut timestep = TimeStep::new();
    let mut dt = 0.;

    while chip.is_running() {
        dt += timestep.delta();

        while dt >= MS_PER_UPDATE {
            // CPU CYCLE

            dt -= MS_PER_UPDATE;
        }

        chip.update();
    }
}
