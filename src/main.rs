extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod math_helper;
use math_helper::*;
mod options;
use options::*;
mod spinny_wheel;
use spinny_wheel::*;
mod cursor;
use cursor::*;
mod order_button;
use order_button::*;

const BACKGROUND: [f32; 4] = [0.0,0.0,0.0,1.0];
struct App<'a> {
    spinny_wheel: SpinnyWheel<'a>,
    cursor: Cursor,
    gl: GlGraphics,
    order_button: OrderButton<'a>
}

impl App<'_> {
    fn draw(&mut self, args: &RenderArgs) {
        let spinny_wheel = &mut self.spinny_wheel;
        let order_button = &mut self.order_button;
        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(BACKGROUND, gl);

            let view_center = [(c.get_view_size()[0] / 2.0), (c.get_view_size()[1] / 2.0)];

            spinny_wheel.draw(c, gl, view_center);

            let transform = c.transform
                .trans(view_center[0], view_center[1] - WHEEL_RADIUS);

            let triangle = [[- 10.0, -15.0], [0.0, 30.0], [10.0, -15.0]];

            polygon([0.8, 0.8, 0.8, 1.0], &triangle, transform, gl);

            if spinny_wheel.result.is_some() {
                order_button.draw(c, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.spinny_wheel.update(&self.cursor);
        self.order_button.update(&self.cursor, self.spinny_wheel.result.map(|service| &service.url[..]))
    }
}


fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("delivery service picker", [1280, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // let options = vec!["test1", "test2", "test3", "test4", "test4"];
    let options = read_services_from_json("src/options.json");

    let mut glyphs: GlyphCache = GlyphCache::new("assets/Roboto-Black.ttf", (), TextureSettings::new()).unwrap();

    let mut app = App { 
        gl: GlGraphics::new(opengl),
        spinny_wheel: SpinnyWheel::new(&options, &mut glyphs),
        cursor: Cursor {
            mouse_down: false,
            curso_pos: [0.0, 0.0],
            old_pos: [0.0, 0.0]
        },
        order_button: OrderButton::new([10.0, 360.0])
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {  
        if let Some(args) = e.update_args() {
            app.update(&args)
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            app.cursor.mouse_down = true;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            app.cursor.mouse_down = false;
        }

        e.mouse_cursor(|pos| {
            app.cursor.mouse_moved(pos);
            //println!("mouse pos: {}, {}", pos[0], pos[1])
        });
    
        if let Some(args) = e.render_args() {
            app.draw(&args)
        }
    }
}




