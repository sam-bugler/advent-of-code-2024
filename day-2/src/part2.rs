use std::str::FromStr;

pub fn process(input: &str) -> usize {
    let reports = input
        .trim()
        .lines()
        .map(|i| Report::from_str(i).expect("Failed to parse report"));

    let (safe, _): (Vec<_>, Vec<_>) = reports
        .map(GradedReport::from)
        .partition(|report| {
            match report {
                GradedReport::Safe(_) => true,
                GradedReport::Unsafe(_) => false
            }
    });

    safe.len()
}

#[derive(Debug, PartialEq)]
struct Report {
    levels: Vec<u8>
}

impl FromStr for Report{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|i| i.parse::<u8>().expect("Failed to parse integer"))
            .collect();

        Ok(Self{levels})
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum GradedReport {
    Safe(Report),
    Unsafe(Report)
}

impl GradedReport {
    
    fn validate_unidirectional(levels: &[u8]) -> bool{

        let diffs = levels
            .windows(2)
            .map(|i| [
                i8::try_from(i[0]).expect("Failed to parse into signed integer"),
                i8::try_from(i[1]).expect("Failed to parse into signed integer"),
            ])
            .map(|i| i[1] - i[0])
            .collect::<Vec<_>>();
        
        diffs
            .windows(2)
            .map(|i| i[0] * i[1])
            .all(|i| i > 0)
    }

    fn validate_difference(levels: &[u8]) -> bool {
        levels.windows(2)
            .map(|i| i[0].abs_diff(i[1]))
            .all(|diff| diff <= 3)
    }
}

impl From<Report> for GradedReport {
    fn from(report: Report) -> Self {
        
        let validate = |levels: &[u8] | {
            Self::validate_difference(levels) && Self::validate_unidirectional(levels)
        };
        
        let bruteforce_retry = |levels: &[u8]| {
            (0..levels.len()).any(|ptr| {
                let mut levels = report.levels.clone();
                levels.remove(ptr);
                validate(&levels)
            })
        };

        let result = validate(&report.levels) || bruteforce_retry(&report.levels);

        match result {
            true => Self::Safe(report),
            false => Self::Unsafe(report)
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::str::FromStr;
    use crate::part2::{process, Report, GradedReport};

    #[test]
    fn test_process() {
        // arrange
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;
        let expected = 4;

        // act
        let result = process(input);

        // assert
        assert_eq!(result, expected);
    }

    #[test]
    fn test_report() {
        let tests = HashMap::from([
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", true),
            ("8 6 4 4 1", true),
            ("1 3 6 7 9", true),
        ]);

        for (input, expected) in tests {
            // arrange
            let report = Report::from_str(input).expect("Failed parsing report");

            // act
            let result = match GradedReport::from(report) {
                GradedReport::Safe(_) => true,
                GradedReport::Unsafe(_) => false
            };

            // assert
            assert_eq!(result, expected, "input {} failed", input);
        }
    }
}
