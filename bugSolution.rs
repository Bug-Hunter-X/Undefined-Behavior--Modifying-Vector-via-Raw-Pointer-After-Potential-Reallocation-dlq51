fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointer, use indexing to change the values
    v[0] = 10; 
    println!("{:?}", v);
} 