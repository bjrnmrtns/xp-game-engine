pub struct FlowField {
    cells: Vec<u32>,
    width: usize,
    height: usize,
}

impl FlowField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![std::u32::MAX; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[self.height * y + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u32) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[self.height * y + x] = value;
    }

    pub fn set_destination(&mut self, x: usize, y: usize) {
        self.set(x, y, 0);
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{:010} ", self.get(x, y));
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::navigation::FlowField;

    #[test]
    fn set_destination_test() {
        let mut flow_field = FlowField::new(10, 10);
        flow_field.set_destination(4, 4);
        flow_field.print();
    }
}
