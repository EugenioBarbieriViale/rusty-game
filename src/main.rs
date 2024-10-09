// TODO: 
// - enum of directions
// - erase dots

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
}

fn main() {
    let mut pyr = Pyramid::create_with(6, 'o');

    initscr();
    raw();
    noecho();

    let mut pos = (0, 0);

    loop {
        draw(&pyr);

        mv(pos.1, pos.0);

        let key = getch() as u8 as char;
        match key {
            'q' => break,
            'c' => erase(pos, &mut pyr),
            _ => (),
        }

        pos = move_curs(key, pos, &pyr);

        refresh();
        clear();

        // let coords = format!("{}, {}", cur_x, cur_y);
        // addstr(&coords as &str).unwrap();
    }

    endwin();

    // for (i, layer) in pyr.core.iter().enumerate() {
    //     for (j, dot) in layer.iter().enumerate() {
    //         print!("({j}, {i})");
    //     }
    //     println!("\n");
    // }
}

fn erase(pos: (i32, i32), pyr: &mut Pyramid) {
    pyr.core[pos.1 as usize][pos.0 as usize] = '-';
}

fn move_curs(key: char, mut pos: (i32, i32), pyr: &Pyramid) -> (i32, i32) {
    match key {
        'k' => {
            if pos.1 > 0 {
                pos.1 -= 1;
            }
            pos
        },

        'j' => {
            pos.1 = min(pos.1 + 1, pyr.size as i32 - 1);
            pos
        },

        'h' => {
            if pos.0 > 0 {
                pos.0 -= 1;
            }
            pos
        },

        'l' => {
            pos.0 = min(pos.0 + 1, (pyr.core[pos.1 as usize].len() - 1).try_into().unwrap());
            pos
        },
        
        _ => pos
    }
}

fn draw(pyr: &Pyramid) {
    for layer in pyr.core.iter() {
        for _dot in layer.iter() {
            let dot_str = format!("{}", pyr.dot);
            addstr(&dot_str as &str).unwrap();
        }
        addstr("\n").unwrap();
    }
}
