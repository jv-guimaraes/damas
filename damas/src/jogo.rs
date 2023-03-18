#![allow(unused)]

use std::fmt::Display;
use crate::coord::Coord;
use crate::coord::c;

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

#[derive(Debug, PartialEq)]
enum Vez {
    Branca,
    Preta,
}

#[derive(Debug, Clone, Copy)]
enum Casa {
    Ocupada(Peça),
    Vazia,
}

impl Casa {
    fn get_peça(self) -> Option<Peça> {
        match self {
            Casa::Ocupada(peça) => Some(peça),
            Casa::Vazia => None,
        }
    }

    fn é_vazia(self) -> bool {
        match self {
            Casa::Ocupada(_) => false,
            Casa::Vazia => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Peça {
    Branca,
    RainhaBranca,
    Preta,
    RainhaPreta,
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
                    'P' => tabuleiro[y][x] = Casa::Ocupada(Peça::Preta),
                    'B' => tabuleiro[y][x] = Casa::Ocupada(Peça::Branca),
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
                            Peça::Branca => buffer.push_str(" x "),
                            Peça::Preta => buffer.push_str(" o "),
                            Peça::RainhaBranca => buffer.push_str(" X "),
                            Peça::RainhaPreta => buffer.push_str(" O "),
                            
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
    pub fn possiveis_jogadas_em(&self, coord: Coord) -> Vec<Coord> {
        if let Some(peça) = self.peça_em(coord) {
            let jogadas: Vec<Coord> = Vec::new();
            let diagonais = match peça {
                Peça::Branca => coord.diagonais_da_frente(),
                Peça::RainhaBranca => coord.diagonais_da_frente(),
                Peça::Preta => coord.diagonais_de_trás(),
                Peça::RainhaPreta => coord.diagonais_de_trás(),
            };
            diagonais.into_iter().filter(|x| self.casa_em(*x).é_vazia()).collect()
        } else {
            vec![]
        }
    }

    pub fn peças_brancas(&self) -> Vec<Peça> {
        let mut peças = Vec::new();
        for casa in self.tabuleiro.into_iter().flatten() {
            if let Casa::Ocupada(peça) = casa {
                if let Peça::Branca = peça {
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
                if let Peça::Preta = peça {
                    peças.push(peça);
                }
            }
        }
        peças
    }

    fn peça_em(&self, coord: Coord) -> Option<Peça> {
        self.tabuleiro[coord.y][coord.x].get_peça()
    }

    fn casa_em(&self, coord: Coord) -> Casa {
        self.tabuleiro[coord.y][coord.x]
    }

    pub fn mover(&mut self, de: Coord, para: Coord) {
        let peça = self.peça_em(de).unwrap();
        self.tabuleiro[de.y][de.x] = Casa::Vazia;
        self.tabuleiro[para.y][para.x] = Casa::Ocupada(peça);

    }
}