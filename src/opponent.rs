use rand::Rng;
use crate::pyramid::*;

struct RandValues {
    pos: (i32, i32),
    n_dots: u32,
}

impl RandValues {
    fn new(max_pos: (i32, i32), max_dots: u32) -> Self {
        let rand_x = rand::thread_rng().gen_range(0..=max_pos.0);
        let rand_y = rand::thread_rng().gen_range(0..=max_pos.1);
        let rand_dots  = rand::thread_rng().gen_range(0..=max_dots);

        Self {
            pos: (rand_x, rand_y),
            n_dots: rand_dots,
        } 
    }
}

pub fn erase(pyr: &mut Pyramid, pos: (i32, i32), n_dots: u32, erase_char: char) -> bool {
    let rand_vals = RandValues::new(pos, n_dots);
    pyr.erase(rand_vals.pos, rand_vals.n_dots, erase_char)
}
