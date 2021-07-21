use std::cell::Cell;

// Note that distinct lifetimes are given to `Wrapper` struct.
pub struct Wrapper<'a, 'b> {
    data: Option<&'a Inner<'b>>,
}

impl<'a, 'b> Wrapper<'a, 'b> {
    pub fn new() -> Self {
        Self { data: None }
    }

    pub fn set_inner(&mut self, data: &'a Inner<'b>) {
        self.data = Some(data);
    }
}

// Note that the `may_dangle` attribute is applied only on `'a`.
unsafe impl<#[may_dangle] 'a, 'b> Drop for Wrapper<'a, 'b> {
    fn drop(&mut self) {}
}

pub struct Inner<'a> {
    data: Cell<Option<Box<dyn SomeTrait + 'a>>>,
}

impl<'a> Inner<'a> {
    pub fn new() -> Self {
        Self {
            data: Cell::new(None),
        }
    }
}

pub trait SomeTrait {
    fn func(&self);
}

pub fn test() {
    let mut wrapper = Wrapper::new();
    let inner = Inner::new();
    wrapper.set_inner(&inner);
}

/*

By giving two distinct lifetimes `'a` and `'b` to `Wrapper` struct and making it
covariant over `'a`, the problem has been resolved.

Another nice solution here is to implement `Drop` as `impl<'a, 'b> Drop for
Wrapper<'a, 'b>` and changing the order of the declarations of `wrapper` and
`inner`. It works now since the conditions for (maybe) invariance + dropck bug
have been removed.

*/
