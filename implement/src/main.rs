use pollster::block_on;

use tutorial2_surface::run;

fn main() {
    let result = block_on(run());

    match result {
        Ok(_) => println!("\n\nApp ran successfully."),
        Err(_) => println!("\n\nError while running."),
    }
}
