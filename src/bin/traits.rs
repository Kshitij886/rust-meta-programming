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

use std::sync::WaitTimeoutResult;

trait Limiter {
    fn allow(&mut self) -> bool;
}
struct FixedWindowLimiter {
    max_requests: u32,
    current_requests: u32,
}

impl Limiter for FixedWindowLimiter {
    fn allow(&mut self) -> bool {
        self.current_requests = self.current_requests + 1;
        if self.current_requests > self.max_requests {
            return false;
        }
        return true;
    }
}

struct Service<T: Limiter> {
    limiter: T,
}
impl<T: Limiter> Service<T> {
    fn handle_request(&mut self) {
        if self.limiter.allow() {
            println!("Request processed");
        } else {
            println!("Rate Limit Exceeded");
        }
    }
}

fn main() {
    let mut fixed_window = FixedWindowLimiter {
        max_requests: 3,
        current_requests: 0,
    };
    let mut service = Service {
        limiter: fixed_window,
    };
    for i in 0..5 {
        println!("is allowed {:?}", service.handle_request());
    }
}
