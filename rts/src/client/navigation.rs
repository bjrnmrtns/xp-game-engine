use std::collections::VecDeque;

pub struct Neighbours(Vec<Cell>);

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

pub struct FlowField {
    cells: Vec<u32>,
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
            cells: vec![std::u32::MAX; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, cell: &Cell) -> u32 {
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.cells[self.height * cell.y + cell.x]
    }

    pub fn set(&mut self, cell: &Cell, value: u32) {
        assert!(cell.x < self.width);
        assert!(cell.y < self.height);
        self.cells[self.height * cell.y + cell.x] = value;
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

    pub fn with_destination(mut self, cell: Cell) -> Self {
        let mut open = VecDeque::new();
        self.set(&cell, 0);
        open.push_back(cell);
        while !open.is_empty() {
            let cell = open.pop_front().unwrap();
        }
        self
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:010} ", self.get(&(x, y).into()));
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
        let flow_field = FlowField::new(10, 10).with_destination(Cell::new(4, 4));
        flow_field.print();
    }
}
