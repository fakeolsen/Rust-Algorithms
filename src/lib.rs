pub mod stacks;

#[cfg(test)]
mod tests {
    use stacks::VecStack; 
    #[test]
    fn create_vec_stack() {
        let v: VecStack<u32> = VecStack {
                stack: vec![],
                number: 0usize
            };     
        let b = VecStack::new();
        assert_eq!(v, b);
    }
    #[test]
    fn push_vec_stack(){
        let mut v = VecStack::new();
        v.push(3);
        assert_eq!(v.peek().clone(), 3);
    }
}
