extern crate glium;
extern crate glium_text;
extern crate cgmath;

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium_text::{FontTexture, TextDisplay};

    let window_builder = glium::glutin::WindowBuilder::new();
    let display = window_builder.build_glium().unwrap();
    let window = display.get_window().unwrap();

    let system = glium_text::TextSystem::new(&display);

    let font_file = std::fs::File::open(&std::path::Path::new("src/fonts/sf_display_regular.ttf")).unwrap();

    let font = glium_text::FontTexture::new(&display, font_file, 64).unwrap();

    let text = glium_text::TextDisplay::new(&system, &font, "Hello World");
    let text_width = text.get_width();

    let text_color = (0.1, 0.1, 0.1, 1.0);

    println!("tw: {}",  text_width);

    let mut a = 0.0f32;

    loop {

        a = a + 0.01f32;

        if(a > 1.0f32) {
            a = -1.0f32;
        }

        let (window_width, window_height) = window.get_inner_size_pixels().unwrap();
        // println!("width: {}",  window_width);
        // println!("height: {}",  window_height);

        let width_unit = 2.0f32 / window_width as f32;
        let height_unit = 2.0f32 / window_height as f32;

        let x_mod = text_width * 8f32 * width_unit;
        let y_mod = 56f32 * height_unit;

        let matrix = [[x_mod, 0.0, 0.0, 0.0],
                            [0.0, y_mod, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [-1.0 + a, 0.0, 0.0, 1.0]];


        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        glium_text::draw(&text, &system, &mut target, matrix, text_color);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
