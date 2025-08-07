use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::VirtualKeyCode;
use speedy2d::window::{WindowHandler, WindowHelper};
pub use speedy2d::{Graphics2D, Window};
use std::time::{Duration, Instant};

// use crate::window::VirtualKeyCode;
pub struct MyWindowHandler {
    points: Vec<Vector2<f32>>,
    points_temp: Vec<Vector2<f32>>,
    point: Vector2<f32>,
    last_update: Instant,
    is_draw: bool,
    step: i8,
}

impl MyWindowHandler {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            points_temp: Vec::new(),
            point: Vector2 { x: 0.0, y: 0.0 },
            last_update: Instant::now(),
            is_draw: false,
            step: 0,
        }
    }

    fn chaickin_algo(&mut self) {
        if self.last_update.elapsed() >= Duration::new(1, 0) {
            if self.step == 0 {
                self.points_temp = self.points.clone();
                self.step += 1;
            } else {
                if self.step % 8 != 0 {
                    // println!("step :  {:?}", self.step);
                    let mut temp: Vec<Vector2<f32>> = Vec::new();
                    for i in 0..self.points_temp.len() - 1 {
                        if i == 0 {
                            temp.push(self.points_temp[0]);
                        }
                        if i + 1 < self.points_temp.len() {
                            let r: Vector2<f32> = Vector2 {
                                x: (0.25 * self.points_temp[i].x
                                    + 0.75 * self.points_temp[i + 1].x),
                                y: (0.25 * self.points_temp[i].y
                                    + 0.75 * self.points_temp[i + 1].y),
                            };
                            let q: Vector2<f32> = Vector2 {
                                x: (0.75 * self.points_temp[i].x
                                    + 0.25 * self.points_temp[i + 1].x),
                                y: (0.75 * self.points_temp[i].y
                                    + 0.25 * self.points_temp[i + 1].y),
                            };

                            temp.push(q);
                            temp.push(r);
                        }
                    }
                    temp.push(self.points_temp[self.points_temp.len() - 1]);
                    self.points_temp = temp;

                    self.step += 1;
                }
            }
            self.last_update = Instant::now();
        }
    }
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);
        for p in &self.points {
            graphics.draw_circle(p.clone(), 5.0, Color::CYAN);
            graphics.draw_circle(p.clone(), 2.0, Color::BLACK);
        }

        if self.is_draw {
            //// REMOVE THIS IF AND KEEP JUST ELSE
            if self.points_temp.len() == 2 {
                graphics.draw_line(self.points_temp[0], self.points_temp[1], 2.0, Color::GREEN);
            } else {
                for i in 0..self.points_temp.len() {
                    if i + 1 < self.points_temp.len() {
                        graphics.draw_line(
                            self.points_temp[i],
                            self.points_temp[i + 1],
                            2.0,
                            Color::WHITE,
                        );
                        
                        //// REMOVE THIS TWO CIRCLES
                        graphics.draw_circle(self.points_temp[i], 2.0, Color::RED);
                        graphics.draw_circle(self.points_temp[i + 1], 2.0, Color::RED);
                    }
                }
                self.chaickin_algo();
                if self.step == 8 {
                    self.step = 0;
                }
            }
        }
        _helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(key) = virtual_key_code {
            match key {
                VirtualKeyCode::Return => {
                    if self.points.len() > 1 {
                        self.last_update = Instant::now();
                        self.is_draw = true;
                    } else {
                        println!("No points to process. Please add a point first.");
                    }
                }
                VirtualKeyCode::Escape => {
                    helper.terminate_loop();
                }
                VirtualKeyCode::C => {
                    self.points.clear();
                    self.points_temp.clear();
                    self.is_draw = false;
                    self.step = 0;
                }
                _ => {}
            }
        }
    }
    fn on_mouse_button_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        if button == speedy2d::window::MouseButton::Left {
            if !self.is_draw {
                self.points.push(self.point);
            }
        }
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2) {
        self.point = position;
    }
}
