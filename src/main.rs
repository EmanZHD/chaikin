mod tools;
use tools::*;

fn main() {
    let window = Window::new_centered("Chaikin", (840, 620)).unwrap();
    window.run_loop(MyWindowHandler::new());
}