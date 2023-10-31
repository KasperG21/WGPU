use pollster::block_on;

use tutorial2_surface::run;

fn main() {
    let result = block_on(run());

    match result {
        Ok(_) => println!("App ran successfully."),
        Err(_) => println!("Error while running."),
    }
}
