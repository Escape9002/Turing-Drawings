use rand::prelude::*;
use image::{GrayImage, Luma};

pub const ACTION_LEFT: u32 = 0;
pub const ACTION_RIGHT: u32 = 1;
pub const ACTION_UP: u32 = 2;
pub const ACTION_DOWN: u32 = 3;
pub const NUM_ACTIONS: u32 = 4;

pub struct TuringProgram {
    pub num_states: u32,
    pub num_symbols: u32,
    pub map_width: u32,
    pub map_height: u32,
    pub table: Vec<u32>,
    pub map: Vec<u32>,
    pub pos: (u32, u32),
    pub state: u32,
}

impl TuringProgram {
    pub fn new(num_states: u32, num_symbols: u32, map_width: u32, map_height: u32) -> Self {
        let mut table = vec![0; (num_states * num_symbols * 3) as usize];
        let map = vec![0; (map_width * map_height) as usize];
        
        for st in 0..num_states {
            for sy in 0..num_symbols {
                let st1 = rand::random_range(0..num_states);
                let sy1 = rand::random_range(0..num_symbols);
                let ac1 = rand::random_range(0..NUM_ACTIONS);

                Self::set_trans(num_symbols, &mut table, st, sy, st1, sy1, ac1);
            }
        }

        Self {
            num_states,
            num_symbols,
            map_width,
            map_height,
            table,
            map,
            pos: (map_width / 2, map_height / 2),
            state: 0,
        }
    }

    fn set_trans(num_symbols: u32, table: &mut Vec<u32>, st0: u32, sy0: u32, st1: u32, sy1: u32, ac1: u32) {
        let idx = ((st0 * num_symbols + sy0) * 3) as usize;
        table[idx + 0] = st1;
        table[idx + 1] = sy1;
        table[idx + 2] = ac1;
    }

    pub fn step(&mut self) {
        let (x, y) = self.pos;
        let idx = (y * self.map_width + x) as usize;
        let symbol = self.map[idx];
    
        let trans_idx = ((self.state * self.num_symbols + symbol) * 3) as usize;
        self.state = self.table[trans_idx + 0];
        self.map[idx] = self.table[trans_idx + 1];
        let action = self.table[trans_idx + 2];
    
        let (mut new_x, mut new_y) = match action {
            ACTION_LEFT  => (x.wrapping_sub(1), y),
            ACTION_RIGHT => (x + 1, y),
            ACTION_UP    => (x, y.wrapping_sub(1)),
            ACTION_DOWN  => (x, y + 1),
            _ => (x, y),
        };
    
        let new_x = match action {
            ACTION_LEFT  => (x.wrapping_sub(1)) % self.map_width,
            ACTION_RIGHT => (x.wrapping_add(1)) % self.map_width,
            _ => x,
        };
        
        let new_y = match action {
            ACTION_UP    => (y.wrapping_sub(1)) % self.map_height,
            ACTION_DOWN  => (y.wrapping_add(1)) % self.map_height,
            _ => y,
        };
        
        self.pos = (new_x, new_y);
        
    }

    pub fn save_map_as_image(&self, filename: &str) {
        let mut img = GrayImage::new(self.map_width, self.map_height);

        for y in 0..self.map_height {
            for x in 0..self.map_width {
                let idx = (y * self.map_width + x) as usize;
                let symbol = self.map[idx];

                // Map symbol to grayscale (0-255)
                let intensity = ((symbol * 255) / (self.num_symbols - 1)).min(255) as u8;
                img.put_pixel(x, y, Luma([intensity]));
            }
        }

        img.save(filename).expect("Failed to save image");
    }
    
}
