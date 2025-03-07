use hello_macro::Hello;

#[derive(Hello)]
struct Person;

fn main() {
    let p = Person;

    println!("{}", p.hello());
}