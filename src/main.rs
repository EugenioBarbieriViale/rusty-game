use std::io::{self, Write};
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
}

fn main() {
    let pyr = Pyramid::create_with(6, 'o');

    initscr();
    raw();
    noecho();

    loop {
        draw(&pyr);
        // refresh();

        match getch() as u8 as char {
            'q' => break,
            _ => (),
        }

        clear();
    }

    endwin();
}

fn draw(pyr: &Pyramid) {
    for layer in pyr.core.iter() {
        for dot in layer.iter() {
            let dot_str = format!(" {} ", pyr.dot);
            addstr(&dot_str as &str).unwrap();
        }
        addstr("\n").unwrap();
    }
}
