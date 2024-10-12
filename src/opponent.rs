use rand::Rng;
use crate::pyramid::*;

struct RandValues {
    pos: (i32, i32),
    n_dots: u32,
}

fn rand(high: usize) -> usize {
    rand::thread_rng().gen_range(0..high)
}

impl RandValues {
    fn new(pyr: &Pyramid) -> Self {
        let rand_y = rand(pyr.size as usize);
        let rand_x = rand(pyr.core[rand_y].len());
        // let rand_dots = rand((pyr.core[rand_y].len() - rand_x).try_into().unwrap());
        let rand_dots = 1;

        Self {
            pos: (rand_x as i32, rand_y as i32),
            n_dots: rand_dots as u32,
        } 
    }
}

pub fn erase(pyr: &mut Pyramid, erase_char: char) -> bool {
    let rand_vals = RandValues::new(&pyr);
    pyr.erase(rand_vals.pos, rand_vals.n_dots, erase_char)
}
