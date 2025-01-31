fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe and efficient way to modify vector element
    println!("First element: {:?}", v[0]);
} 