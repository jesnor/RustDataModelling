use crate::{clear::Clear, utils::decr};
use std::cell::Cell;

type Index = usize;

pub struct CellPool<T> {
    items: Vec<T>,
    prev: Vec<Cell<Index>>,
    next: Vec<Cell<Index>>,
    first_free: Cell<Index>,
    first: Cell<Index>,
    last: Cell<Index>,
    size: Cell<Index>,
}

impl<T: Default> CellPool<T> {
    pub fn new(capacity: Index) -> Self {
        let mut s = Self {
            items: Vec::with_capacity(capacity),
            prev: Vec::with_capacity(capacity),
            next: Vec::with_capacity(capacity),
            first_free: Cell::new(0),
            first: Cell::new(0),
            last: Cell::new(0),
            size: Cell::new(0),
        };

        for i in 0..capacity {
            s.items.push(Default::default());
            s.prev.push(Cell::new(capacity));
            s.next.push(Cell::new(i + 1));
        }

        s
    }
}

impl<T> CellPool<T> {
    pub fn alloc(&self) -> Result<&T, &'static str> {
        let s = self.size.get();

        if s >= self.items.capacity() {
            return Err("Pool empty!");
        }

        // Remove first from free list
        let index = self.first_free.get();
        self.first_free.set(self.next[index].get());

        // Add last in item list
        let li = self.last.get();
        self.next[li].set(index);
        self.prev[index].set(li);
        self.next[index].set(self.items.capacity());
        self.last.set(index);

        if s == 0 {
            self.first.set(index)
        }

        self.size.set(s + 1);
        Ok(&self.items[index])
    }

    pub fn iter(&self) -> PoolIter<T> {
        PoolIter {
            pool: self,
            index: self.first.get(),
        }
    }
}

impl<T: Clear> CellPool<T> {
    pub fn free(&self, p: &T) -> Result<(), &'static str> {
        let si = unsafe { (p as *const T).offset_from(&self.items[0] as *const T) };

        if si < 0 {
            return Err("Invalid item!");
        }

        let i = si as usize;
        let p = self.prev[i].get();

        if p >= self.items.capacity() {
            return Err("Item already freed!");
        }

        let n = self.next[i].get();

        // Remove from item list
        self.next[p].set(n);
        self.prev[n].set(p);

        if i == self.first.get() {
            self.first.set(n)
        }

        if i == self.last.get() {
            self.last.set(p)
        }

        // Add to free list
        let ff = self.first_free.get();
        self.first_free.set(i);
        self.prev[i].set(self.items.capacity());
        self.next[i].set(ff);

        decr(&self.size);
        Ok(())
    }
}

impl<T: Clear> Clear for CellPool<T> {
    fn clear(&self) {
        for v in self.items.iter() {
            v.clear()
        }

        for x in self.prev.iter() {
            x.set(self.items.capacity())
        }

        for (i, x) in self.next.iter().enumerate() {
            x.set(i + 1)
        }

        self.first_free.set(0);
        self.first.set(0);
        self.last.set(0);
        self.size.set(0);
    }
}

pub struct PoolIter<'t, T> {
    pool: &'t CellPool<T>,
    index: usize,
}

impl<'t, T> Iterator for PoolIter<'t, T> {
    type Item = &'t T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;

        if i >= self.pool.items.capacity() {
            None
        } else {
            self.index = self.pool.next[i].get();
            Some(&self.pool.items[i])
        }
    }
}
