/*
 * @Author: ss
 * @Date: 2023-10-14 18:18:08
 * @LastEditTime: 2023-10-14 18:22:04
 * @Description: 
 * @FilePath: \rust-rustlings-2023-autumn-Ss-shuang123\exercises\error_handling\errors4.rs
 */
// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.
 

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value >0{
            Ok(PositiveNonzeroInteger(value as u64))
        }
        else if value <0
        {
            Err(CreationError::Negative)
        }
        else
        {
            Err(CreationError::Zero)
        }
       
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
