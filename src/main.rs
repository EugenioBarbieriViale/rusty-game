use ncurses::*;
use std::cmp::min;

mod pyramid;
use pyramid::*;

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
            x if x.is_numeric() => {
                match turn % 2 {
                    0 => pyr.erase(pos, x.to_digit(10).unwrap(), '-'),
                    1 => pyr.erase(pos, x.to_digit(10).unwrap(), '*'),
                    _ => (),
                }
                turn += 1;
            }
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
        }

        'j' => {
            if pos.0 < 2 * pyr.core[pos.1 as usize].len() as i32 - 1 {
                pos.1 = min(pos.1 + 1, pyr.size as i32 - 1);
            }
            pos
        }

        'h' => {
            if pos.0 > 1 {
                pos.0 -= step;
            }
            pos
        }

        'l' => {
            pos.0 = min(pos.0 + step, 2 * pyr.core[pos.1 as usize].len() as i32 - 1);
            pos
        }

        _ => pos,
    }
}
