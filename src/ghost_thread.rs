use std::cell::RefCell;

use rayon::join;

use crate::ghost_cell::{GhostCell, GhostToken};

pub fn test() {
    GhostToken::new(|mut token| {
        let c = GhostCell::new(10);
        let d = RefCell::new(10);
        let cr = &c;

        rayon::join(
            || println!("{}", c.borrow(&token)),
            || println!("{}", c.borrow(&token)),
        );

        println!("{}", c.borrow(&token));
    })
}
