// creating our own custom derive macros
// Create a simple Serialize and Deserilize macros that would implement the serialize and deserialize traits

use std::io::Error;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(base: Vec<u8>) -> Swap;
}

struct Swap {
    qty_1: u32,
    qty_2: u32,
}
impl Deserialize for Swap {
    fn deserialize(base: Vec<u8>) -> Result<Swap, Error> {
        if base.len() < 8 {
            return Err(Error);
        }
        let qty_1 = base[0..3];
        let qty_2 = base[4..7];
        return Ok(Swap { qty_1, qty_2 });
    }
}
impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}

fn main() {
    println!("hello");
}
