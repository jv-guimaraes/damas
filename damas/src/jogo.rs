use itertools::Itertools;
use std::fmt::Display;

use crate::coord::{c, Coord};

const TABULEIRO_INICIAL: [[char; 8]; 8] = [
    ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
    ['.', 'p', '.', 'p', '.', 'p', '.', '.'],
    ['P', '.', 'P', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', 'P', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['B', '.', 'B', 'B', '.', '.', 'B', 'B'],
    ['.', 'b', '.', 'b', '.', 'b', '.', '.'],
    ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
];

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn é_rainha(self) -> bool {
        matches!(self, Peça::RainhaBranca | Peça::RainhaPreta)
    }
}

#[derive(Debug, PartialEq)]
pub enum Jogada {
    Mover(Coord),
    Comer(Coord, Coord),
    RComer(Coord, Vec<Coord>),
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
                    'p' => tabuleiro[y][x] = Casa::Ocupada(Peça::Preta),
                    'b' => tabuleiro[y][x] = Casa::Ocupada(Peça::Branca),
                    'P' => tabuleiro[y][x] = Casa::Ocupada(Peça::RainhaPreta),
                    'B' => tabuleiro[y][x] = Casa::Ocupada(Peça::RainhaBranca),
                    '.' => (),
                    c => panic!("{c} não é uma peça válida!"),
                }
            }
        }
        // Começar o jogo com a peça branca
        Jogo {
            tabuleiro,
            vez: Vez::Branca,
        }
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
                    Casa::Ocupada(peça) => match peça {
                        Peça::Branca => buffer.push_str(" x "),
                        Peça::Preta => buffer.push_str(" o "),
                        Peça::RainhaBranca => buffer.push_str(" X "),
                        Peça::RainhaPreta => buffer.push_str(" O "),
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
    pub fn new(tabuleiro: [[char; 8]; 8]) -> Self {
        // Construir tabuleiro inicial
        let mut tab = [[Casa::Vazia; 8]; 8];
        for y in 0..tab.len() {
            for x in 0..tab.len() {
                match tabuleiro[y][x] {
                    'p' => tab[y][x] = Casa::Ocupada(Peça::Preta),
                    'b' => tab[y][x] = Casa::Ocupada(Peça::Branca),
                    'P' => tab[y][x] = Casa::Ocupada(Peça::RainhaPreta),
                    'B' => tab[y][x] = Casa::Ocupada(Peça::RainhaBranca),
                    '.' => (),
                    c => panic!("{c} não é uma peça válida!"),
                }
            }
        }
        // Começar o jogo com a peça branca
        Jogo {
            tabuleiro: tab,
            vez: Vez::Branca,
        }
    }

    pub fn mover(&mut self, origem: Coord, destino: Coord) -> bool {
        todo!()
    }

    pub fn possiveis_jogadas(&self, coord: Coord) -> Vec<Jogada> {
        // Caso a casa dada por coord esteja vazia
        if let Casa::Vazia = self.casa_em(coord) {
            return vec![];
        }

        // Caso não seja a vez da peça que está em coord
        let peça_selecionada = self.peça_em(coord).unwrap();
        if !self.é_a_vez_de(peça_selecionada) {
            return vec![];
        }

        match peça_selecionada {
            Peça::Branca | Peça::Preta => self.possiveis_jogadas_peão(coord),
            Peça::RainhaBranca | Peça::RainhaPreta => self.possiveis_jogadas_rainha(coord),
        }
    }

    fn possiveis_jogadas_peão(&self, coord: Coord) -> Vec<Jogada> {
        let peça_selecionada = self.peça_em(coord).unwrap();

        // Computar casas andaveis
        let casas: Vec<Coord> = match peça_selecionada {
            Peça::Branca => coord
                .diagonais_frente()
                .into_iter()
                .filter(|c| self.casa_em(*c).é_vazia())
                .collect(),
            Peça::Preta => coord
                .diagonais_atrás()
                .into_iter()
                .filter(|c| self.casa_em(*c).é_vazia())
                .collect(),
            _ => vec![],
        };
        let andaveis = casas.into_iter().map(Jogada::Mover).collect();

        // Computar casa comíveis
        let mut comiveis = vec![];
        for vizinho in coord.diagonais_comiveis() {
            if let Casa::Vazia = self.casa_em(vizinho) {
                continue;
            }
            if self.é_a_vez_de(self.peça_em(vizinho).unwrap()) {
                continue;
            }

            let pulo = coord.distancia(vizinho).vezes(2);
            if self.casa_em(coord + pulo).é_vazia() {
                comiveis.push(Jogada::Comer(vizinho, coord + pulo));
            }
        }

        if comiveis.is_empty() {
            andaveis
        } else {
            comiveis
        }
    }

    fn possiveis_jogadas_rainha(&self, coord: Coord) -> Vec<Jogada> {
        let mut movimentos = vec![];
        let mut comidas = vec![];
        for dir in [c(1, 1), c(-1, -1), c(1, -1), c(-1, 1)] {
            let mut atual = coord + dir;
            while atual.é_valida() && self.casa_em(atual).é_vazia() {
                movimentos.push(Jogada::Mover(atual));
                atual = atual + dir;
            }
            if atual.é_valida() {
                let mut pulo = (atual) + (coord.distancia(atual).normal());
                if atual.é_valida() && pulo.é_valida() && self.casa_em(pulo).é_vazia() {
                    let mut possiveis_pulos = vec![];
                    while pulo.é_valida() && self.casa_em(pulo).é_vazia() {
                        possiveis_pulos.push(pulo);
                        pulo = (pulo + dir);
                    }
                    comidas.push(Jogada::RComer(atual, possiveis_pulos));
                }
            }
        }
        if comidas.is_empty() {
            movimentos
        } else {
            comidas
        }
    }

    fn peça_em(&self, coord: Coord) -> Option<Peça> {
        self.casa_em(coord).get_peça()
    }

    fn casa_em(&self, coord: Coord) -> Casa {
        self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    fn é_a_vez_de(&self, peça: Peça) -> bool {
        if peça.é_branca() && self.vez == Vez::Preta {
            return false;
        }
        if peça.é_preta() && self.vez == Vez::Branca {
            return false;
        }
        true
    }
}

mod test {
    use super::*;
    #[test]
    fn testar_rainhas() {
        const TABULEIRO: [[char; 8]; 8] = TABULEIRO_INICIAL;
        let mut jogo = Jogo::new(TABULEIRO);

        let coord = c(0, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(1, 4)),
                Jogada::Mover(c(2, 3)),
                Jogada::Mover(c(3, 2)),
                Jogada::Mover(c(4, 1)),
                Jogada::Mover(c(5, 0))
            ]
        );

        let coord = c(0, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(1, 4)),
                Jogada::Mover(c(2, 3)),
                Jogada::Mover(c(3, 2)),
                Jogada::Mover(c(4, 1)),
                Jogada::Mover(c(5, 0))
            ]
        );

        let coord = c(2, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(1, 4)),
                Jogada::Mover(c(0, 3)),
                Jogada::Mover(c(3, 4)),
                Jogada::Mover(c(4, 3)),
                Jogada::Mover(c(5, 2)),
                Jogada::Mover(c(6, 1)),
                Jogada::Mover(c(7, 0))
            ]
        );

        let coord = c(3, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![Jogada::RComer(c(5, 3), vec![c(6, 2), c(7, 1)])]
        );

        let coord = c(6, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(7, 6)),
                Jogada::Mover(c(5, 4)),
                Jogada::Mover(c(4, 3)),
                Jogada::Mover(c(3, 2)),
                Jogada::Mover(c(2, 1)),
                Jogada::Mover(c(1, 0)),
                Jogada::Mover(c(7, 4))
            ]
        );

        let coord = c(7, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![Jogada::RComer(c(5, 3), vec![c(4, 2)])]
        );
    }
}
