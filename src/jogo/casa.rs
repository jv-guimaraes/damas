use super::Pedra;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Casa {
    Ocupada(Pedra),
    Vazia,
}

impl Casa {
    pub fn peça(self) -> Option<Pedra> {
        match self {
            Casa::Ocupada(peça) => Some(peça),
            Casa::Vazia => None,
        }
    }

    pub fn é_vazia(self) -> bool {
        match self {
            Casa::Ocupada(_) => false,
            Casa::Vazia => true,
        }
    }
}