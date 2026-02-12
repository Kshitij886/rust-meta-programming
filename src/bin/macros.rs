// macros are the way of writing code which writes other code which is known as meta programming
// macros VS functions
// MetaProgramming is useful for reducing the amount of code you have to write and maintain, whihch is also one of the roles of functions.
// However, macros have some additional powers that functions don't have
//
// A function signature mut declare the number and type of params the function has. Macros, on the other hand, can take a varibale number of params:
// we can call println!() with one args or println!() with two args. Also macros are expanded before the compiler interprets the meaning of the code,
// so a macro can, for eg, implement a trait on a given type. A function can't, because it gets called at runtime and a trait need to be implemented before compiler time.
//

fn main() {
    println!("Hello world");
}
