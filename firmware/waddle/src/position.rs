pub mod position {
    use crate::layout::COLS;

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Position {
        row: u8,
        col: u8,
    }

    impl Position {
        pub fn from(i: usize) -> Self {
            let row = i / COLS;
            let col = i % COLS;
            Self { row: row as u8, col: col as u8 }
        }

        pub fn row(&self) -> u8 {
            self.row
        }
        pub fn col(&self) -> u8 {
            self.col
        }
    }
}