#![allow(unused)]

use minifb::Key;

use renderer::{Renderer, TimeStep};

mod keyboard;
mod renderer;
mod speaker;

fn main() {
    let mut renderer = Renderer::new(10);

    renderer.set_pixel(0, 0);
    renderer.set_pixel(5, 2);

    renderer.render();

    const HERTZ: f32 = 60.;
    const MS_PER_UPDATE: f32 = (1. / HERTZ) * 1000.;

    let mut timestep = TimeStep::new();
    let mut dt = 0.;

    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        dt += timestep.delta();

        while dt >= MS_PER_UPDATE {
            // CPU CYCLE

            dt -= MS_PER_UPDATE;
        }

        renderer.window.update();
    }
}
