use {
    anyhow::Result,
    log::debug,
};

pub struct Factorial {}

impl Factorial {
    /// Factorial calculates the factorial of the given number. Cannot calculate
    /// negative numbers and floating point's factorials. Also, it cannot handle
    /// Integer values that are bigger than 128 bytes.
    ///
    /// # Example:
    /// ```rust
    /// use container::logic::factorial::*;
    ///
    /// let number = 3;
    /// assert_eq!(6, Factorial::factorial(number).unwrap())
    /// ```
    pub fn factorial(n: i128) -> Result<i128> {
        let mut sum = 1;
        debug!("Number that its factorial going to be calculated: {}", &n);
        if n.is_negative() {
            return Err(anyhow::anyhow!("Cannot handle Negative numbers!",));
        }
        match n {
            0 => Ok(1),
            1 => Ok(1),
            _ => {
                debug!("Iterating number");
                for num in (1..n + 1).rev() {
                    debug!("Current number: {}", &num);
                    sum *= num;
                    debug!("Current sum: {}", &sum);
                }
                Ok(sum)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn factorial_of_one_should_be_one() {
        assert_eq!(1, Factorial::factorial(1).unwrap());
    }
    #[test]
    fn factorial_of_zero_should_be_one() {
        assert_eq!(1, Factorial::factorial(0).unwrap());
    }

    #[test]
    fn non_negative_numbers_should_error() {
        let num = -1;
        if let Err(e) = Factorial::factorial(num) {
            assert_eq!("Cannot handle Negative numbers!", e.to_string());
        }
    }

    #[test]
    fn factorial_of_3_should_return_6() {
        assert_eq!(6, Factorial::factorial(3).unwrap())
    }

    #[test]
    fn check_factorials() {
        let num = 25;
        let expected = 15511210043330985984000000;

        assert_eq!(expected, Factorial::factorial(num).unwrap());
    }
}
