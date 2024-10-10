use std::cmp::min;
use ncurses::*;

struct Pyramid {
    size: u8,
    dot: char,
    core: Vec<Vec<char>>,
}

impl Pyramid {
    fn create_with(size: u8, dot: char) -> Self {
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

    fn erase(&mut self, pos: (i32, i32), erase_char: char) {
        let yi = pos.1 as usize;
        let xi = (pos.0 / 2) as usize;

        if self.core[yi][xi] == 'o' {
            self.core[yi][xi] = erase_char;
        }

        else {
            self.core[yi][xi] = 'o';
        }
    }

    fn draw(&self) {
        for layer in self.core.iter() {
            for dot in layer.iter() {
                let dot_str = format!(" {}", dot);
                addstr(&dot_str as &str).unwrap();
            }
            addstr("\n").unwrap();
        }
    }
}

fn main() {
    let mut pyr = Pyramid::create_with(6, 'o');

    initscr();
    raw();
    noecho();

    let mut turn = 0;
    let mut pos = (1, 0);

    loop {
        pyr.draw();

        mv(pos.1, pos.0);

        let key = getch() as u8 as char;
        match key {
            'q' => break,
            'c' => {
                match turn % 2 {
                    0 => pyr.erase(pos, '-'),
                    1 => pyr.erase(pos, '*'),
                    _ => (),
                }
                turn += 1;
            },
            _ => (),
        }

        pos = move_curs(key, pos, &pyr);

        refresh();
        clear();
    }

    endwin();
}

fn move_curs(key: char, mut pos: (i32, i32), pyr: &Pyramid) -> (i32, i32) {
    let step = 2;
    match key {
        'k' => {
            if pos.1 > 0 {
                pos.1 -= 1;
            }
            pos
        },

        'j' => {
            if pos.0 < 2 * pyr.core[pos.1 as usize].len() as i32 - 1 {
                pos.1 = min(pos.1 + 1, pyr.size as i32 - 1);
            }
            pos
        },

        'h' => {
            if pos.0 > 1 {
                pos.0 -= step;
            }
            pos
        },

        'l' => {
            pos.0 = min(pos.0 + step, 2 * pyr.core[pos.1 as usize].len() as i32 - 1);
            pos
        },
        
        _ => pos
    }
}
