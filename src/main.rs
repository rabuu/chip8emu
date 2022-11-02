use minifb::Key;

use renderer::{Renderer, TimeStep};

mod keyboard;
mod renderer;

fn main() {
    let mut renderer = Renderer::new(10);

    renderer.set_pixel(0, 0);
    renderer.set_pixel(5, 2);

    renderer.render();

    let mut timestep = TimeStep::new();
    let mut dt = 0.;
    const MS_PER_UPDATE: f32 = 16.6666667; // ~60Hz

    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        dt += timestep.delta();

        while dt >= MS_PER_UPDATE {
            // CPU CYCLE

            dt -= MS_PER_UPDATE;
        }

        renderer.window.update();
    }
}
