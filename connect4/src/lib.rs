/// Can be either Red or Blue
#[derive(Clone, PartialEq, Debug)]
pub enum Chip {
    Red,
    Blue,
}

/// Container for Chips, which is empty (None) by default
pub struct Board {
    rows: usize,
    cols: usize,
    chips: Vec<Vec<Option<Chip>>>,
}

impl Board {
    /// Instantiate board with None
    ///
    /// # Arguments
    ///
    /// * `rows` - size of rows
    /// * `cols` - size of columns
    ///
    /// # Example
    ///
    /// ```
    /// let game = Board::new(10, 10);
    ///
    /// assert!(game.get(9, 9) == None)
    /// ```
    ///
    /// # Panic
    ///
    /// It needs to have space for the game to happen
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 4 && cols > 4);

        Board {
            rows,
            cols,
            chips: (0..rows)
                .map(|_| (0..cols).map(|_| None).collect())
                .collect(),
        }
    }

    /// Drop a Chip in given column
    ///
    /// Should let the Chip fall until the lowest available empty row
    ///
    /// # Arguments
    ///
    /// * `self` - mutable reference to board
    /// * `col` - column to drop
    /// * `pl` - color of Chip (who's turn is it)
    ///
    /// # Example
    ///
    /// ```
    /// let game = Board::new(10, 10);
    ///
    /// game.add(1, Chip::Blue);
    /// assert!(game.get(9, 1) == Some(Chip::Blue));
    ///
    /// game.add(1, Chip::Red);
    /// assert!(game.get(8, 1) == Some(Chip::Red));
    /// ```
    pub fn add(&mut self, col: usize, pl: Chip) {
        assert!(self.chips[0][col].is_none());

        for row in self.chips.iter_mut().rev() {
            if let None = row[col] {
                row[col] = Some(pl);
                break;
            }
        }
    }

    /// Get a copy of value in given position of board
    ///
    /// Return None instead of index error
    ///
    /// # Arguments
    ///
    /// * `self` - reference to board
    /// * `row` - row index
    /// * `col` - column index
    ///
    /// # Example
    ///
    /// ```
    /// let game = Board::new(10, 10);
    /// assert!(game.get(9, 0) == None);
    ///
    /// game.add(0, Chip::Red);
    /// assert!(game.get(9, 0) == Some(Chip::Red));
    /// ```
    pub fn get(&self, row: i8, col: i8) -> Option<Chip> {
        if row < 0 || row >= self.rows as i8 || col < 0 || col >= self.cols as i8 {
            return None;
        }
        let (row, col) = (row as usize, col as usize);

        self.chips[row][col].clone()
    }

    /// Checks if a player won the game
    ///
    /// If 4 chips of the same color can be found sequentially
    /// in a vertical, horizontal or diagonal position, it means
    /// a player has won the game, else return None
    ///
    /// # Arguments
    ///
    /// * `self` - reference to board
    ///
    /// # Example
    ///
    /// ```
    /// let game = Board::new(10, 10);
    ///
    /// (0..4).map(|_| game.add(0, Chip::Blue));
    /// assert!(game.check() == Some(Chip::Blue));
    ///
    /// let game = Board::new(10, 10);
    ///
    /// (0..4).map(|c| game.add(c, Chip::Red));
    /// assert!(game.check() == Some(Chip::Red));
    /// ```
    pub fn check(&self) -> Option<Chip> {
        let dirs: [(i8, i8); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];

        for i in 0..self.rows {
            for j in 0..self.cols {
                for dir in dirs.iter() {
                    let mut counter = Vec::with_capacity(4);

                    for freq in 0..4 {
                        let (x, y) = ((dir.0 * freq) + i as i8, (dir.1 * freq) + j as i8);
                        counter.push(self.get(x, y));
                    }

                    if counter.windows(2).all(|w| w[0] == w[1]) && counter[0] != None {
                        return counter[0].clone();
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests;
