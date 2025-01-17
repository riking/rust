// Test that when we infer the lifetime to a subset of the fn body, it
// works out.
//
// check-pass

#![allow(drop_ref)]

trait MyTrait<'a> {
    type Output;
}

fn foo1<T>()
where
    for<'x> T: MyTrait<'x>,
{
    // Here the region `'c` in `<T as MyTrait<'c>>::Output` will be
    // inferred to a subset of the fn body.
    let x = bar::<T::Output>();
    drop(x);
}

fn bar<'a, T>() -> &'a ()
where
    T: 'a,
{
    &()
}

fn main() {}
