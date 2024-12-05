use std::str::FromStr;

// todo! - solution producing result too low, unit tests are passing, revisit!!

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
    fn validate_unidirectional(levels: &[u8]) -> Result<(), u8> {

        let diffs = levels
            .windows(2)
            .map(|i| [
                i8::try_from(i[0]).expect("Failed to parse into signed integer"),
                i8::try_from(i[1]).expect("Failed to parse into signed integer"),
            ])
            .map(|i| (i[0], i[1] - i[0]))
            .collect::<Vec<_>>();
        
        let mut iter = diffs.iter().peekable();
        
        while let Some(i) = iter.next() {
            if let Some(next) = iter.peek() {
                let delta = i.1 * next.1;
                if delta < 0 {
                    return Err(next.0 as u8)
                }
            }
        };
        Ok(())
    }

    fn validate_difference(levels: &[u8]) -> Result<(), u8> {
        levels.windows(2)
            .map(|i| (i[0], i[0].abs_diff(i[1])))
            .try_for_each(|(level, diff)| {
                match 0 < diff && diff <= 3 {
                    true => Ok(()),
                    false => Err(level)
                }
            })
    }
}

impl From<Report> for GradedReport {
    fn from(value: Report) -> Self {
        let mut report = value;

        let validate = | report: &Report | {
            Ok(())
                .and(Self::validate_unidirectional(&report.levels))
                .and(Self::validate_difference(&report.levels))
        };

        let result = validate(&report)
            .or_else(|level| {
                println!("error with report {:?} at {:?}", report.levels, level);
                if let Some(position) = report.levels.iter().position(|i| *i == level) {
                    report.levels.remove(position);
                    println!("report dampened, new report: {:?}", report.levels)
                }
                validate(&report)
            });

        match result {
            Ok(_) => Self::Safe(report),
            Err(_) => Self::Unsafe(report)
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
