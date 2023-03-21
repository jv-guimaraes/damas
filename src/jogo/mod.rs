use itertools::Itertools;
use std::fmt::Display;

mod jogador;
mod casa;
mod pedra;
mod jogada;
pub mod jogada_resultado;

use super::coord::{c, Coord};
use jogada::Jogada;
use jogador::Jogador;
use jogada_resultado::JogadaResultado;
use casa::Casa;
use pedra::Pedra;

const TABULEIRO_INICIAL_CHARS: [[char; 8]; 8] = [
    ['p','.','.','.','.','.','.','.'],
    ['.','.','.','.','.','.','.','.'],
    ['.','.','.','.','.','.','.','.'],
    ['.','.','.','.','.','.','p','.'],
    ['.','.','.','.','.','.','.','.'],
    ['.','.','.','.','p','.','.','.'],
    ['.','.','.','b','.','.','p','.'],
    ['b','.','.','.','.','.','.','b'],
];

#[derive(Debug, Clone)]
pub struct Jogo {
    tabuleiro: [[Casa; 8]; 8],
    pub vez: Jogador,
}

impl Default for Jogo {
    fn default() -> Self {
        // Construir tabuleiro inicial
        let mut tabuleiro = [[Casa::Vazia; 8]; 8];
        for y in 0..tabuleiro.len() {
            for x in 0..tabuleiro.len() {
                match TABULEIRO_INICIAL_CHARS[y][x] {
                    'p' => tabuleiro[y][x] = Casa::Ocupada(Pedra::Preta),
                    'b' => tabuleiro[y][x] = Casa::Ocupada(Pedra::Branca),
                    'P' => tabuleiro[y][x] = Casa::Ocupada(Pedra::DamaPreta),
                    'B' => tabuleiro[y][x] = Casa::Ocupada(Pedra::DamaBranca),
                    '.' => (),
                    c => panic!("{c} não é uma peça válida!"),
                }
            }
        }
        // Começar o jogo com a peça branca
        Jogo {
            tabuleiro,
            vez: Jogador::Branco,
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
                        Pedra::Branca => buffer.push_str(" x "),
                        Pedra::Preta => buffer.push_str(" o "),
                        Pedra::DamaBranca => buffer.push_str(" X "),
                        Pedra::DamaPreta => buffer.push_str(" O "),
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
                    'p' => tab[y][x] = Casa::Ocupada(Pedra::Preta),
                    'b' => tab[y][x] = Casa::Ocupada(Pedra::Branca),
                    'P' => tab[y][x] = Casa::Ocupada(Pedra::DamaPreta),
                    'B' => tab[y][x] = Casa::Ocupada(Pedra::DamaBranca),
                    '.' => (),
                    c => panic!("{c} não é uma peça válida!"),
                }
            }
        }
        // Começar o jogo com a peça branca
        Jogo {
            tabuleiro: tab,
            vez: Jogador::Branco,
        }
    }

    pub fn mover(&mut self, origem: Coord, destino: Coord) -> JogadaResultado {
        // // Filtrar todas as jogadas para encontrar a jogada que tem origem e destino correspondente ao input
        // let jogada = self
        //     .todas_possiveis_jogadas()
        //     .into_iter()
        //     .filter(|jogada| jogada.tem(origem, destino))
        //     .collect_vec();

        // // Caso não encontre uma jogada que move 'origem' para 'destino'
        // if jogada.is_empty() {
        //     return JogadaResultado::Falha;
        // }

        // // Realizar a jogada que foi encontrada
        // let mut capturou = false; // Flag para saber se uma peça foi capturada
        // assert!(jogada.len() == 1);
        // let jogada = jogada.into_iter().next().unwrap();
        // match jogada {
        //     Jogada::Mover(_, _) => self.mover_sem_checar(origem, destino),
        //     Jogada::Capturar(_, comida, _) | Jogada::DCapturar(_, comida, _) => {
        //         self.mover_sem_checar(origem, destino);
        //         *self.casa_mut(comida) = Casa::Vazia;
        //         capturou = true;
        //     }
        // }

        // // Transformar em dama caso necessário
        // if destino.y == 7 || destino.y == 0 {
        //     *self.casa_mut(destino) = Casa::Ocupada(self.peça(destino).unwrap().dama())
        // }

        // // Checar se acabou o jogo
        // if capturou && self.acabou() {
        //     return JogadaResultado::FimDoJogo(self.vez);
        // }

        // // Checar se da ou não para capturar em sequencia. Passa o turno caso não de
        // if capturou && self.possiveis_jogadas(destino).iter().any(|j| j.é_capturar()) {
        //     JogadaResultado::Sequencia
        // } else {
        //     self.passar_turno();
        //     JogadaResultado::Sucesso
        // }
        todo!()
    }

    fn executar_jogada(&mut self, jogada: Jogada) {
        match jogada {
            Jogada::Mover(origem, destino) => self.mover_sem_checar(origem, destino),
            Jogada::Capturar(origem, captura, destino) => {
                self.mover_sem_checar(origem, destino);
                *self.casa_mut(captura) = Casa::Vazia;
            },
        }
    }

    fn mover_sem_checar(&mut self, origem: Coord, destino: Coord) {
        *self.casa_mut(destino) = self.casa(origem);
        *self.casa_mut(origem) = Casa::Vazia;
    }

    pub fn calcular_capturas(&self, origem: Coord) -> Vec<Vec<Jogada>> {
        let mut stack: Vec<Jogada> = vec![];
        let mut sequencias: Vec<Vec<Jogada>> = vec![];
        let peça = self.peça(origem);
        if peça.is_none() { return vec![vec![]];}
        let mut clone_sem_origem = self.clone();
        *clone_sem_origem.casa_mut(origem) = Casa::Vazia;
        clone_sem_origem.calcular_capturas_recursivamente(origem, &mut stack, &mut sequencias, peça.unwrap());
        // Filtrar apenas as maiores cadeias
        if sequencias.is_empty() { return vec![vec![]] }
        let maior_cadeia = sequencias.iter().max_by_key(|x| x.len()).unwrap().len();
        sequencias.into_iter().filter(|x| x.len() == maior_cadeia).collect()
        // Sem filtrar
        // sequencias
    }

    fn calcular_capturas_recursivamente(&self, origem: Coord, stack: &mut Vec<Jogada>, sequencias: &mut Vec<Vec<Jogada>>, peça: Pedra) {
        // println!("origem: {:?}", origem);
        // println!("stack: {:?}", stack);
        // println!("sequencias: {:?}", sequencias.len());
        // println!("--------------------------------------------------------------------");
        'a: for captura in self.capturas_imediatas(origem, peça) {
            // println!("{:?}", captura);
            // if !stack.is_empty() && stack.last().unwrap().origem() == captura.destino() {
            //     continue;
            // }
            // if stack.contains(&captura) { continue; }
            for captura_anterior in stack.iter() {
                if captura_anterior.captura() == captura.captura() {
                    continue 'a;
                }
            }
            stack.push(captura);
            // println!("{:?}", stack);
            self.calcular_capturas_recursivamente(captura.destino(), stack, sequencias, peça)
        }
        if !stack.is_empty() {
            sequencias.push(stack.clone());
        }
        stack.pop();
        // println!("{:?}", stack);
    }

    fn capturas_imediatas(&self, origem: Coord, peça: Pedra) -> Vec<Jogada> {
        match peça {
            Pedra::Branca | Pedra::Preta => self.capturas_imediatas_peão(origem),
            Pedra::DamaBranca | Pedra::DamaPreta => self.capturas_imediatas_dama(origem),
        }
    }

    fn capturas_imediatas_dama(&self, origem: Coord) -> Vec<Jogada> {
        let mut capturas = vec![];
        for dir in [c(1, 1), c(-1, -1), c(1, -1), c(-1, 1)] {
            let mut atual = origem + dir;
            while atual.é_valida() && self.casa(atual).é_vazia() {
                atual = atual + dir;
            }
            if atual.é_valida() && !self.é_a_vez_de(self.peça(atual).unwrap()) {
                let mut pulo = (atual) + (origem.distancia(atual).normal());
                if atual.é_valida() && pulo.é_valida() && self.casa(pulo).é_vazia() {
                    while pulo.é_valida() && self.casa(pulo).é_vazia() {
                        capturas.push(Jogada::Capturar(origem, atual, pulo));
                        pulo = pulo + dir;
                    }
                }
            }
        }
        capturas
    }

    fn capturas_imediatas_peão(&self, origem: Coord) -> Vec<Jogada> {
        let mut capturas = vec![];
        for vizinho in origem.diagonais_comiveis() {
            if let Casa::Ocupada(peça) = self.casa(vizinho) {
                if self.é_a_vez_de(peça) { continue; }
                let destino = vizinho + origem.distancia(vizinho);
                if self.casa(destino).é_vazia() {
                    capturas.push(Jogada::Capturar(origem, vizinho, destino));
                }
            }
        }
        capturas
    }

    pub fn calcular_movimentos(&self, origem: Coord) -> Vec<Jogada> {
        match self.peça(origem).unwrap() {
            Pedra::Branca | Pedra::Preta => self.movimentos_peão(origem),
            Pedra::DamaBranca | Pedra::DamaPreta => self.movimentos_dama(origem),
        }
    }

    pub fn movimentos_dama(&self, origem: Coord) -> Vec<Jogada> {
        let mut movimentos = vec![];
        for dir in [c(1, 1), c(-1, -1), c(1, -1), c(-1, 1)] {
            let mut atual = origem + dir;
            while atual.é_valida() && self.casa(atual).é_vazia() {
                movimentos.push(Jogada::Mover(origem, atual));
                atual = atual + dir;
            }
        }
        movimentos
    }

    pub fn movimentos_peão(&self, origem: Coord) -> Vec<Jogada> {
        let diagonais = match self.peça(origem).unwrap() {
            Pedra::Branca => origem.diagonais_frente(),
            Pedra::Preta => origem.diagonais_atrás(),
            _ => panic!(),
        };
        diagonais
            .into_iter()
            .filter(|c| self.casa(*c).é_vazia())
            .map(|c| Jogada::Mover(origem, c))
            .collect()
    }

    fn peça(&self, coord: Coord) -> Option<Pedra> {
        self.casa(coord).peça()
    }

    fn casa(&self, coord: Coord) -> Casa {
        self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    fn casa_mut(&mut self, coord: Coord) -> &mut Casa {
        &mut self.tabuleiro[coord.y as usize][coord.x as usize]
    }

    fn é_a_vez_de(&self, peça: Pedra) -> bool {
        if peça.é_branca() && self.vez == Jogador::Preto {
            return false;
        }
        if peça.é_preta() && self.vez == Jogador::Branco {
            return false;
        }
        true
    }

    fn passar_turno(&mut self) {
        self.vez = match self.vez {
            Jogador::Branco => Jogador::Preto,
            Jogador::Preto => Jogador::Branco,
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

    pub fn todas_capturas_possiveis(&self) -> Vec<Vec<Jogada>> {
        let mut capturas = vec![];
        let peças = self.peças_da_cor_atual();
        for peça in peças {
            capturas.append(&mut self.calcular_capturas(peça));
        }
        let capturas = capturas.into_iter().filter(|x| !x.is_empty()).collect_vec();
        if capturas.is_empty() { return vec![] }
        let maior_len = capturas.iter().max_by_key(|x| x.len()).unwrap().len();
        capturas.into_iter().filter(|x| x.len() == maior_len).collect()
    }

    pub fn todos_movimentos_possiveis(&self) -> Vec<Vec<Jogada>> {
        let mut movimentos = vec![];
        let peças = self.peças_da_cor_atual();
        for peça in peças {
            movimentos.push(self.calcular_movimentos(peça));
        }
        movimentos.into_iter().filter(|x| !x.is_empty()).collect()
    }

    pub fn todas_jogadas_possiveis(&self) -> Vec<Vec<Jogada>> {
        let capturas = self.todas_capturas_possiveis();
        if capturas.is_empty() {
            self.todos_movimentos_possiveis()
        } else {
            capturas
        }
    }
}

mod test {
    use super::*;

}
