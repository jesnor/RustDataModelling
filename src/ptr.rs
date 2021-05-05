use core::fmt;
use std::hash::Hasher;
use std::{cmp::Ordering, fmt::Pointer};
use std::{hash::Hash, ops::Deref};

pub struct Ptr<'t, T> {
    value: &'t T,
}

impl<T> Copy for Ptr<'_, T> {}

impl<T> Clone for Ptr<'_, T> {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl<T> std::fmt::Debug for Ptr<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> fmt::Pointer for Ptr<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&(self.value as *const T), f)
    }
}

impl<'t, T> Ptr<'t, T> {
    pub fn new(value: &'t T) -> Self {
        Self { value }
    }

    pub fn get(self) -> &'t T {
        self.value
    }
}

impl<'t, T> From<&'t T> for Ptr<'t, T> {
    fn from(value: &'t T) -> Self {
        Ptr::new(value)
    }
}

impl<T> Deref for Ptr<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<T> PartialEq for Ptr<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value as *const T == other.value as *const T
    }
}

impl<T> Eq for Ptr<'_, T> {}

impl<'t, T> Ord for Ptr<'t, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.value as *const T).cmp(&(other.value as *const T))
    }
}

impl<'t, T> PartialOrd for Ptr<'t, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'t, T> Hash for Ptr<'t, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.value as *const T).hash(state);
    }
}
