use std::io::{self, Write};

fn main() {
    let size = 6;
    let dot_char = 'o';

    let pir = pyramid(dot_char, size);

    loop {
        draw(&pir);

        print!("\nEnter col, start and finish: ");
        io::stdout().flush().unwrap();

        let mut coords = String::new();
        io::stdin().read_line(&mut coords).expect("Could not get coordinates :(");
    }
}

fn pyramid(dot: char, size: u8) -> Vec<Vec<char>> {
    let mut vec = Vec::new();
    for i in (1..=size).rev() {
        let layer = vec![dot; i as usize];
        vec.push(layer);
    }
    vec
}

fn draw(pir: &Vec<Vec<char>>) {
    for (i, layer) in pir.iter().enumerate() {
        for (j, dot) in layer.iter().enumerate() {
            print!(" {} ", dot);
        }
        println!("{i}");
    }
}
