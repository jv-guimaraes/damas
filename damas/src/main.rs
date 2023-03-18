#![allow(unused)]
use damas::jogo::Jogo;
use damas::coord::c;
use damas::coord::Coord;

use itertools::Itertools;

fn input() -> Coord {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    buffer.trim().to_string();
    let (x, y) = buffer.split_ascii_whitespace().collect_tuple().unwrap();
    let (x, y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());
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

    println!("{:?}", c(7, 7).diagonais_rainha());
}