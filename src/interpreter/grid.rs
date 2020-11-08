pub trait Grid: Default {
    fn get(&self, x: u8, y: u8) -> u8;
    fn set(&mut self, x: u8, y: u8, value: u8);
}

pub struct SimpleGrid {
    // naive implementation
    cells: Vec<Vec<u8>>,
}

impl Default for SimpleGrid {
    fn default() -> Self {
        SimpleGrid {
            cells: vec![vec![0; 256]; 256],
        }
    }
}

impl Grid for SimpleGrid {
    fn get(&self, x: u8, y: u8) -> u8 {
        unimplemented!()
    }

    fn set(&mut self, x: u8, y: u8, value: u8) {
        unimplemented!()
    }
}
