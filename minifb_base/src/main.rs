use chaikin::*;
use minifb::{ Window, WindowOptions };

fn main() {
    let width = 640;
    let height = 480;
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut control_points: Vec<(usize, usize)> = Vec::new();

    let mut window = Window::new(
        "Chaikin - izahid",
        width,
        height,
        WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("window creqtion fqiled: {}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        buffer.fill(0);
        detect_click(&mut window, &mut control_points);

        for &(x, y) in &control_points {
            draw_point(&mut buffer, width, height, x, y, 0xffffff);
        }
        // break;

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
