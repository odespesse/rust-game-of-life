use crate::cell::Cell;

pub struct World {
    pub generation: u32,
    pub grid: Vec<Vec<Cell>>
}

impl World {
    pub fn new(generation: u32, grid: Vec<Vec<Cell>>) -> World {
        World {
            generation,
            grid
        }
    }

    pub fn get_cell_at(&self, row_index: usize, column_index: usize) -> Option<&Cell> {
        let column = match self.grid.get(row_index) {
            Some(column) => column,
            None => return None
        };
        column.get(column_index)
    }
}
