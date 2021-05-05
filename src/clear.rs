use std::cell::Cell;

pub trait Clear {
    fn clear(&self);
}

impl<T: Default> Clear for Cell<T> {
    fn clear(&self) {
        self.set(Default::default())
    }
}
