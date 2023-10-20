pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn my_demo_function(a: u32) -> u32{
    a
}
pub fn my_demo_function_alias(a: u32) -> u32{
    a
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
