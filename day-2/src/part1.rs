pub fn process(input: &str) -> usize {
    let reports = input
        .trim()
        .lines()
        .map(Report::from);
    
    let (safe, _): (Vec<_>, Vec<_>) = reports.partition(|report| {
        match report.safety() {
            ReportSafety::Safe => true,
            ReportSafety::Unsafe => false
        }
    });
    
    safe.len()
}



struct Report {
    levels: Vec<u8>
}

impl Report {
    pub fn safety(&self) -> ReportSafety {
        let valid = Some(())
            .and(self.validate_difference())
            .and(self.validate_unidirectional());

        match valid {
            Some(_) => ReportSafety::Safe,
            None => ReportSafety::Unsafe
        }
    }
    
    fn validate_unidirectional(&self) -> Option<()> {
        let diffs = self.levels.iter()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|i| [
                i8::try_from(*i[0]).expect("Failed to parse into signed integer"), 
                i8::try_from(*i[1]).expect("Failed to parse into signed integer"),
            ])
            .map(|i| i[1] - i[0])
            .collect::<Vec<i8>>();
        
        
        // we can determine all are negative or positive
        // by multiplication, as a switch in direction will
        // produce a negative value
        diffs
            .windows(2)
            .map(|i| i[0] * i[1])
            .all(|i| i > 0)
            .then_some(())
    }
    
    fn validate_difference(&self) -> Option<()> {
        self.levels.iter().collect::<Vec<_>>()
            .windows(2)
            .map(|i| i[0].abs_diff(*i[1]))
            .all(|diff| 0 < diff && diff <= 3)
            .then_some(())
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ReportSafety {
    Safe,
    Unsafe
}

impl From<&str> for Report{
    fn from(value: &str) -> Self {
        let levels = value
            .split_whitespace()
            .map(|i| i.parse::<u8>().expect("Failed to parse integer"))
            .collect();
        
        Self{levels}
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::part1::{process, Report, ReportSafety};

    #[test]
    fn test_process() {
        // arrange
        let input = r#"7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9"#;
        let expected = 2;

        // act
        let result = process(input);

        // assert
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_report() {
        let tests = HashMap::from([
            ("7 6 4 2 1", ReportSafety::Safe),
            ("1 2 7 8 9", ReportSafety::Unsafe),
            ("9 7 6 2 1", ReportSafety::Unsafe),
            ("1 3 2 4 5", ReportSafety::Unsafe),
            ("8 6 4 4 1", ReportSafety::Unsafe),
            ("1 3 6 7 9", ReportSafety::Safe),
        ]);
        
        for (input, expected) in tests {
            // arrange
            let report = Report::from(input);
            
            // act
            let result = report.safety();
            
            // assert
            assert_eq!(result, expected);
        }
    }
}
