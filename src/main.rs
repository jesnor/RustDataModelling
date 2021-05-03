mod cell;
mod fixed_vec;
mod ref_cell;
mod ref_count;
mod ref_set;
mod static_cell;

fn main() {
    println!("Ref count:");
    ref_count::run_game();
    println!();

    println!("Ref cell:");
    ref_cell::run_game();
    println!();

    println!("Cell:");
    cell::run_game();
    println!();

    println!("Static cell:");
    static_cell::run_game();
    println!();
}
