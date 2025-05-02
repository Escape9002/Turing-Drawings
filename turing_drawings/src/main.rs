mod turing;

use turing::{TuringProgram, ACTION_LEFT, ACTION_RIGHT};

fn main() {
    let mut program = TuringProgram::new(4, 3, 512, 512);

    for _ in 0..1000000 {
        program.step();
    }

    println!("Final position: {:?}", program.pos);
    println!("Final state: {}", program.state);
    program.save_map_as_image("output.png");
}
