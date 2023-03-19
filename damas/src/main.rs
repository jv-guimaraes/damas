#![allow(unused)]
use damas::jogo::Jogo;
use damas::coord::c;
use damas::coord::Coord;

use itertools::Itertools;

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn input(msg: &str) -> Coord {
    println!("{msg}");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    buffer.trim().to_string();
    let (x, y) = buffer.split_ascii_whitespace().collect_tuple().unwrap();
    let (x, y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());
    c(x, y)
}

fn main() {
    let mut jogo = Jogo::default();
    jogo.todas_possiveis_jogadas().into_iter().for_each(|x| println!("{:?}", x));
    println!("{}", jogo);
    loop {
        let de = input("De: ");
        let para = input("Para: ");
        let res = jogo.mover(de, para);
        clear_terminal();
        println!("{}", if res { "sucesso" } else { "falha" });
        println!("{}\nVez da {:?}:", jogo, jogo.vez);
    }

    println!("{:?}", jogo.todas_possiveis_jogadas());
}