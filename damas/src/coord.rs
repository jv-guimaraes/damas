use std::fmt::{Display, Debug};

use crate::Casa;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn diagonais_da_frente(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        if self.y == 0 { return diagonais };
        
        let y = self.y - 1;
        if self.x > 0 {
            diagonais.push(c(self.x - 1, y));
        }
        if self.x < 7 {
            diagonais.push(c(self.x + 1, y));
        } 
        diagonais
    }

    pub fn diagonais_de_trás(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        if self.y == 7 { return diagonais };
        
        let y = self.y + 1;
        if self.x > 0 {
            diagonais.push(c(self.x - 1, y));
        }
        if self.x < 7 {
            diagonais.push(c(self.x + 1, y));
        } 
        diagonais
    }

    pub fn diagonais_da_rainha(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();

        for (i, j) in [(1, 1), (-1i32, -1), (1, -1), (-1, 1)] {
            let (mut x, mut y) = (self.x as i32, self.y as i32);
            loop {
                if x != self.x as i32 && y != self.y as i32 {
                    diagonais.push(c(x as usize, y as usize));
                }
                x += i; y += j;
                if x == 8 || x == -1 { break; }
                if y == 8 || y == -1 { break; }
            }
        }
        diagonais
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn c(x: usize, y: usize) -> Coord {
    Coord { x, y }
}

#[test]
fn testar_diagonais() {
    let coord = c(2, 5);
    assert_eq!(coord.diagonais_da_frente(), vec![c(1, 4), c(3, 4)]);
    assert_eq!(coord.diagonais_de_trás(), vec![c(1, 6), c(3, 6)]);
    let coord = c(0, 0);
    assert_eq!(coord.diagonais_da_frente(), vec![]);
    assert_eq!(coord.diagonais_de_trás(), vec![c(1, 1)]);
    let coord = c(7, 7);
    assert_eq!(coord.diagonais_da_frente(), vec![c(6, 6)]);
    assert_eq!(coord.diagonais_de_trás(), vec![]);
    let coord = c(7, 4);
    assert_eq!(coord.diagonais_da_frente(), vec![c(6, 3)]);
    assert_eq!(coord.diagonais_de_trás(), vec![c(6, 5)]);

    let coord = c(3, 3);
    assert_eq!(coord.diagonais_da_rainha(), vec![c(4,4), c(5,5), c(6,6),c(7,7), c(2,2), c(1,1),
                                                 c(0,0), c(4,2), c(5,1), c(6,0), c(2,4), c(1,5), c(0,6)])
}