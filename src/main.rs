use std::io::{self, Write};

#[derive(Debug)]
struct InputCoords(u8, u8, u8);

impl InputCoords {
    fn get(size: u8) -> Self {
        println!("\nEnter the line you want to erase.");

        print!("Enter column (0 to {size}): ");
        io::stdout().flush().unwrap();

        let mut column = String::new();
        io::stdin().read_line(&mut column).expect("Could not get coordinates :(");


        print!("Enter the starting x (0 to {size}: ");
        io::stdout().flush().unwrap();

        let mut x1 = String::new();
        io::stdin().read_line(&mut x1).expect("Could not get coordinates :(");


        print!("Enter the ending x (0 to {size}: ");
        io::stdout().flush().unwrap();

        let mut x2 = String::new();
        io::stdin().read_line(&mut x2).expect("Could not get coordinates :(");

        let column = column.trim().parse().expect("Not a number");
        let x1 = x1.trim().parse().expect("Not a number");
        let x2 = x2.trim().parse().expect("Not a number");

        Self(column, x1, x2)
    }
}

fn main() {
    let size = 6;

    let pir = pyramid(size);

    loop {
        draw(&pir);

        let input = InputCoords::get(size);
        println!("\n{:?}\n", input);
        io::stdout().flush().unwrap();
    }
}

fn pyramid(size: u8) -> Vec<Vec<char>> {
    let mut vec = Vec::new();
    for i in (1..=size).rev() {
        let layer = vec!['o'; i as usize];
        vec.push(layer);
    }
    vec
}

fn draw(pir: &Vec<Vec<char>>) {
    for (i, layer) in pir.iter().enumerate() {
        for (j, dot) in layer.iter().enumerate() {
            print!(" {} ", dot);
        }
        println!("");
    }
}
