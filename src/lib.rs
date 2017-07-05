pub mod stacks;

#[cfg(test)]
mod tests {
    use stacks::VecStack; 
    #[test]
    fn create_vec_stack() {
        let v: VecStack<u32> = VecStack {
                stack: vec![],
                number: 0u32
            };     
        let b = VecStack::new();
        assert_eq!(v, b);
    }
}
