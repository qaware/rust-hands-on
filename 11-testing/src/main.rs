#![allow(unused)]

fn main() {}

fn get_result() -> Result<(), String> {
    Err(String::from("something happened"))
}

#[cfg(test)]
mod tests {
    use crate::get_result;

    #[test]
    fn test_stuff() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected="divide by zero")]
    fn test_arithmetic() {
        let zero = 1 - 1;
        1 / zero;
    }

    #[test]
    fn test_result_ok() -> Result<(), String> {
        //get_result()
        Ok(())
    }
}