// use minifb::{Window, WindowOptions};

// fn main() {
//     let width = 640;
//     let height = 480;
//     let mut buffer: Vec<u32> = vec![0; width * height];

//     let mut window = Window::new(
//         "Canvas in Rust - minifb",
//         width,
//         height,
//         WindowOptions::default(),
//     )
//     .unwrap();

//     while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
//         // Fill the buffer with a color gradient or any drawing
//         for y in 0..height {
//             for x in 0..width {
//                 let color = ((x ^ y) * 0x010101) as u32;
//                 buffer[y * width + x] = color;
//             }
//         }

//         window.update_with_buffer(&buffer, width, height).unwrap();
//     }
// }