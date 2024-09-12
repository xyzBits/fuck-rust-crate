use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("static value passed in is: {:?}", input);
}

fn main() {
    // I is owned and contains no references, thus it's 'static
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of main()
    // so it's not 'static
    // print_it(&i);

    let x = Box::new(i);
    let input: &'static mut i32 = Box::leak(x);
    print_it(input);
}
