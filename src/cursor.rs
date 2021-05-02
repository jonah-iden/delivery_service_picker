pub struct Cursor {
    pub mouse_down: bool,
    pub curso_pos: [f64; 2],
    pub old_pos: [f64; 2]
}

impl Cursor {
    pub fn mouse_moved(&mut self, new_pos: [f64; 2]) {
        self.old_pos = self.curso_pos;
        self.curso_pos = new_pos;
    }
}