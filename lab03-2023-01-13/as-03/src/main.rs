
fn sum_of_numbers(n: i32) -> i32 {
    (n*(n+1))/2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_numbers() {
        assert_eq!(sum_of_numbers(100), 5050);  
    }
}