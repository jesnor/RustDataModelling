use std::cell::Cell;

#[derive(Clone)]
pub struct RefSet<'t, T> {
    items: Vec<Cell<Option<&'t T>>>,
}

impl<'t, T> RefSet<'t, T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: vec![Default::default(); capacity],
        }
    }

    pub fn add(&self, v: &'t T) -> Result<(), &'static str> {
        for x in self.items.iter() {
            if x.get().is_none() {
                x.set(Some(v));
                return Ok(());
            }
        }

        Err("Out of space!")
    }

    pub fn remove(&self, v: &'t T) -> Result<(), &'static str> {
        for x in self.items.iter() {
            if let Some(a) = x.get() {
                if a as *const T == v as *const T {
                    x.set(None);
                    return Ok(());
                }
            }
        }

        Err("Item not in set!")
    }

    pub fn iter(&self) -> impl Iterator<Item = &'t T> + '_ {
        self.items.iter().filter_map(|x| x.get())
    }
}

impl<'t, T> Default for RefSet<'t, T> {
    fn default() -> Self {
        Self::new(10)
    }
}
