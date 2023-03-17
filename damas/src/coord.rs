#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn diagonais_da_frente(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        if self.y == 0 { return diagonais };
        
        let y = self.y - 1;
        if self.x > 0 {
            diagonais.push(Coord::new(self.x - 1, y));
        }
        if self.x < 7 {
            diagonais.push(Coord::new(self.x + 1, y));
        } 
        diagonais
    }

    fn diagonais_de_tras(self) -> Vec<Coord> {
        let mut diagonais: Vec<Coord> = Vec::new();
        if self.y == 7 { return diagonais };
        
        let y = self.y + 1;
        if self.x > 0 {
            diagonais.push(Coord::new(self.x - 1, y));
        }
        if self.x < 7 {
            diagonais.push(Coord::new(self.x + 1, y));
        } 
        diagonais
    }
}

fn c(x: usize, y: usize) -> Coord {
    Coord { x, y }
}

#[test]
fn testar_diagonais() {
    let coord = Coord::new(2, 5);
    assert_eq!(coord.diagonais_da_frente(), vec![c(1, 4), c(3, 4)]);
    assert_eq!(coord.diagonais_de_tras(), vec![c(1, 6), c(3, 6)]);
    let coord = Coord::new(0, 0);
    assert_eq!(coord.diagonais_da_frente(), vec![]);
    assert_eq!(coord.diagonais_de_tras(), vec![c(1, 1)]);
    let coord = Coord::new(7, 7);
    assert_eq!(coord.diagonais_da_frente(), vec![c(6, 6)]);
    assert_eq!(coord.diagonais_de_tras(), vec![]);
    let coord = Coord::new(7, 4);
    assert_eq!(coord.diagonais_da_frente(), vec![c(6, 3)]);
    assert_eq!(coord.diagonais_de_tras(), vec![c(6, 5)]);
}