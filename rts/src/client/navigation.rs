use bevy::prelude::Vec2;
use std::collections::VecDeque;

#[derive(PartialEq, Eq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct FlowField {
    values: Vec<u32>,
    flow: Vec<Option<IVec2>>,
    width: usize,
    height: usize,
}

impl Into<Cell> for (usize, usize) {
    fn into(self) -> Cell {
        Cell {
            x: self.0,
            y: self.1,
        }
    }
}

impl FlowField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![std::u32::MAX - 1; width * height],
            flow: vec![None; width * height],
            width,
            height,
        }
    }

    pub fn with_blocked_cell(mut self, cell: &Cell) -> Self {
        self.set(&cell, std::u32::MAX);
        self
    }

    pub fn with_blocked_cells(mut self, cells: &[Cell]) -> Self {
        for cell in cells {
            self.set(cell, std::u32::MAX);
        }
        self
    }

    pub fn get(&self, cell: &Cell) -> u32 {
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.values[self.height * cell.y + cell.x]
    }

    pub fn set(&mut self, cell: &Cell, value: u32) {
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.values[self.height * cell.y + cell.x] = value;
    }

    pub fn set_flow_cell(&mut self, cell: &Cell, direction: Option<IVec2>) {
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.flow[self.height * cell.y + cell.x] = direction;
    }

    pub fn get_flow_cell(&self, cell: &Cell) -> Option<IVec2> {
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.flow[self.height * cell.y + cell.x]
    }

    pub fn get_flow(&self, position: &Vec2) -> Option<IVec2> {
        let cell = self.position_to_cell(position);
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.get_flow_cell(&cell)
    }

    pub fn get_neighbours(&self, cell: &Cell) -> Vec<Cell> {
        let mut neighbours = Vec::new();
        if cell.x + 1 < self.width {
            neighbours.push((cell.x + 1, cell.y).into());
        }
        if cell.x > 0 {
            neighbours.push((cell.x - 1, cell.y).into());
        }
        if cell.y + 1 < self.height {
            neighbours.push((cell.x, cell.y + 1).into());
        }
        if cell.y > 0 {
            neighbours.push((cell.x, cell.y - 1).into());
        }
        neighbours
    }

    pub fn get_neighbours_cross(&self, cell: &Cell) -> Vec<Cell> {
        let mut neighbours = Vec::new();
        if cell.x + 1 < self.width && cell.y + 1 < self.height {
            neighbours.push((cell.x + 1, cell.y + 1).into());
        }
        if cell.x + 1 < self.width && cell.y > 0 {
            neighbours.push((cell.x + 1, cell.y - 1).into());
        }
        if cell.x > 0 && cell.y + 1 < self.height {
            neighbours.push((cell.x - 1, cell.y + 1).into());
        }
        if cell.x > 0 && cell.y > 0 {
            neighbours.push((cell.x - 1, cell.y - 1).into());
        }
        neighbours
    }

    fn position_to_cell(&self, position: &Vec2) -> Cell {
        (
            (position.x + self.width as f32 / 2.0) as usize,
            (position.y + self.height as f32 / 2.0) as usize,
        )
            .into()
    }

    pub fn set_destination(&mut self, destination: Vec2) {
        self.set_destination_cell(self.position_to_cell(&destination));
    }

    pub fn set_destination_cell(&mut self, cell: Cell) {
        let mut open = VecDeque::new();
        self.set(&cell, 0);
        open.push_back(cell);
        while !open.is_empty() {
            let cell = open.pop_front().unwrap();
            let value = self.get(&cell);
            for neighbour_cell in self.get_neighbours(&cell) {
                let n_value = self.get(&neighbour_cell);
                if n_value != std::u32::MAX && n_value > value + 100 {
                    self.set(&neighbour_cell, value + 100);
                    if !open.contains(&neighbour_cell) {
                        open.push_back(neighbour_cell);
                    }
                }
            }
            for neighbour_cell in self.get_neighbours_cross(&cell) {
                let n_value = self.get(&neighbour_cell);
                if n_value != std::u32::MAX && n_value > value + 141 {
                    self.set(&neighbour_cell, value + 141);
                    if !open.contains(&neighbour_cell) {
                        open.push_back(neighbour_cell);
                    }
                }
            }
        }
    }

    pub fn calculate_flow(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let current = Cell::new(x, y);
                let mut value = std::u32::MAX - 1;
                let mut direction = None;
                if self.get(&current) != std::u32::MAX {
                    for neighbour in self.get_neighbours(&current) {
                        let n_value = self.get(&neighbour);
                        if n_value < value {
                            value = n_value;
                            direction = Some(IVec2::new(
                                neighbour.x as i32 - current.x as i32,
                                neighbour.y as i32 - current.y as i32,
                            ));
                        }
                    }
                    for neighbour in self.get_neighbours_cross(&current) {
                        let n_value = self.get(&neighbour);
                        if n_value < value {
                            value = n_value;
                            direction = Some(IVec2::new(
                                neighbour.x as i32 - current.x as i32,
                                neighbour.y as i32 - current.y as i32,
                            ));
                        }
                    }
                }
                self.set_flow_cell(&current, direction);
            }
        }
    }

    pub fn print(&self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                print!("{:10} ", self.get(&(x, y).into()));
            }
            println!("");
        }
    }

    fn get_string_vector(v: &IVec2) -> String {
        let print_direction = match (v.x, v.y) {
            (-1, -1) => "⬋",
            (-1, 0) => "←",
            (-1, 1) => "⬉",
            (0, -1) => "↓",
            (0, 0) => " ",
            (0, 1) => "↑",
            (1, -1) => "⬊",
            (1, 0) => "→",
            (1, 1) => "⬈",
            _ => {
                assert!(false);
                " "
            }
        };
        print_direction.to_string()
    }

    pub fn print_flow(&self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if let Some(direction) = self.get_flow_cell(&(x, y).into()) {
                    print!(" {}", Self::get_string_vector(&direction));
                } else {
                    print!(" x");
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::navigation::{Cell, FlowField};
    #[test]
    fn get_neighbours_test() {
        let neighbours = FlowField::new(10, 10).get_neighbours(&(3, 3).into());
        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&Cell::new(3, 4)));
        assert!(neighbours.contains(&Cell::new(3, 2)));
        assert!(neighbours.contains(&Cell::new(2, 3)));
        assert!(neighbours.contains(&Cell::new(4, 3)));
    }

    #[test]
    fn get_neighbours_test_edge() {
        let neighbours = FlowField::new(10, 10).get_neighbours(&(0, 0).into());
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&Cell::new(1, 0)));
        assert!(neighbours.contains(&Cell::new(0, 1)));
    }

    #[test]
    fn get_neighbours_cross_test() {
        let neighbours = FlowField::new(10, 10).get_neighbours_cross(&(3, 3).into());
        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&Cell::new(4, 4)));
        assert!(neighbours.contains(&Cell::new(2, 4)));
        assert!(neighbours.contains(&Cell::new(4, 2)));
        assert!(neighbours.contains(&Cell::new(2, 2)));
    }

    #[test]
    fn get_neighbours_cross_edge_test() {
        let neighbours = FlowField::new(10, 10).get_neighbours_cross(&(10, 10).into());
        assert_eq!(neighbours.len(), 1);
        assert!(neighbours.contains(&Cell::new(9, 9)));
    }

    #[test]
    fn set_destination_test() {
        let mut flow_field = FlowField::new(10, 10);
        flow_field.set_destination_cell(Cell::new(4, 4));
        flow_field.print();
    }

    #[test]
    fn set_destination_with_one_blocked_test() {
        let mut flow_field = FlowField::new(10, 10).with_blocked_cell(&Cell::new(3, 3));
        flow_field.set_destination_cell(Cell::new(4, 4));
        assert!(flow_field.get(&Cell::new(3, 3)) == std::u32::MAX);
        flow_field.print();
    }

    #[test]
    fn print_flow() {
        let mut flow_field = FlowField::new(25, 25).with_blocked_cells(&[
            Cell::new(8, 8),
            Cell::new(7, 8),
            Cell::new(8, 7),
            Cell::new(7, 7),
        ]); //.with_blocked_cell(&Cell::new(3, 3));
        flow_field.set_destination_cell(Cell::new(10, 10));
        flow_field.calculate_flow();
        flow_field.print();
        flow_field.print_flow();
    }
}
