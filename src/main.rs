use ncurses::*;
use std::cmp::min;

mod pyramid;
mod opponent;

use crate::pyramid::Pyramid;

fn main() {
    let mut pyr = Pyramid::create_with(6, 'o', '.');

    initscr();
    raw();
    noecho();

    let mut turn = 0;
    let mut erased_dots = 0;

    let mut coll_human = false;
    let mut coll_opp = false;

    let mut pos = (1, 0);

    loop {
        pyr.draw();

        if erased_dots == 21 || coll_human || coll_opp {
            mv(pyr.size as i32 + 1, 1);
            score_board(coll_human, coll_opp, turn);

            pyr.reset();
        }

        mv(pos.1, pos.0);

        let key = getch() as u8 as char;
        match key {
            'q' => break,

            n if n.is_numeric() => {
                let n = n.to_digit(10).unwrap();

                if turn % 2 == 0 {
                    coll_human = pyr.erase(pos, n);
                    erased_dots += n;
                }
                turn += 1;
            }

            _ => (),
        }

        if turn % 2 == 1 {
            coll_opp = opponent::erase(&mut pyr);
            turn += 1;
            // return erased dots and add to collision var
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

fn score_board(coll_human: bool, coll_opp: bool, turn: usize) {
    if coll_human {
        addstr("Erasing already erased dots is not admitted! (your fault)").unwrap();
    }
    else if coll_opp {
        addstr("Erasing already erased dots is not admitted! (opponent's fault)").unwrap();
    }
    else {
        match turn % 2 {
            0 => {addstr("You won!").unwrap();},
            1 => {addstr("Opponent won!").unwrap();},
            _ => (),
        }
    }
}
