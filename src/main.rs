mod tools;
use tools::*;
fn main() {
    let window = Window::new_centered("IZAHID", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler::new());
}
