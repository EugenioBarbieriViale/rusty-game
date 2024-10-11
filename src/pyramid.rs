use ncurses::addstr;

pub struct Pyramid {
    pub size: u8,
    dot: char,
    pub core: Vec<Vec<char>>,
}

impl Pyramid {
    pub fn create_with(size: u8, dot: char) -> Self {
        let mut vec = Vec::new();
        for i in (1..=size).rev() {
            let layer = vec![dot; i as usize];
            vec.push(layer);
        }

        Self {
            size,
            dot,
            core: vec,
        }
    }

    pub fn erase(&mut self, pos: (i32, i32), n_dots: u32, erase_char: char) -> bool {
        let yi = pos.1 as usize;
        let x = (pos.0 / 2) as usize;

        if self.core[yi].len() - x >= n_dots as usize {
            for xi in x..(x + n_dots as usize) {
                if self.core[yi][xi] == self.dot {
                    self.core[yi][xi] = erase_char;
                } 
                else if self.core[yi][xi] == erase_char {
                    self.core[yi][xi] = self.dot;
                }
                else {
                    return true
                }
            }
        }
        false
    }

    pub fn draw(&self) {
        for layer in self.core.iter() {
            for dot in layer.iter() {
                let dot_str = format!(" {}", dot);
                addstr(&dot_str as &str).unwrap();
            }
            addstr("\n").unwrap();
        }
    }
}
