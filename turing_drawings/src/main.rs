use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod turing;
use turing::TuringProgram;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let mut program = TuringProgram::new(6, 6, WIDTH as u32, HEIGHT as u32);
    let mut window = Window::new("Turing Drawings", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Unable to open window");

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..1000 {
            program.step();
        }

        for y in 0..HEIGHT as u32 {
            for x in 0..WIDTH as u32 {
                let idx = (y * WIDTH as u32 + x) as usize;
                let symbol = program.map[idx];
                let intensity = ((symbol * 255) / (program.num_symbols - 1)).min(255) as u8;
                buffer[idx] = ((intensity as u32) << 16) | ((intensity as u32) << 8) | (intensity as u32);
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");

        // std::thread::sleep(Duration::from_millis(8)); // Slow down to ~60 FPS
    }
}
