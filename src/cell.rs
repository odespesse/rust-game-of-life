#[derive(PartialEq)]
#[derive(Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false
        }
    }
}
