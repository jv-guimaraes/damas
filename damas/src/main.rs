#![allow(unused)]
use damas::jogo::Jogo;
use damas::jogo::JogadaResultado;
use damas::coord::c;
use damas::coord::Coord;

use itertools::Itertools;
use std::io::Write;
use std::io;

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn input(msg: &str) -> Coord {
    print!("{msg} ");
    io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed to read stdin");
    buffer.trim().to_string();
    let (x, y) = buffer.split_ascii_whitespace().collect_tuple().unwrap();
    let (x, y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());
    c(x, y)
}

fn main() {
    let mut jogo = Jogo::default();
    println!("{}", jogo);
    loop {
        jogo.todas_possiveis_jogadas().into_iter().for_each(|x| print!("{:?} ", x));
        println!();
        let de = input("De: ");
        let para = input("Para: ");
        let res = jogo.mover(de, para);
        clear_terminal();
        println!("{:?}", res);
        if let JogadaResultado::FimDoJogo(ganhador) = res {
            println!("{} ganhou!!!", if matches!(ganhador, damas::jogo::Vez::Branca) {"Branco"} else {"Preto"});
            break;
        }
        println!("{}\nVez da {:?}:", jogo, jogo.vez);
    }
}