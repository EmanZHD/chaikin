use minifb::{ MouseButton, MouseMode, Window };

pub fn draw_point(buffer: &mut [u32], width: usize, height: usize, x: usize, y: usize, color: u32) {
    for dy in 0..3 {
        for dx in 0..3 {
            let px = x + dx - 1;
            let py = y + dy - 1;
            if px < width && py < height {
                buffer[py * width + px] = color;
            }
        }
    }
}

pub fn detect_click(window: &mut Window, control_points: &mut Vec<(usize, usize)>) {
    if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Discard) {
        if window.get_mouse_down(MouseButton::Left) {
            let point = (mx as usize, my as usize);
            if !control_points.contains(&point) {
                control_points.push(point);
            }
        }
    }
}
