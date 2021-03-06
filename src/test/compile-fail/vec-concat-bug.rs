fn concat<T: Copy>(v: ~[const ~[const T]]) -> ~[T] {
    let mut r = ~[];

    // Earlier versions of our type checker accepted this:
    vec::iter(v, |&&inner: ~[T]| {
        //~^ ERROR values differ in mutability
        r += inner;
    });

    return r;
}

fn main() {}