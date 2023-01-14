//! Unsafe: tread carefully!
//!
//! # GhostCell: A cell with ghost ownership.
//!
//! Often, interior mutability in Rust comes with unsatisfying tradeoffs: thread
//! safety, generality of types, runtime overhead (both time and space) or even
//! runtime failure.
//! To avoid some of these issues, one can improve interior mutability by taking
//! advantage of Rust's ownership, but the trick is unfortunately
//! limited to `Cell`s where one has direct ownership.
//!
//! Here, we extend this trick to `GhostCell` where one has *logical* (ghost)
//! ownership, with the following techniques:
//!
//! - Invariant lifetimes (to generate unforgeable, compile-time unique lifetime
//!   tokens).
//! - Higher rank lifetimes (to ensure code is parametric with respect to the
//!   chosen token).
//! - Ghost ownership (to allow ownership to be held by the token's owner, rather
//!   than by the cells themselves).
//!
//! The API works as follows:
//! 1. create a `GhostToken`, within an appropriate scope (a "context").
//! 2. create cells that reference the ghost token. They must reference exactly
//!    one ghost token, due to lifetime invariance, and no two ghost tokens can
//!    be the same, due to lifetime parametricity.
//! 3. to access values at a given lifetime and mutability, borrow the token
//!    with that lifetime and mutability. As a result, Rust's guarantees about
//!    ownership and lifetimes flow from the token to its owned cells.
//!
//! [The methods provided by this type have been formally verified in Coq.](http://plv.mpi-sws.org/rustbelt/ghostcell/)

use core::{cell::UnsafeCell, marker::PhantomData};

/// An invariant lifetime--required in order to make sure that a GhostCell can
/// be owned by a single ghost token.
#[derive(Clone, Copy, Default)]
struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

impl<'id> InvariantLifetime<'id> {
    #[inline]
    const fn new() -> InvariantLifetime<'id> {
        InvariantLifetime(PhantomData)
    }
}

/// A ghost token.
///
/// Once created, a `GhostToken` can neither be cloned nor copied.
///
/// Note that `GhostToken` does not need to know which types it is protecting. The
/// reason is that in order to do anything with a `GhostCell<T>`, both the token
/// *and* a reference to the `GhostCell` are required. Since a `GhostCell` inherits
/// trait markers (and other information) from the type `T` of data it is
/// protecting, `GhostToken` does not need to; it only needs to carry around
/// ownership of the entire set of `GhostCell`s.
/// For example, one could create a `GhostCell<'id, Rc<T>>` and then send its
/// owning `GhostToken<'id>` to another thread, but since one cannot actually send
/// the `GhostCell` to another thread, it is not possible to create a race
/// condition in this way.
///
/// Note also that `'id` is totally disjoint from `T` itself--it is only used as
/// a unique compile-time identifier for a set of (`GhostCell`s of) `T`s, and
/// otherwise has no relationship to `T`.
pub struct GhostToken<'id> {
    _marker: InvariantLifetime<'id>,
}

/// A ghost cell.
///
/// A `GhostCell` acts exactly like a `T`, except that its contents are
/// accessible only through its owning `GhostToken`.
#[derive(Default)]
#[repr(transparent)]
pub struct GhostCell<'id, T: ?Sized> {
    _marker: InvariantLifetime<'id>,
    value: UnsafeCell<T>, // invariant in `T`
}

/// `GhostToken<'id>` does not need to worry about what types it is protecting,
/// since information about `Send` and `Sync` is already carried by
/// the `GhostCell<'id, T>`. Therefore, it's always safe to send a `GhostToken`
/// between threads.
unsafe impl<'id> Send for GhostToken<'id> {}

/// `GhostToken<'id>` does not need to worry about what types it is protecting,
/// since information about `Send` and `Sync` is already carried by
/// the `GhostCell<'id, T>`. Therefore, it's always safe to share a `GhostToken`
/// between threads.
unsafe impl<'id> Sync for GhostToken<'id> {}

/// `GhostCell<'id, T>` implements `Send` iff `T` does. This is safe because in
/// order to access the `T` mutably within a `GhostCell<T>`, you need both a
/// mutable reference to its owning `GhostToken` and an immutable reference to
/// `GhostCell<T>`, and both references must have the same lifetime.
unsafe impl<'id, T> Send for GhostCell<'id, T> where T: Send {}

/// `GhostCell<'id, T>` implements `Sync` iff `T` is `Send + Sync`. This is safe
/// because in order to access the `T` immutably within a `GhostCell<T>`, you
/// need both an immutable reference to its owning `GhostToken` and an immutable
/// reference to `GhostCell<T>`, and both references must have the same lifetime.
unsafe impl<'id, T> Sync for GhostCell<'id, T> where T: Send + Sync {}

impl<'id> GhostToken<'id> {
    /// Create a new `GhostToken` with a particular lifetime identifier, `'id`.
    ///
    /// The argument function `f` must be parametric with respect to its lifetime
    /// parameter `'new_id`; this guarantees that `'id` is chosen by `new`, not
    /// the client. Because `'id` is invariant with respect to `GhostToken`, and
    /// cannot be chosen by the client to replicate an existing `GhostToken`, we
    /// know that `'id` is unique per call of `new`.
    #[inline]
    pub fn new<F, R>(f: F) -> R
    where
        F: for<'new_id> FnOnce(GhostToken<'new_id>) -> R,
    {
        // We choose the lifetime; it is definitely unique for each new instance
        // of `GhostToken`.
        let token = GhostToken {
            _marker: InvariantLifetime::new(),
        };
        // Return the result of running `f`.  Note that the `GhostToken` itself
        // cannot be returned, because `R` cannot mention the lifetime `'id`, so
        // the `GhostToken` only exists within its scope.
        f(token)
    }
}

impl<'id, T> GhostCell<'id, T> {
    /// Creates a new cell that belongs to the token at lifetime `'id`. This
    /// consumes the value of type `T`. From this point on, the only way to access
    /// the inner value is by using a `GhostToken` with the same `'id`. Since
    /// `'id` is always chosen parametrically, and `'id` is invariant for both
    /// the `GhostCell` and the `GhostToken`, if one chooses `'id` to correspond
    /// to an existing `GhostToken<'id>`, that is the only `GhostToken<'id>` to
    /// which the `GhostCell` belongs. Therefore, there is no way to access the
    /// value through more than one token at a time.
    ///
    /// As with `GhostToken` itself, note that `'id` has no relationship to
    /// `T`---it is only used as a unique, static marker.
    ///
    /// A subtle point to make is around `Drop`. If `T`'s `Drop` implementation
    /// is run, and `T` has a reference to a `GhostToken<'id>`, it seems that
    /// since the invariant that `GhostToken<'id>` must be accessed mutably to
    /// get a mutable reference to the `T` inside a `GhostCell<'id, T>` is being
    /// bypassed, there could be a soundness bug here. Fortunately, thanks to
    /// `dropck`, such pathological cases appear to be ruled out. For example,
    /// this code will not compile:
    ///
    /// ```compile_fail
    /// use util::ghost_cell::{GhostToken, GhostCell};
    ///
    /// struct Foo<'a, 'id>(&'a GhostToken<'id>,
    ///                         GhostCell<'id, GhostCell<Option<&'a Foo<'a, 'id>>>>)
    ///                              where 'id: 'a;
    ///
    /// impl<'a, 'id> Drop for Foo<'a, 'id> {
    ///     fn drop(&mut self) {
    ///         match self.0.get(&self.1).get() {
    ///             Some(ref foo) => {
    ///                 println!("Oops, have aliasing.");
    ///             },
    ///             None => {
    ///                 println!("Okay")
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// GhostToken::new(|token| {
    ///     let foo = Foo(&token, GhostCell::new(GhostCell::new(None)));
    ///     foo.1.borrow(&token).set(Some(&foo));
    /// });
    /// ```
    ///
    /// It will compile if the manual `Drop` implementation is removed, but only
    /// pathological `Drop` implementations are an issue here.  I believe there
    /// are two factors at work: one, in order to have a reference to a value,
    /// the token must outlive the reference. Two, types must *strictly* outlive
    /// the lifetimes of things they reference if they have a nontrivial `Drop`
    /// implementation.  As a result, if there is any reference to a `GhostCell`
    /// containing the type being dropped from within the type being dropped,
    /// and it has a nontrivial `Drop` implementation, it will not be possible to
    /// complete the cycle.  To illustrate more clearly, this fails, too:
    ///
    /// ```compile_fail
    /// fn foo() {
    ///     struct Foo<'a>(GhostCell<Option<&'a Foo<'a>>>);
    ///
    ///     impl<'a> Drop for Foo<'a> {
    ///         fn drop(&mut self) {}
    ///     }
    ///
    ///     let foo = Foo(GhostCell::new(None));
    ///     foo.0.set(Some(&foo));
    /// }
    /// ```
    ///
    /// So any conceivable way to peek at a self-reference within a `Drop`
    /// implementation is probably covered.
    #[inline]
    pub const fn new(value: T) -> Self {
        GhostCell {
            _marker: InvariantLifetime::new(),
            value: UnsafeCell::new(value),
        }
    }

    /// Unwraps the value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    /// Get an immutable reference to the item that lives for as long as the
    /// owning token is immutably borrowed (the lifetime `'a`).
    #[inline]
    pub fn borrow<'a>(&'a self, _token: &'a GhostToken<'id>) -> &'a T {
        unsafe {
            // We know the token and lifetime are both borrowed at 'a, and the
            // token is borrowed immutably; therefore, nobody has a mutable
            // reference to this token. Therefore, any items in the set that are
            // currently aliased would have been legal to alias at &'a T as well,
            // so we can take out an immutable reference to any of them, as long
            // as we make sure that nobody else can take a mutable reference to
            // any item in the set until we're done.
            &*self.value.get()
        }
    }

    /// Get a mutable reference to the item that lives for as long as the owning
    /// token is mutably borrowed.
    #[inline]
    pub fn borrow_mut<'a>(&'a self, _token: &'a mut GhostToken<'id>) -> &'a mut T {
        unsafe {
            // We know the token and lifetime are both borrowed at `'a`, and the
            // token is borrowed mutably; therefore, nobody else has a mutable
            // reference to this token.  As a result, all items in the set are
            // currently unaliased, so we can take out a mutable reference to
            // any one of them, as long as we make sure that nobody else can
            // take a mutable reference to any other item in the set until
            // the current borrow is done.
            &mut *self.value.get()
        }
    }
}

impl<'id, T> From<T> for GhostCell<'id, T> {
    #[inline]
    fn from(t: T) -> Self {
        GhostCell::new(t)
    }
}

impl<'id, T: ?Sized> GhostCell<'id, T> {
    /// Returns a raw pointer to the underlying data in this cell.
    pub const fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// This call borrows `GhostCell` mutably (at compile-time) which guarantees
    /// that we possess the only reference.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }

    /// Returns a `&mut GhostCell<'id, T>` from a `&mut T`
    #[inline]
    pub fn from_mut(t: &mut T) -> &mut Self {
        unsafe { &mut *(t as *mut T as *mut Self) }
    }
}

impl<'id, T> GhostCell<'id, [T]> {
    /// Returns a `&[GhostCell<'id, T>]` from a `&GhostCell<'id, [T]>`
    #[inline]
    pub fn as_slice_of_cells(&self) -> &[GhostCell<'id, T>] {
        unsafe { &*(self as *const GhostCell<'id, [T]> as *const [GhostCell<'id, T>]) }
    }
}

impl<'id, T: Clone> GhostCell<'id, T> {
    /// Convenience method to clone the `GhostCell` when `T` is `Clone`, as long
    /// as the token is available.
    #[inline]
    pub fn clone(&self, token: &GhostToken<'id>) -> Self {
        GhostCell::new(self.borrow(token).clone())
    }
}

impl<'id, T: ?Sized> AsMut<T> for GhostCell<'id, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}
