mod cell;
mod cell_pool;
mod clear;
mod ghost_pool;
mod ghost_rc;
mod ptr;
mod ref_cell;
mod ref_count;
mod ref_set;
mod static_cell;
mod utils;

fn main() -> Result<(), &'static str> {
    println!("Rc:");
    ref_count::run_game();
    println!();

    println!("RefCell:");
    ref_cell::run_game();
    println!();

    println!("Cell pool:");
    cell::run_game()?;
    println!();

    println!("Cell static pool:");
    static_cell::run_game()?;
    println!();

    println!("Ghost Rc:");
    ghost_rc::run_game();
    println!();

    println!("Ghost pool:");
    ghost_rc::run_game();
    println!();

    Ok(())
}
