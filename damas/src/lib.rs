#![allow(unused)]

use std::fmt::Display;

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

#[derive(Debug, Clone, Copy)]
enum Casa {
    Ocupada(Peça),
    Vazia,
}

#[derive(Debug, Clone, Copy)]
enum Peça {
    Branca,
    Preta,
}

#[derive(Debug)]
pub struct Jogo {
    tabuleiro: [[Casa; 8]; 8],
    vez: Peça,
}

impl Default for Jogo {
    fn default() -> Self {
        // Construir tabuleiro inicial
        let mut tabuleiro = [[Casa::Vazia; 8]; 8];
        for y in 0..tabuleiro.len() {
            for x in 0..tabuleiro.len() {
                match TABULEIRO_INICIAL[y][x] {
                    'P' => tabuleiro[y][x] = Casa::Ocupada(Peça::Preta),
                    'B' => tabuleiro[y][x] = Casa::Ocupada(Peça::Branca),
                    _ => (),
                }
            }
        }
        // Começar o jogo com a peça branca
        let vez = Peça::Branca;
        Jogo { tabuleiro, vez }
    }
}

impl Display for Jogo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for y in 0..8 {
            for x in 0..8 {
                match self.tabuleiro[y][x] {
                    Casa::Ocupada(peça) => {
                        match peça {
                            Peça::Branca => buffer.push_str(" B "),
                            Peça::Preta => buffer.push_str(" P "),
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