// impl std::fmt::Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(My name is {} and my age is  {})", self.name, self.age)
//     }
// }
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}
fn main() {
    let u = User {
        name: String::from("Kshitij khatri"),
        age: 18,
    };
    println!("{:?}", u);
    // if i do this this spit out an error
    // `User` doesn't implement `std::fmt::Display`
    // the trait `std::fmt::Display` is not implemented for `User`
    // in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // so lets implement this trait
    // after implementing display trait
    // output is : (My name is Kshitij khatri and my age is  18)
    // but wouldn't it be nice it to have a default implementation
    // so this is where the first procedural macro (custom derive macros)
    // to implement this we need to do #[derive(Debug)]
}
