#![allow(unused)]
use damas::Jogo;
use damas::coord::c;
use damas::coord::Coord;
use itertools::Itertools;

fn input() -> Coord {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    buffer.trim().to_string();
    let (x, y) = buffer.split_ascii_whitespace().collect_tuple().unwrap();
    let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
    c(x, y)
}

fn main() {
    let mut jogo = Jogo::default();
    println!("{}", jogo);
    // loop {
    //     println!("De: ");
    //     let de = input();
    //     println!("Para: ");
    //     let para = input();
    //     jogo.mover(de, para);
    //     println!("{}", jogo);
    // }

    println!("{:?}", c(3, 4).diagonais_da_rainha());
}