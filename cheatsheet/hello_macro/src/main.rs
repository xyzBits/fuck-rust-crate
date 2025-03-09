use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Person;

fn main() {
    Person::hello_macro();
}
