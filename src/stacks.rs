#[derive(Debug, PartialEq)]
pub struct VecStack<T> {
    pub stack: Vec<T>,
    pub number: usize
}

impl<T> VecStack<T> {
    //return an empty stack
    pub fn new() -> VecStack<T> {
       VecStack {
            stack: vec![],
            number: 0usize
        } 
    }
    //add an item to the stack
    pub fn push(&mut self, item: T){
        self.stack.push(item);
        self.number = self.number + 1;
    }
    //fetch the current top item on the stack
    //Note: we have to return a reference because we cannot move out of index. 
    //However, some references do not implement equality comparison.
    //In said cases we have to clone the result to a new value as in the test for push.
    pub fn peek(&self) -> &T {
      if self.number > 0 {
            &self.stack[self.number - 1] 
        } else {
            &self.stack[self.number]
        } 
    }
}
