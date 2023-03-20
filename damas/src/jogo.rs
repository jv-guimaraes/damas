use itertools::Itertools;
use std::fmt::Display;

use crate::coord::{c, Coord};

const TABULEIRO_INICIAL_CHARS: [[char; 8]; 8] = [
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', 'p', '.', 'p', '.', '.'],
    ['.', '.', 'b', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', 'p', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
    ['.', '.', '.', '.', '.', '.', '.', '.'],
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vez {
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

    fn rainha(self) -> Self {
        if self.é_branca() {
            Peça::RainhaBranca
        } else {
            Peça::RainhaPreta
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Jogada {
    Mover(Coord, Coord),              // (origem, destino)
    Capturar(Coord, Coord, Coord),       // (origem, comida, destino)
    RCapturar(Coord, Coord, Vec<Coord>), // (origem, comida, destinos)
}

impl Jogada {
    fn é_capturar(&self) -> bool {
        matches!(self, Jogada::Capturar(_, _, _) | Jogada::RCapturar(_, _, _))
    }

    fn tem(&self, origem: Coord, destino: Coord) -> bool {
        match self {
            Jogada::Mover(o, d) | Jogada::Capturar(o, _, d) => *o == origem && *d == destino,
            Jogada::RCapturar(o, _, d) => *o == origem && d.contains(&destino),
        }
    }
}

#[derive(Debug)]
pub struct Jogo {
    tabuleiro: [[Casa; 8]; 8],
    pub vez: Vez,
}

impl Default for Jogo {
    fn default() -> Self {
        // Construir tabuleiro inicial
        let mut tabuleiro = [[Casa::Vazia; 8]; 8];
        for y in 0..tabuleiro.len() {
            for x in 0..tabuleiro.len() {
                match TABULEIRO_INICIAL_CHARS[y][x] {
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

#[derive(Debug)]
pub enum JogadaResultado {
    Falha,          // Jogada invalida. Não passa o turno nem mexe no tabuleiro
    Sucesso,        // Jogada válida e passa o turno. Não tem mais possiveis captura
    Sequencia,      // Jogada válida e não passa o turno. Ainda tem pessas para capturar
    FimDoJogo(Vez), // Jogada válida e fim do jogo. Retorna a vez de quem ganhou
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

    pub fn mover(&mut self, origem: Coord, destino: Coord) -> JogadaResultado {
        // Filtrar todas as jogadas para encontrar a jogada que tem origem e destino correspondente ao input
        let jogada = self
            .todas_possiveis_jogadas()
            .into_iter()
            .filter(|jogada| jogada.tem(origem, destino))
            .collect_vec();

        // Caso não encontre uma jogada que mexe 'origem' para 'destino'
        if jogada.is_empty() {
            return JogadaResultado::Falha;
        }

        // Realizar a jogada que foi encontrada
        let mut capturou = false; // Flag para saber se uma peça foi capturada
        assert!(jogada.len() == 1);
        let jogada = jogada.into_iter().next().unwrap();
        match jogada {
            Jogada::Mover(_, _) => self.mover_sem_checar(origem, destino),
            Jogada::Capturar(_, comida, _) | Jogada::RCapturar(_, comida, _) => {
                self.mover_sem_checar(origem, destino);
                *self.casa_mut(comida) = Casa::Vazia;
                capturou = true;
            }
        }

        // Transformar em rainha caso necessário
        if destino.y == 7 || destino.y == 0 {
            *self.casa_mut(destino) = Casa::Ocupada(self.peça(destino).unwrap().rainha())
        }

        // Checar se acabou o jogo
        if capturou && self.acabou() {
            return JogadaResultado::FimDoJogo(self.vez);
        }

        // Checar se da ou não para capturar em sequencia. Passa o turno caso não de
        if capturou && self.possiveis_jogadas(destino).iter().any(|j| j.é_capturar()) {
            JogadaResultado::Sequencia
        } else {
            self.passar_turno();
            JogadaResultado::Sucesso
        }
    }

    fn mover_sem_checar(&mut self, origem: Coord, destino: Coord) {
        *self.casa_mut(destino) = self.casa(origem);
        *self.casa_mut(origem) = Casa::Vazia;
    }

    pub fn possiveis_jogadas(&self, coord: Coord) -> Vec<Jogada> {
        // Caso a casa dada por coord esteja vazia
        if let Casa::Vazia = self.casa(coord) {
            return vec![];
        }

        // Caso não seja a vez da peça que está em coord
        let peça_selecionada = self.peça(coord).unwrap();
        if !self.é_a_vez_de(peça_selecionada) {
            return vec![];
        }

        match peça_selecionada {
            Peça::Branca | Peça::Preta => self.possiveis_jogadas_peão(coord),
            Peça::RainhaBranca | Peça::RainhaPreta => self.possiveis_jogadas_rainha(coord),
        }
    }

    fn possiveis_jogadas_peão(&self, coord: Coord) -> Vec<Jogada> {
        let peça_selecionada = self.peça(coord).unwrap();

        // Computar casas andaveis
        let casas: Vec<Coord> = match peça_selecionada {
            Peça::Branca => coord
                .diagonais_frente()
                .into_iter()
                .filter(|c| self.casa(*c).é_vazia())
                .collect(),
            Peça::Preta => coord
                .diagonais_atrás()
                .into_iter()
                .filter(|c| self.casa(*c).é_vazia())
                .collect(),
            _ => vec![],
        };
        // Transformar lista de coordenadas em Jogadas
        let andaveis = casas.into_iter().map(|c| Jogada::Mover(coord, c)).collect();

        // Computar casas comíveis
        let mut comiveis = vec![];
        for vizinho in coord.diagonais_comiveis() {
            // Se o vizinho for vazio n tem o que capturar
            if let Casa::Vazia = self.casa(vizinho) {
                continue;
            }
            // Se o vizinho for da mesma cor não pode come-lo
            if self.é_a_vez_de(self.peça(vizinho).unwrap()) {
                continue;
            }
            // Calcular aonde pular para capturar a peça
            let pulo = coord.distancia(vizinho).vezes(2);
            // Se a casa do pulo for vazia então da pra capturar o vizinho
            if self.casa(coord + pulo).é_vazia() {
                comiveis.push(Jogada::Capturar(coord, vizinho, coord + pulo));
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
            while atual.é_valida() && self.casa(atual).é_vazia() {
                movimentos.push(Jogada::Mover(coord, atual));
                atual = atual + dir;
            }
            if atual.é_valida() && !self.é_a_vez_de(self.peça(atual).unwrap()) {
                let mut pulo = (atual) + (coord.distancia(atual).normal());
                if atual.é_valida() && pulo.é_valida() && self.casa(pulo).é_vazia() {
                    let mut possiveis_pulos = vec![];
                    while pulo.é_valida() && self.casa(pulo).é_vazia() {
                        possiveis_pulos.push(pulo);
                        pulo = pulo + dir;
                    }
                    comidas.push(Jogada::RCapturar(coord, atual, possiveis_pulos));
                }
            }
        }
        if comidas.is_empty() {
            movimentos
        } else {
            comidas
        }
    }

    fn peça(&self, coord: Coord) -> Option<Peça> {
        self.casa(coord).get_peça()
    }

    fn casa(&self, coord: Coord) -> Casa {
        self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    fn casa_mut(&mut self, coord: Coord) -> &mut Casa {
        &mut self.tabuleiro[coord.y as usize][coord.x as usize]
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

    fn passar_turno(&mut self) {
        self.vez = match self.vez {
            Vez::Branca => Vez::Preta,
            Vez::Preta => Vez::Branca,
        }
    }

    pub fn todas_possiveis_jogadas(&self) -> Vec<Jogada> {
        // Todas as coordenadas com peças da cor da vez atual
        let peças_cor_atual = self.peças_da_cor_atual();
        let jogadas = peças_cor_atual
            .into_iter()
            .flat_map(|c| self.possiveis_jogadas(c))
            .collect_vec();

        // Se tiver pelo menos uma jogada do tipo 'capturar', retorna só elas
        if jogadas.iter().any(|j| j.é_capturar()) {
            jogadas.into_iter().filter(|j| j.é_capturar()).collect()
        } else {
            jogadas
        }
    }

    fn peças_da_cor_atual(&self) -> Vec<Coord> {
        let mut peças = vec![];
        for y in 0..8 {
            for x in 0..8 {
                if let Casa::Ocupada(peça) = self.tabuleiro[y][x] {
                    if self.é_a_vez_de(peça) {
                        peças.push(c(x as i32, y as i32));
                    }
                }
            }
        }
        peças
    }

    fn acabou(&self) -> bool {
        let mut count = 0;
        for y in 0..8 {
            for x in 0..8 {
                if let Casa::Ocupada(peça) = self.tabuleiro[y][x] {
                    if !self.é_a_vez_de(peça) {
                        // encontrou uma peça do inimigo, logo, o jogo não acabou
                        return false;
                    }
                }
            }
        }
        true
    }
}

mod test {
    use super::*;
    #[test]
    fn testar_rainhas() {
        const TABULEIRO: [[char; 8]; 8] = [
            ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
            ['.', 'p', '.', 'p', '.', 'p', '.', '.'],
            ['P', '.', 'P', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', 'P', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['B', '.', 'B', 'B', '.', '.', 'B', 'B'],
            ['.', 'b', '.', 'b', '.', 'b', '.', '.'],
            ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
        ];
        let jogo = Jogo::new(TABULEIRO);

        let coord = c(0, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(0, 5), c(1, 4)),
                Jogada::Mover(c(0, 5), c(2, 3)),
                Jogada::Mover(c(0, 5), c(3, 2)),
                Jogada::Mover(c(0, 5), c(4, 1)),
                Jogada::Mover(c(0, 5), c(5, 0))
            ]
        );

        let coord = c(0, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(0, 5), c(1, 4)),
                Jogada::Mover(c(0, 5), c(2, 3)),
                Jogada::Mover(c(0, 5), c(3, 2)),
                Jogada::Mover(c(0, 5), c(4, 1)),
                Jogada::Mover(c(0, 5), c(5, 0))
            ]
        );

        let coord = c(2, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(2, 5), c(1, 4)),
                Jogada::Mover(c(2, 5), c(0, 3)),
                Jogada::Mover(c(2, 5), c(3, 4)),
                Jogada::Mover(c(2, 5), c(4, 3)),
                Jogada::Mover(c(2, 5), c(5, 2)),
                Jogada::Mover(c(2, 5), c(6, 1)),
                Jogada::Mover(c(2, 5), c(7, 0))
            ]
        );

        let coord = c(3, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![Jogada::RCapturar(c(3, 5), c(5, 3), vec![c(6, 2), c(7, 1)])]
        );

        let coord = c(6, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![
                Jogada::Mover(c(6, 5), c(7, 6)),
                Jogada::Mover(c(6, 5), c(5, 4)),
                Jogada::Mover(c(6, 5), c(4, 3)),
                Jogada::Mover(c(6, 5), c(3, 2)),
                Jogada::Mover(c(6, 5), c(2, 1)),
                Jogada::Mover(c(6, 5), c(1, 0)),
                Jogada::Mover(c(6, 5), c(7, 4))
            ]
        );

        let coord = c(7, 5);
        assert_eq!(
            jogo.possiveis_jogadas(coord),
            vec![Jogada::RCapturar(c(7, 5), c(5, 3), vec![c(4, 2)])]
        );
    }

    #[test]
    fn testar_todas_possiveis_jogadas() {
        const TABULEIRO: [[char; 8]; 8] = [
            ['p', '.', 'p', '.', 'p', '.', 'p', '.'],
            ['.', 'p', '.', 'p', '.', 'p', '.', 'p'],
            ['.', '.', 'p', '.', '.', '.', '.', '.'],
            ['.', 'p', '.', 'p', '.', '.', '.', 'p'],
            ['.', '.', 'b', '.', '.', '.', 'b', '.'],
            ['b', '.', '.', '.', 'b', '.', '.', '.'],
            ['.', 'b', '.', 'b', '.', 'b', '.', 'b'],
            ['b', '.', 'b', '.', 'b', '.', 'b', '.'],
        ];
        let jogo = Jogo::new(TABULEIRO);
        assert_eq!(
            jogo.todas_possiveis_jogadas(),
            vec![
                Jogada::Capturar(c(2, 4), c(1, 3), c(0, 2)),
                Jogada::Capturar(c(2, 4), c(3, 3), c(4, 2))
            ]
        );

        const TABULEIRO1: [[char; 8]; 8] = [
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', 'p', '.', 'p', '.', '.', '.'],
            ['.', '.', '.', 'B', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', 'p', '.', '.', '.', 'p', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
            ['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let jogo = Jogo::new(TABULEIRO1);
        assert_eq!(
            jogo.todas_possiveis_jogadas(),
            vec![
                Jogada::RCapturar(c(3, 3), c(5, 5), vec![c(6, 6), c(7, 7)]),
                Jogada::RCapturar(c(3, 3), c(2, 2), vec![c(1, 1), c(0, 0)]),
                Jogada::RCapturar(c(3, 3), c(4, 2), vec![c(5, 1), c(6, 0)]),
                Jogada::RCapturar(c(3, 3), c(1, 5), vec![c(0, 6)])
            ]
        );
    }
}
