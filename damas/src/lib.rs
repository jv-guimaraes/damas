#![allow(unused)]
mod coord;

use std::fmt::Display;
use coord::Coord;
use coord::c;

const TABULEIRO_INICIAL: [[char; 8]; 8] = [
    ['P', '.', 'P', '.', 'P', '.', 'P', '.'],
    ['.', 'P', '.', 'P', '.', 'P', '.', 'P'],
    ['P', '.', 'P', '.', 'P', '.', 'P', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['B', '.', 'B', '.', 'B', '.', 'B', '.'],
    ['.', 'B', '.', 'B', '.', 'B', '.', 'B'],
    ['B', '.', 'B', '.', 'B', '.', 'B', '.'],
];

#[derive(Debug)]
enum Vez {
    Branca,
    Preta,
}

#[derive(Debug, Clone, Copy)]
enum Casa {
    Ocupada(Peça),
    Vazia,
}

#[derive(Debug, Clone, Copy)]
pub enum Peça {
    Branca(Coord),
    RainhaBranca(Coord),
    Preta(Coord),
    RainhaPreta(Coord),
}

#[derive(Debug)]
pub struct Jogo {
    tabuleiro: [[Casa; 8]; 8],
    vez: Vez,
}

impl Default for Jogo {
    fn default() -> Self {
        // Construir tabuleiro inicial
        let mut tabuleiro = [[Casa::Vazia; 8]; 8];
        for y in 0..tabuleiro.len() {
            for x in 0..tabuleiro.len() {
                match TABULEIRO_INICIAL[y][x] {
                    'P' => tabuleiro[y][x] = Casa::Ocupada(Peça::Preta(c(x, y))),
                    'B' => tabuleiro[y][x] = Casa::Ocupada(Peça::Branca(c(x, y))),
                    _ => (),
                }
            }
        }
        // Começar o jogo com a peça branca
        let vez = Vez::Branca;
        Jogo { tabuleiro, vez }
    }
}

impl Display for Jogo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("   0  1  2  3  4  5  6  7\n");
        for y in 0..8 {
            buffer.push_str(&format!("{y} "));
            for x in 0..8 {
                match self.tabuleiro[y][x] {
                    Casa::Ocupada(peça) => {
                        match peça {
                            Peça::Branca(_) => buffer.push_str(" b "),
                            Peça::Preta(_) => buffer.push_str(" p "),
                            Peça::RainhaBranca(_) => buffer.push_str(" B "),
                            Peça::RainhaPreta(_) => buffer.push_str(" P "),
                            
                        }
                    },
                    Casa::Vazia => buffer.push_str(" . "),
                }
            }
            buffer.push('\n');
        }
        write!(f, "{}", &buffer)
    }
}

impl Jogo {
    pub fn jogadas_possiveis(&self) -> Vec<(usize, usize)> {
        match self.vez {
            Vez::Branca => {

            },
            Vez::Preta => {

            },
        }
        todo!()
    }

    pub fn peças_brancas(&self) -> Vec<Peça> {
        let mut peças = Vec::new();
        for casa in self.tabuleiro.into_iter().flatten() {
            if let Casa::Ocupada(peça) = casa {
                if let Peça::Branca(_) = peça {
                    peças.push(peça);
                }
            }
        }
        peças
    }

    pub fn peças_pretas(&self) -> Vec<Peça> {
        let mut peças = Vec::new();
        for casa in self.tabuleiro.into_iter().flatten() {
            if let Casa::Ocupada(peça) = casa {
                if let Peça::Preta(_) = peça {
                    peças.push(peça);
                }
            }
        }
        peças
    }
}