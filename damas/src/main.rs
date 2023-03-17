#![allow(unused)]
use damas::Jogo;

fn main() {
    let jogo = Jogo::default();
    println!("{}", jogo);

    println!("{:?}", jogo.peças_brancas());
    println!("{:?}", jogo.peças_pretas());
}