// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x; //value borrowed 
    println!("{:?}, {:?}", x, y);
}
