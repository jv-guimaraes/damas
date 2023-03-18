use std::fmt::Display;
use itertools::Itertools;

use crate::coord::Coord;
use crate::coord::c;

const TABULEIRO_INICIAL: [[char; 8]; 8] = [
    ['P', '.', 'P', '.', 'P', '.', 'P', '.'],
    ['.', 'P', '.', 'P', '.', 'P', '.', 'P'],
    ['P', '.', 'P', '.', '.', '.', 'P', '.'],
    ['.', '.', '.', '.', 'P', '.', '.', '.'],
    ['.', '.', '.', 'B', '.', '.', '.', '.'],
    ['B', '.', '.', '.', 'B', '.', 'B', '.'],
    ['.', 'B', '.', 'B', '.', 'B', '.', 'B'],
    ['B', '.', 'B', '.', 'B', '.', 'B', '.'],
];

#[derive(Debug, PartialEq)]
enum Vez {
    Branca,
    Preta,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Peça {
    Branca,
    RainhaBranca,
    Preta,
    RainhaPreta,
}

impl Peça {
    fn é_branca(self) -> bool {
        matches!(self, Peça::Branca | Peça::RainhaBranca)
    }

    fn é_preta(self) -> bool {
        matches!(self, Peça::Preta | Peça::RainhaPreta)
    }
}

#[derive(Debug)]
pub struct Jogo {
    tabuleiro: [[Casa; 8]; 8],
    vez: Vez,
    tem_que_comer: bool,
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
        Jogo { tabuleiro, vez, tem_que_comer: false }
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

#[derive(Debug)]
pub enum Jogada {
    Mover(Coord),
    Comer(Coord, Coord),
}

impl Jogo {
    pub fn possiveis_jogadas_em(&self, coord: Coord) -> Vec<Jogada> {
        let mut jogadas = Vec::new();
        
        // Caso a casa dada por coord esteja vazia
        if let Casa::Vazia = self.casa_em(coord) { return vec![]; }
        
        // Caso não seja a vez da peça que está em coord
        let peça = self.peça_em(coord).unwrap();
        if !self.é_a_vez_de(peça) { return vec![];}
        
        // Computar casas andaveis
        let casas: Vec<Coord> = match peça {
            Peça::Branca => coord.diagonais_frente().into_iter().filter(|c| self.casa_em(*c).é_vazia()).collect(),
            Peça::Preta => coord.diagonais_atrás().into_iter().filter(|c| self.casa_em(*c).é_vazia()).collect(),
            _ => coord.diagonais_rainha().into_iter().filter(|c| self.casa_em(*c).é_vazia()).collect(),
        };
        jogadas = casas.into_iter().map(Jogada::Mover).collect();

        // Computar casa comíveis
        let mut comiveis = vec![];
        for vizinho in coord.diagonais_comiveis() {
            if let Casa::Vazia = self.casa_em(vizinho) { continue; }
            if self.é_a_vez_de(self.peça_em(vizinho).unwrap()) { continue; }

            let pulo = coord.distancia(vizinho).vezes(2);
            if self.casa_em(coord + pulo).é_vazia() {
                comiveis.push(Jogada::Comer(vizinho, coord + pulo));
            }
        }
        
        if comiveis.is_empty() {
            jogadas
        } else {
            comiveis
        }
    }

    fn peça_em(&self, coord: Coord) -> Option<Peça> {
        self.casa_em(coord).get_peça()
    }

    fn casa_em(&self, coord: Coord) -> Casa {
        self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    pub fn mover(&mut self, origem: Coord, destino: Coord) -> bool {
        if let Casa::Vazia = self.casa_em(origem) { return false; }
        if let Casa::Ocupada(_) = self.casa_em(destino) { return false; }        
        
        let peça = self.peça_em(origem).unwrap();
        if peça.é_branca() && self.vez == Vez::Preta { return false; }
        if peça.é_preta() && self.vez == Vez::Branca { return false; }

        self.tabuleiro[origem.y as usize][origem.x as usize] = Casa::Vazia;
        self.tabuleiro[destino.y as usize][destino.x as usize] = Casa::Ocupada(peça);
        true

    }

    fn é_a_vez_de(&self, peça: Peça) -> bool {
        if peça.é_branca() && self.vez == Vez::Preta { return false; }
        if peça.é_preta() && self.vez == Vez::Branca { return false; }
        true
    }
}