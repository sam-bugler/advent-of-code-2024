use std::cmp::PartialEq;
use std::fmt::Debug;

pub fn process(input: &str) -> usize {
    let mut word_grid = WordGrid::from(input);
    let mut search_kernel = Kernel::new([['M', '*', 'S'], ['*', 'A', '*'], ['M', '*', 'S']]);

    let mut count = 0;

    while let Some(kernel) = word_grid.current_kernel() {
        for _ in 0..4 {
            if search_kernel == kernel {
                count += 1;
            }
            search_kernel.rotate();
        }

        word_grid.increment_position()
    }

    count
}

/* === Coords === */
#[derive(Debug, PartialEq)]
struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/* === Kernel === */
#[derive(Debug)]
struct Kernel([[char; 3]; 3]);

impl Kernel {
    const WILDCARD: char = '*';

    pub fn new(value: [[char; 3]; 3]) -> Self {
        Self(value)
    }

    // could use From<[char;9]> and From<[[char;3];3]> traits
    pub fn new_flat(value: [char; 9]) -> Self {
        let kernel = [
            [value[0], value[1], value[2]],
            [value[3], value[4], value[5]],
            [value[6], value[7], value[8]],
        ];

        Self(kernel)
    }

    pub fn rotate(&mut self) {
        let mut rotated = [[' '; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                rotated[j][2 - i] = self.0[i][j];
            }
        }

        self.0 = rotated;
    }
}

impl PartialEq for Kernel {
    fn eq(&self, other: &Self) -> bool {
        let self_flat = self.0.iter().flatten();
        let other_flat = other.0.iter().flatten();

        self_flat
            .zip(other_flat)
            .all(|(a, b)| a == &Self::WILDCARD || b == &Self::WILDCARD || a == b)
    }
}

/* === Word Grid === */
struct WordGrid {
    grid: Vec<Vec<char>>,
    kernel_size: usize,
    kernel_position: Coords,
}

impl WordGrid {
    pub fn current_kernel(&self) -> Option<Kernel> {
        let range = |position: usize| position..position + self.kernel_size;

        let mut chars = [' '; 9];
        let mut ptr = 0;

        for y in range(self.kernel_position.y) {
            for x in range(self.kernel_position.x) {
                let char = self.grid.get(y).and_then(|row| row.get(x))?;
                chars[ptr] = *char;
                ptr += 1;
            }
        }

        Some(Kernel::new_flat(chars))
    }

    pub fn increment_position(&mut self) {
        let row = self.grid.get(self.kernel_position.y);

        match row {
            Some(_) if self.kernel_position.y + self.kernel_size < self.grid.len() => {
                self.kernel_position.y += 1;
            }
            _ => {
                self.kernel_position.x += 1;
                self.kernel_position.y = 0;
            }
        }
    }
}

impl From<&str> for WordGrid {
    fn from(value: &str) -> Self {
        let grid = value
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect::<Vec<_>>();

        Self {
            grid,
            kernel_size: 3,
            kernel_position: Coords::new(0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part2::{process, Kernel};

    #[test]
    fn test_process() {
        // arrange
        let input = r#"
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
        "#;

        let expected = 9;

        // act
        let result = process(input);

        // assert
        assert_eq!(
            result, expected,
            "result: {:?}, expected: {:?}",
            result, expected
        )
    }

    #[test]
    fn kernel_rotates() {
        // arrange
        let a = Kernel::new_flat(['a', 'b', 'c', 'd', 'e', 'f', 'h', 'i', 'j']);
        let mut b = Kernel::new_flat(['a', 'b', 'c', 'd', 'e', 'f', 'h', 'i', 'j']);

        // act
        for _ in 0..4 {
            b.rotate();
        }

        // assert
        assert_eq!(a, b)
    }
}
