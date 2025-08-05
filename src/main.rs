
use minifb::{Window, WindowOptions};

fn main() {
    let width = 640;
    let height = 480;
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Chaikin - izahid",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // for y in 0..height {
        //     for x in 0..width {
                let color = ( 0x010101) as u32;
                buffer[width] = color;
        //     }
        // }


        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}