use crate::options::*;
use crate::math_helper::{angle, vec};
use crate::cursor::*;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use std::f64::consts::PI;


pub struct SpinnyWheel<'a> {
    rotation: f64,
    rot_speed: f64,
    resistance: f64,
    options: &'a Vec<DelService>,
    glyphs: GlyphCache<'a>,
    grip_point: [f64; 2],
    pub result: Option<&'a DelService>
}
const WHEEL_COLOR:  [f32; 4] = [1.0,1.0,1.0,1.0];
const TEXT_COLOR:  [f32; 4] = [0.0,0.0,0.0,1.0];
pub const WHEEL_RADIUS: f64 = 300.0;

impl SpinnyWheel<'_> {
    pub fn new<'a>(options: &'a Vec<DelService>, glyphs: &'a mut GlyphCache<'a>,) -> SpinnyWheel<'a> {
        SpinnyWheel {
            rotation: 0.0,
            rot_speed: 0.0,
            resistance: 0.001,
            options: options,
            glyphs: GlyphCache::new("assets/Roboto-Black.ttf", (), TextureSettings::new()).unwrap(),
            grip_point: [0.0, 0.0],
            result: None
        }
    }

    pub fn draw(&mut self, c: graphics::Context, gl: &mut GlGraphics, wheel_center: [f64; 2]) {
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
                & mut self.glyphs,
                &c.draw_state,
                transform, gl
            ).unwrap();
        }
    }

    pub fn update(&mut self, cursor: &Cursor) {
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