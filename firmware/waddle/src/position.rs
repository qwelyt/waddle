pub mod position {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Position {
        row: u8,
        col: u8,
    }

    impl Position {
        pub const fn new(row: u8, col: u8) -> Self {
            Self { row, col }
        }

        fn row(&self) -> u8 {
            self.row
        }
        fn col(&self) -> u8 {
            self.col
        }
    }

    impl hash32::Hash for Position {
        fn hash<H>(&self, state: &mut H)
            where
                H: hash32::Hasher,
        {
            self.row.hash(state);
            self.col.hash(state);
        }
    }
}