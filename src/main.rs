#![deny(unsafe_code)]
#![deny(clippy::all)]

mod ppu;

use fltk::app;
use fltk::prelude::*;
use fltk::window::Window;

fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 400, 300, "z-gb");

    {
        // let image = image::;
    }

    window.end();
    window.show();
    app.run().unwrap();
}
