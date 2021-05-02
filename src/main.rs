extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::f64::consts::PI as PI;

mod math_helper;
use math_helper::*;
mod options;
use options::*;

const BACKGROUND: [f32; 4] = [0.0,0.0,0.0,1.0];
struct App<'a> {
    spinny_wheel: SpinnyWheel<'a>,
    cursor: Cursor,
    gl: GlGraphics
}

impl App<'_> {
    fn draw(&mut self, args: &RenderArgs) {
        let spinny_wheel = &mut self.spinny_wheel;
        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            clear(BACKGROUND, gl);

            let view_center = [(c.get_view_size()[0] / 2.0), (c.get_view_size()[1] / 2.0)];

            spinny_wheel.draw(c, gl, view_center);

            let transform = c.transform
                .trans(view_center[0], view_center[1] - WHEEL_RADIUS);

            let triangle = [[- 10.0, -15.0], [0.0, 30.0], [10.0, -15.0]];

            polygon([0.8, 0.8, 0.8, 1.0], &triangle, transform, gl);

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.spinny_wheel.update(&self.cursor)
    }
}


struct SpinnyWheel<'a> {
    rotation: f64,
    rot_speed: f64,
    resistance: f64,
    options: &'a Vec<DelService>,
    glyphs: &'a mut GlyphCache<'a>,
    grip_point: [f64; 2],
    result: Option<&'a DelService>
}
const WHEEL_COLOR:  [f32; 4] = [1.0,1.0,1.0,1.0];
const TEXT_COLOR:  [f32; 4] = [0.0,0.0,0.0,1.0];
const WHEEL_RADIUS: f64 = 300.0;

impl SpinnyWheel<'_> {
    fn new<'a>(options: &'a Vec<DelService>, glyphs: &'a mut GlyphCache<'a>,) -> SpinnyWheel<'a> {
        SpinnyWheel {
            rotation: 0.0,
            rot_speed: 0.0,
            resistance: 0.001,
            options: options,
            glyphs: glyphs,
            grip_point: [0.0, 0.0],
            result: None
        }
    }

    fn draw(&mut self, c: graphics::Context, gl: &mut GlGraphics, wheel_center: [f64; 2]) {
        use graphics::*;

        let transform = c
            .transform
            .trans(wheel_center[0] - WHEEL_RADIUS, wheel_center[1] - WHEEL_RADIUS);
                
        ellipse(WHEEL_COLOR, [0.0,0.0,WHEEL_RADIUS * 2.0,WHEEL_RADIUS * 2.0], transform, gl);
        
        let n_options = self.options.len();
        for (i, option) in self.options.iter().enumerate() {
            let angle = ((i as f64)/ n_options as f64) * (PI * 2.0) + self.rotation;
            //print!("angle {} : {} \n", angle, option);
            line_from_to([0.0,0.0,0.0,1.0], 2.0, wheel_center, 
                SpinnyWheel::calc_line_end(wheel_center, angle + (std::f64::consts::PI / n_options as f64)), 
                c.transform, gl);
            
            let height = 24;
            let width = self.glyphs.width(height, &option.name).unwrap_or(0.0);
            let transform = c
                .transform
                .trans(wheel_center[0], wheel_center[1])
                .rot_rad(angle)
                .trans((WHEEL_RADIUS / 2.0) - (width as f64 / 2.0) , height as f64 / 2.0);
            
            text::Text::new_color(TEXT_COLOR, height).draw(
                &option.name,
                self.glyphs,
                &c.draw_state,
                transform, gl
            ).unwrap();
        }
    }

    fn update(&mut self, cursor: &Cursor) {
        let wheel_center = [640.0, 360.0];
        if cursor.mouse_down {
            if self.grip_point == [0.0, 0.0] {
                self.grip_point = cursor.curso_pos
            }

            self.rot_speed = angle(vec(wheel_center, self.grip_point), vec(wheel_center, cursor.curso_pos)) / 10.0
        } else {
            self.grip_point = [0.0 ,0.0]
        }
        self.rotation += self.rot_speed;

        if self.rot_speed > 0.001 {
            self.result = None;
            self.rot_speed -= self.resistance;
        } else if self.rot_speed < -0.001 {
            self.result = None;
            self.rot_speed += self.resistance;
        } else {
            if self.result.is_none() {
                let res_option = &self.options[self.calc_result_index()];
                println!("result is: {}", res_option.name);
                self.result = Some(res_option);
            }
            self.rot_speed = 0.0;
        }
    }
    
    fn calc_result_index(&self) -> usize {
        let pi2 = 2.0 * PI;
        let n_options = self.options.len();
        let option_slice_size = pi2 / n_options as f64;
        let norm_rotation = self.rotation % pi2;
        let slice_number = (norm_rotation / option_slice_size).round();
        return (n_options - 1) - ((n_options as i32 + slice_number as i32) % n_options as i32) as usize;
    }

    fn calc_line_end([x, y]: [f64; 2], angle: f64) -> [f64; 2] {
        return [x + WHEEL_RADIUS * angle.cos(), y + WHEEL_RADIUS * angle.sin()]
    }
}

struct Cursor {
    mouse_down: bool,
    curso_pos: [f64; 2],
    old_pos: [f64; 2]
}

impl Cursor {
    fn mouse_moved(&mut self, new_pos: [f64; 2]) {
        self.old_pos = self.curso_pos;
        self.curso_pos = new_pos;
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
        }

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




