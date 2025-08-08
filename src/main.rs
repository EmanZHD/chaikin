//this to include tools file or module
mod tools;
use tools::*;
//this imports (use) everything from the tools module
fn main() {
    //create new window 
    let window = Window::new_centered("Chaikin", (840, 620)).unwrap();
    //this start even loop keep running until u quit
    window.run_loop(MyWindowHandler::new());
    // struct that stores program state (points, steps, etc.)
}

//  (adding two new points between each pair)