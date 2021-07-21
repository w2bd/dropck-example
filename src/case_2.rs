use std::cell::Cell;

pub struct Wrapper<'a> {
    data: Option<&'a Inner<'a>>,
}

impl<'a> Wrapper<'a> {
    pub fn new() -> Self {
        Self { data: None }
    }

    pub fn set_inner(&mut self, data: &'a Inner<'a>) {
        self.data = Some(data);
    }
}

// This is the only different part from the previous one.
unsafe impl<#[may_dangle] 'a> Drop for Wrapper<'a> {
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

Now, we get a slightly different compile error.

```
error[E0597]: `inner` does not live long enough
  --> src/case_2.rs:42:23
   |
42 |     wrapper.set_inner(&inner);
   |                       ^^^^^^ borrowed value does not live long enough
43 | }
   | -
   | |
   | `inner` dropped here while still borrowed
   | borrow might be used here, when `inner` is dropped and runs the destructor for type `Inner<'_>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
```

Note that the different part is "~, when `inner` is dropped ~"; It was "~, when
`wrapper` is dropped ~" before fixing the drop declaration. Also note that
changing the order of let statement for `inner` and `wrapper` will not fix the
problem.

It looks like no problem since we promise to the compiler that `wrapper` will
not access the borrowed `inner` in its drop function. As compiler does not emit
errors about `wrapper`, it seems that `wrapper` does not have a problem. The
error is about `inner`. `inner` also should not have a problem in this example.
Then, is it somehow related to variance? It seems not. The variance concept
itself should not have the problem in this example.

The cause of the problem here seems to be an internal compile error as
described in https://rust-lang.github.io/rfcs/0769-sound-generic-drop.html#mixing-acyclic-structure-and-drop-is-sometimes-rejected.

Here is my guess based on the RFC 769. Invariance along with drop check confuses
the region inference system in compiler. `Inner` struct is invariant over `'a`
since it has the field of `Cell<T>` type which is invariant over `T`, and so
`Wrapper` struct is also invariant over `'a` as it has the `&'a Inner<'a>` type
inside. When `wrapper.set_inner(&inner)` is called, the lifetime for `&inner`
should be the scope of the `inner` variable but the compiler by mistake selects
the region which covers the whole block of `fn test()` function for the lifetime
for `&inner` rather than the region covered by only `let inner = ...` statement.
It means to compiler that when `inner` is dropped there might still be borrow
for the `inner` and it somehow triggers the drop check problem for `inner`.

(Note that `Inner` struct is subject to drop check because it owns the `Box<dyn
SomeTrait + 'a>` type in the end(`Cell<...>` owns `Option<...>` and it owns
`Box<dyn SomeTrait + 'a>`) and the `Box<dyn SomeTrait + 'a>` type is assumed
that it has a `Drop` implementation parametric in `'a` as mentioned in the RFC.)

Then, how can we fix the problem? We can circumvent it
1) by making `Wrapper` struct covariant over `'a` or
2) by making `Inner` struct not subject to drop check.
The first one can be easily executed by giving two distinct lifetimes `'a` and
`'b` to `Wrapper` or replacing `Cell<T>` type with other owned and covariant
type over `T` from `Inner` struct. The second one can be executed by not owning
the `Box<dyn SomeTrait + 'a>` type in the end, e.g., by replacing it with
`&'a (dyn SomeTrait + 'a)`.

let's try the first solution in the case_3.

*/
