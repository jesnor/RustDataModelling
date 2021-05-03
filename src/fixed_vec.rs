use std::cell::Cell;

pub trait Clear {
    fn clear(&self);
}

pub struct FixedVec<'t, T: Clear> {
    items: &'t [T],
    is_occupied: Vec<Cell<bool>>,
    size: Cell<usize>,
}

impl<'t, T: Clear> FixedVec<'t, T> {
    pub fn new(items: &'t [T]) -> Self {
        Self {
            items,
            is_occupied: vec![Cell::new(false); items.len()],
            size: Cell::new(0),
        }
    }

    pub fn alloc(&self) -> &'t T {
        for i in 0..self.items.len() {
            if !self.is_occupied[i].get() {
                self.is_occupied[i].set(true);
                self.size.set(self.size.get() + 1);
                return &self.items[i];
            }
        }

        panic!("Vector is full!")
    }

    pub fn free(&self, v: &'t T) {
        let is = unsafe { (v as *const T).offset_from(&self.items[0] as *const T) };
        assert!(is >= 0);
        let i = is as usize;
        v.clear();
        self.is_occupied[i].set(false);
        self.size.set(self.size.get() - 1);
    }
}
