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

impl<'a> Drop for Wrapper<'a> {
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

If we compile the `fn test()`, we will get this compile error.

```
error[E0597]: `inner` does not live long enough
  --> src/base_case.rs:42:23
   |
42 |     wrapper.set_inner(&inner);
   |                       ^^^^^^ borrowed value does not live long enough
43 | }
   | -
   | |
   | `inner` dropped here while still borrowed
   | borrow might be used here, when `wrapper` is dropped and runs the `Drop` code for type `Wrapper`
   |
   = note: values in a scope are dropped in the opposite order they are defined

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
```

What is the error message about? It looks like no problem as the drop function
for `wrapper` does not really access the borrowed `inner` when `wrapper` is
dropped even though `wrapper` has an window where the `data` field refers to
the already dropped `inner` because `inner` is dropped first and then `wrapper`
does. But, the problem here is that compiler does not know whether drop
function for `wrapper` actually accesses(reads or writes) the borrowed `inner`
or not. It is because compiler does not see through function body; It only
looks at the declaration part.

With the declaration of `impl<'a> Drop for Wrapper<'a>`, compiler thinks that
`wrapper` might access the `data` field(equivalent to the borrowed `inner` in
this example) regardless of whether it really does or not. We cannot tell the
compiler that `wrapper` does not access the `data` field using this normal
`Drop` syntax. It is where the `may_dangle` attribute comes in. By rewriting
the declaration for drop as `unsafe impl<#[may_dangle] 'a> Drop for Wrapper<'a>`,
we can tell the compiler that `wrapper` will not access the lifetime `'a`
relevant field (`data`). With this setting, compiler will now know that
`wrapper` does not access the borrowed `inner` when dropped and everything is
good. So, the modified version should work.

Let's try it in the `case_2`.

*/
