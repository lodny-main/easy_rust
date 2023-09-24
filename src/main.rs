// closure
// Fn() -> u8      // reference        : &self
// FnMut() -> u8   // can mutate       : &mut Self
// FnOnce() -> u8  // can be used once : Self

fn fn_closure<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn fn_mut_closure<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn fn_once_closure<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let mut my_string = String::from("Hello there");

    // simple closure
    let print_it = || {
        println!("{my_string}");
    };
    print_it();

    // closure parameter
    fn_closure(|| {
        println!("{my_string}");
    });

    fn_mut_closure(|| {
        my_string.push_str(" mut");
        println!("{my_string}");
        // drop(my_string);    // cannot move out of `my_string`, a captured variable in an `FnMut` closure
    });

    fn_once_closure(|| {
        my_string.push_str(" once");
        println!("{my_string}");
        drop(my_string)
    });
}
