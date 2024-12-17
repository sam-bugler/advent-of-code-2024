pub fn process(input: &str) -> usize {
    let mut word_grid = WordGrid::from(input);
    let search_term = SearchTerm::from("XMAS");
    
    word_grid.word_count(search_term)
}

#[derive(Debug)]
enum PeekDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}

/* === Word Grid === */
struct WordGrid {
    pointer: (usize, usize),
    peek_pointer: (usize, usize),
    grid: Vec<Vec<char>>, // stored as y, x
}

impl WordGrid {
    pub fn word_count(&mut self, search_term: SearchTerm) -> usize {
        let mut search_term = search_term;
        let mut count: usize = 0;
        
        while let Some(char) = self.current_char() {
            if Some(char) == search_term.word.first() {
                self.reset_peek_pointer();
                if self.walk(&mut search_term, &PeekDirection::Up) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::UpRight) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::Right) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::DownRight) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::Down) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::DownLeft) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::Left) {count += 1}
                if self.walk(&mut search_term, &PeekDirection::UpLeft) {count += 1}
            }
            
            self.increment_pointer();
        }

        count
    }

    fn increment_pointer(&mut self) {

        let mut x = self.pointer.0;
        let mut y = self.pointer.1;
        
        let row = self.grid.get(y);
        
        match row {
            Some(_) if y < self.grid.len() - 1 => {y+=1}
            _ => {x+=1; y=0}
        }
        
        self.pointer.0 = x;
        self.pointer.1 = y;
    }

    fn reset_peek_pointer(&mut self) {
        self.peek_pointer.0 = self.pointer.0;
        self.peek_pointer.1 = self.pointer.1;
    }

    fn walk(&mut self, search_term: &mut SearchTerm, direction: &PeekDirection) -> bool {
        while let (Some(char), Some(search_char)) = (self.peek(direction), search_term.increment_pointer()) {
            if char != search_char { break } 
        }
        
        let found = search_term.current_char().is_none();

        search_term.reset_pointer();
        self.reset_peek_pointer();

        found
    }

    fn peek(&mut self, direction: &PeekDirection) -> Option<&char> {

        let x = self.peek_pointer.0;
        let y = self.peek_pointer.1;

       let pointer = match direction {
            PeekDirection::Up => { (x,y.checked_sub(1)?) }
            PeekDirection::UpRight => { (x+1, y.checked_sub(1)?) }
            PeekDirection::Right => { (x+1, y) }
            PeekDirection::DownRight => { (x+1, y+1) }
            PeekDirection::Down => { (x,y+1) }
            PeekDirection::DownLeft => { ( x.checked_sub(1)?, y+1 ) }
            PeekDirection::Left => { (x.checked_sub(1)?, y) }
            PeekDirection::UpLeft => { (x.checked_sub(1)?, y.checked_sub(1)?) }
        };
        
        self.peek_pointer.0 = pointer.0;
        self.peek_pointer.1 = pointer.1;

        self.grid
            .get(self.peek_pointer.1)
            .and_then(|i| i.get(self.peek_pointer.0))
    }
    
    fn current_char(&self) -> Option<&char> {
        let x = self.pointer.0;
        let y = self.pointer.1;
        
        self.grid.get(y).and_then(|row| row.get(x))
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
            pointer: (0, 0),
            peek_pointer: (0,0)
        }
    }
}

/* === Search Term === */
struct SearchTerm {
    word: Vec<char>,
    pointer: usize,
}

impl SearchTerm {
    pub fn reset_pointer(&mut self) {
        self.pointer = 0;
    }
    
    pub fn increment_pointer(&mut self) -> Option<&char> {
        self.pointer += 1;
        self.word.get(self.pointer)
    }

    pub fn current_char(&self) -> Option<&char>  {
        self.word.get(self.pointer)
    }
}

impl From<&str> for SearchTerm {
    fn from(value: &str) -> Self {
        Self {
            word: value.chars().collect(),
            pointer: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part1::process;

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
       
        let expected = 18;

        // act
        let result = process(input);

        // assert
        assert_eq!(
            result, expected,
            "result: {:?}, expected: {:?}",
            result, expected
        )
    }
}
