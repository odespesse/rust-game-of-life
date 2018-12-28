mod cell;
use crate::cell::Cell;

mod world;
use crate::world::World;

pub struct Automaton {}
impl Automaton {
    pub fn new() -> Automaton{
        Automaton {}
    }

    pub fn create_next_generation(&self, world: World) -> World {
        let next_grid = world.grid.iter().enumerate().map(|(row_index, _)| {
            let row = &world.grid[row_index];
            row.iter().enumerate().map(|(column_index, _)| {
                let count = self.count_alive_moore_neighbourhood(&world, row_index, column_index);
                let current_cell = &row[column_index];
                self.create_new_cell(current_cell, count)
            }).collect()
        }).collect();
        World::new(world.generation + 1, next_grid)
    }

    fn count_alive_moore_neighbourhood(&self, world: &World, row_index: usize, column_index: usize) -> u8 {
        let mut alive_cells_count = 0;

        if row_index.checked_sub(1).is_some() && column_index.checked_sub(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index - 1, column_index - 1) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if row_index.checked_sub(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index - 1, column_index) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if row_index.checked_sub(1).is_some() && column_index.checked_add(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index - 1, column_index + 1) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if column_index.checked_sub(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index, column_index - 1) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if column_index.checked_add(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index, column_index + 1) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if row_index.checked_add(1).is_some() && column_index.checked_sub(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index + 1, column_index - 1) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if row_index.checked_add(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index + 1, column_index) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        if row_index.checked_add(1).is_some() && column_index.checked_add(1).is_some() {
            if let Some(cell) = world.get_cell_at(row_index + 1, column_index + 1) {
                if cell.is_alive() {
                    alive_cells_count += 1;
                }
            }
        }
        alive_cells_count
    }

    fn create_new_cell(&self, current_cell: &Cell, alive_neighbourhood_count: u8) -> Cell {
        match alive_neighbourhood_count {
            2 if current_cell.is_alive() => {
                Cell::Alive
            },
            3 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_grid_equals(expected: &Vec<Vec<Cell>>, actual: &Vec<Vec<Cell>>) {
        assert_eq!(expected.len(), actual.len());
        let columns_pairs = expected.iter().zip(actual.iter());
        for columns_pair in columns_pairs {
            let rows_pairs = columns_pair.0.iter().zip(columns_pair.1.iter());
            for row_pair in rows_pairs {
                assert_eq!(row_pair.0, row_pair.1);
            }
        }
    }

    #[test]
    fn stable_single_cell() {
        let seed = vec![
            vec![Cell::Dead],
        ];
        let world = World::new(10, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(11, next_world.generation);
        let expected = vec![
            vec![Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn stable_block() {
        let seed = vec![
            vec![Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Alive],
        ];
        let world = World::new(2, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(3, next_world.generation);
        let expected = vec![
            vec![Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Alive],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn stable_beehive() {
        let seed = vec![
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead],
        ];
        let world = World::new(0, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(1, next_world.generation);
        let expected = vec![
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn stable_loaf() {
        let seed = vec![
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Dead,  Cell::Dead, Cell::Alive, Cell::Dead],
        ];
        let world = World::new(0, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(1, next_world.generation);
        let expected = vec![
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Dead, Cell::Alive],
            vec![Cell::Dead,  Cell::Dead, Cell::Alive, Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn stable_boat() {
        let seed = vec![
            vec![Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Dead],
        ];
        let world = World::new(0, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(1, next_world.generation);
        let expected = vec![
            vec![Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn oscillator_blinker() {
        let seed = vec![
            vec![Cell::Dead, Cell::Dead, Cell::Dead,  Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead,  Cell::Dead, Cell::Dead],
        ];
        let world = World::new(7, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(8, next_world.generation);
        let expected = vec![
            vec![Cell::Dead, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Dead, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Dead, Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn oscillator_toad() {
        let seed = vec![
            vec![Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Dead,  Cell::Dead, Cell::Dead,  Cell::Dead],
        ];
        let world = World::new(3, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(4, next_world.generation);
        let expected = vec![
            vec![Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Alive, Cell::Dead,  Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);

        let next_world = automaton.create_next_generation(next_world);
        assert_eq!(5, next_world.generation);
        let expected = vec![
            vec![Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Dead,  Cell::Alive, Cell::Alive, Cell::Alive],
            vec![Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Dead,  Cell::Dead, Cell::Dead,  Cell::Dead],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }

    #[test]
    fn oscillator_beacon() {
        let seed = vec![
            vec![Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead],
            vec![Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive],
            vec![Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive],
        ];
        let world = World::new(3, seed);
        let automaton = Automaton::new();
        let next_world = automaton.create_next_generation(world);

        assert_eq!(4, next_world.generation);
        let expected = vec![
            vec![Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead],
            vec![Cell::Alive, Cell::Dead,  Cell::Dead,  Cell::Dead],
            vec![Cell::Dead,  Cell::Dead,  Cell::Dead,  Cell::Alive],
            vec![Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive],
        ];
        assert_grid_equals(&expected, &next_world.grid);

        let next_world = automaton.create_next_generation(next_world);
        assert_eq!(5, next_world.generation);
        let expected = vec![
            vec![Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead,  Cell::Dead],
            vec![Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive],
            vec![Cell::Dead,  Cell::Dead,  Cell::Alive, Cell::Alive],
        ];
        assert_grid_equals(&expected, &next_world.grid);
    }
}
