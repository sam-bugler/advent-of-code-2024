use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let typed_input = TypedInput::from(input);
    let frequencies = FrequencyGraph::from(typed_input.right);
    
    typed_input.left.iter().fold(0, |acc, i| {
        let frequency = frequencies.hashmap.get(i).unwrap_or(&0);
        acc + (i * frequency)
    })
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        //arrange
        let input = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "#;
        
        // act
        let result = process(input);
        let expected = 31;
        
        // assert
        assert_eq!(result, expected);
    }
}

struct TypedInput {
    left: Vec<u32>,
    right: Vec<u32>
}

impl From<&str> for TypedInput {
    fn from(value: &str) -> Self {

        let mut left: Vec<u32> = Vec::new();
        let mut right: Vec<u32> = Vec::new();

        for line in value.lines() {
            let mut numbers = line.split_whitespace();

            if let Some(number) = numbers.next() {
                left.push(number.parse::<u32>().expect("Error parsing left number"))
            }

            if let Some(number) = numbers.next() {
                right.push(number.parse::<u32>().expect("Error parsing right number"))
            }
        }
        
        Self{left,right}
    }
}

struct FrequencyGraph {
    hashmap: HashMap<u32, u32>
}

impl From<Vec<u32>> for FrequencyGraph {
    fn from(value: Vec<u32>) -> Self {
        let mut hashmap: HashMap<u32, u32> = HashMap::new();
        
        for key in value {
            hashmap.entry(key)
                .and_modify(|i| *i += 1)
                .or_insert(1);
        }
        
        Self{hashmap}
    }
}