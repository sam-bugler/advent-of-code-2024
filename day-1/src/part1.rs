use std::iter::zip;

pub fn process(input: &str) -> u32 {
    // let rows = input.lines().count();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace().into_iter();
        
        if let Some(number) = numbers.next() {
            left.push(number.parse::<u32>().expect("Error parsing left number"))
        }
        
        if let Some(number) = numbers.next() {
            right.push(number.parse::<u32>().expect("Error parsing right number"))
        }
    }
    
    left.sort();
    right.sort();
    
    zip(left, right)
        .fold(0, |acc, (left, right)| acc + right.abs_diff(left) )
}

#[cfg(test)]
pub mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        // arrange
        let input = r#"
            3 4
            4 3
            2 5
            1 3
            3 9
            3 3
        "#;

        // act
        let result = process(input);
        let expected = 11;
        
        // assert
        assert_eq!(result, expected);
    }
}
