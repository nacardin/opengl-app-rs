extern crate glium;
extern crate glium_text;
extern crate cgmath;

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium_text::{FontTexture, TextDisplay};

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let system = glium_text::TextSystem::new(&display);

    // Creating a `FontTexture`, which a regular `Texture` which contains the font.
    // Note that loading the systems fonts is not covered by this library.
    let font = glium_text::FontTexture::new(&display, std::fs::File::open(&std::path::Path::new("/usr/share/fonts/TTF/Oxygen-Sans.ttf")).unwrap(), 256).unwrap();

    // Creating a `TextDisplay` which contains the elements required to draw a specific sentence.
    let text = glium_text::TextDisplay::new(&system, &font, "Hello world!");

    // Finally, drawing the text is done like this:
    let matrix = [[0.1, 0.0, 0.0, 0.0],
                        [0.0, 0.1, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]];

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        glium_text::draw(&text, &system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
