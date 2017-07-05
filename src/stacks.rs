#[derive(Debug, PartialEq)]
pub struct VecStack<T> {
    pub stack: Vec<T>,
    pub number: u32
}

impl<T> VecStack<T> {
    //return an empty stack
    pub fn new() -> VecStack<T> {
       VecStack {
            stack: vec![],
            number: 0
        } 
    }
    //add an item to the stack
    //fn push(&mut self, item){
    //    self.stack.push(item);
    //}
}
