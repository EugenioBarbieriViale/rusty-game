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
    let mut erased_dots = 0;
    let mut collision = false;

    let mut pos = (1, 0);

    loop {
        if erased_dots == 21 || collision {
            break;
        }

        pyr.draw();

        mv(pos.1, pos.0);

        let key = getch() as u8 as char;
        match key {
            'q' => break,

            c if c.is_numeric() => {
                let c = c.to_digit(10).unwrap();

                match turn % 2 {
                    0 => {
                        collision = pyr.erase(pos, c, '-');
                        erased_dots += c;
                    },
                    1 => {
                        collision = pyr.erase(pos, c, '*');
                        erased_dots += c;
                    },
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

    if collision {
        println!("Erasing already erased dots is not admitted!");
    }
    else {
        match turn % 2 {
            0 => println!("You won!"),
            1 => println!("Machine won!"),
            _ => (),
        }
    }
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
