use renderer::Renderer;

mod renderer;

fn main() {
    let mut renderer = Renderer::new(10);

    renderer.set_pixel(0, 0);
    renderer.set_pixel(5, 2);

    renderer.render();

    loop {}
}
