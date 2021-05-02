
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use crate::cursor::Cursor;
use crate::options::DelService;
use webbrowser;

const ORDER_TEXT: &str = "Bestellen";
const BUTTON_BACKGROUND: [f32; 4] = [0.7, 0.7, 0.7, 1.0];
const BUTTON_TEXT_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct OrderButton<'btn> {
    pos: [f64; 2],
    url: &'btn str,
    show_btn: bool,
    glyphs: GlyphCache<'btn>,
    clicked: bool,
    text_size: [f64; 2],
    margin: f64,
}

impl OrderButton<'_> {
    pub fn new<'btn>(pos: [f64; 2]) -> OrderButton<'btn> {
        use graphics::*;

        let mut glyphs = GlyphCache::new("assets/Roboto-Black.ttf", (), TextureSettings::new()).unwrap();
        let text_height = 24;
        let text_width = glyphs.width(text_height, ORDER_TEXT).unwrap_or(0.0);


        OrderButton {
            pos: pos,
            url: "",
            show_btn: true,
            glyphs: glyphs,
            clicked: false,
            text_size: [text_width, text_height as f64],
            margin: 20.0
        }
    }

    pub fn draw(&mut self, c: graphics::Context, gl: &mut GlGraphics) {
        use graphics::*;

        rectangle(BUTTON_BACKGROUND, [self.pos[0], self.pos[1], self.text_size[0] + self.margin, self.text_size[1] + self.margin], 
            c.transform, gl);

        let transform = c.transform
            .trans(self.pos[0] + 10.0, self.pos[1] + self.text_size[1] as f64 + 10.0);    
        text::Text::new_color(BUTTON_TEXT_COLOR, self.text_size[1] as u32).draw(
            ORDER_TEXT,
            & mut self.glyphs,
            &c.draw_state,
            transform, gl
        ).unwrap();
        
    }

    pub fn update(&mut self, cursor: &Cursor, url: Option<&str>) {
        if url.is_some() && cursor.mouse_down && self.check_within_btn_bounds(cursor.curso_pos) && !self.clicked {
            self.clicked = true;
            webbrowser::open(url.unwrap());
        } else if !cursor.mouse_down {
            self.clicked = false
        }
    }

    pub fn check_within_btn_bounds(&self, pos: [f64; 2]) -> bool {
        let width = self.text_size[0] + self.margin;
        let height = self.text_size[1] + self.margin;

        return pos[0] > self.pos[0] && pos[0] < self.pos[0] + width &&
            pos[1] > self.pos[1] && pos[1] < self.pos[1] + height
    }
}