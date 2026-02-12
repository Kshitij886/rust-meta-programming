#![allow(unused)]

// trait Area {
//     fn area(&self) -> i32;
// }

// struct Rec {
//     w: i32,
//     h: i32,
// }

// impl Area for Rec {
//     fn area(&self) -> i32 {
//         return self.h * self.w;
//     }
// }

// fn main() {
//     let shape = Rec { w: 32, h: 34 };
//     println!("Area of Rec is {}", shape.area());
// }

trait Limiter {
    fn allow(&mut self) -> bool;
}
struct FixedWindowLimiter {
    current_rrequests: u32,
    max_request: u32,
}

impl Limiter for FixedWindowLimiter {
    fn allow(&mut self) -> bool {
        self.current_rrequests = self.current_rrequests + 1;
        if self.current_rrequests > self.max_request {
            true
        } else {
            false
        }
    }
}

struct Service<T: Limiter> {
    limiter: T,
}

impl<T: Limiter> Service<T> {
    fn handle_request(&mut self) {
        if self.limiter.allow() {
            println!("Request Accepted");
        } else {
            println!("Rate Limit Reached");
        }
    }
}
fn main() {
    let mut u = FixedWindowLimiter {
        current_rrequests: 0,
        max_request: 4,
    };
    let mut v = Service { limiter: u };
    println!("{:?}", v.handle_request());
}
