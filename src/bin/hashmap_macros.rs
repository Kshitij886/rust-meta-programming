macro_rules! mapify {
    ($($x:expr => $y:expr), *) => {{
        let mut temp_hashmap = std::collections::HashMap::new();
        $(
            temp_hashmap.insert($x,$y);
        )*
        temp_hashmap
    }};
}
fn main() {
    let map = mapify! {
        "a" => 1,
        "b" => 2,
        "c" => 3
    };
    println!("{:?}", map);
}
