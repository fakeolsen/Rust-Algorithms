//main function for sanity
extern crate algorithms;
use algorithms::stacks;

fn main(){
    let mut v = stacks::VecStack::new();
    v.push(3);
    println!("The topmost stack item in v is: {}", v.peek());
}
