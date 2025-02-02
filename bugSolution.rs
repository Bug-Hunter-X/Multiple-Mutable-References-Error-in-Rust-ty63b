fn main() {
    let mut x = 5;
    { //Creating a scope to limit the lifetime of the mutable reference
        let y = &mut x; 
        *y = 6; 
    }
    
    let z = &mut x; //Now this is allowed since y's mutable reference is out of scope
    *z = 7; 
    println!("x = {}", x);
}