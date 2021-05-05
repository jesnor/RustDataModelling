use std::{
    cell::Cell,
    ops::{Add, Sub},
};

pub fn incr<T: Copy + Add<T, Output = T> + From<u8>>(c: &Cell<T>) -> T {
    let v = c.get() + 1u8.into();
    c.set(v);
    v
}

pub fn decr<T: Copy + Sub<T, Output = T> + From<u8>>(c: &Cell<T>) -> T {
    let v = c.get() - 1u8.into();
    c.set(v);
    v
}
