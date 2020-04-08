use std::{fmt, fs};

/// An instance of the game of life board
#[derive(Debug)]
pub struct GameOfLife {
    /// contains a matrix of boolean values
    cells: Vec<Vec<bool>>,
}

impl GameOfLife {
    /// Create an instance
    ///
    /// # Arguments
    ///
    /// * `rows` - num of rows in cell matrix
    /// * `cols` - num of columns in cell matrix
    ///
    /// # Example
    ///
    /// ```
    /// let game = GameOfLife::new(10, 10);
    /// ```
    ///
    /// # Panics
    ///
    /// In order to check the surrounding cells, it needs at least 3x3 size
    pub fn new(rows: usize, cols: usize) -> GameOfLife {
        assert!(rows > 3 && cols > 3);

        GameOfLife {
            cells: vec![vec![false; cols]; rows],
        }
    }

    /// Creates an instance from a given vector
    ///
    /// This if usefull when reading values from a file which contains
    /// all the cells in order
    ///
    /// # Arguments
    ///
    /// * `cells` - matrix of booleas
    ///
    /// # Example 
    ///
    /// ```
    /// let cells = vec![vec![true, false, false, true, true]; 5];
    ///
    /// let game = GameOfLife {
    ///     cells
    /// };
    /// ```
    pub fn from(cells: Vec<Vec<bool>>) -> GameOfLife {
        GameOfLife { cells }
    }

    /// Count the surrouding living cells
    ///
    /// Process that data to decide if current cell stays alive,
    /// if neighbours amount to 2 or 3, gets created, if 3 or dies
    /// of more or less than those values
    ///
    /// # Arguments
    ///
    /// * `row` - row index
    /// * `col` - column index
    pub fn check_surroundings(&self, row: usize, col: usize) -> bool {
        let mut counter = 0;

        for i in row - 1..=row + 1 {
            for j in col - 1..=col + 1 {
                if i == j {
                    continue;
                }

                if self.cells[i % self.cells.len()][j % self.cells[0].len()] {
                    counter += 1;
                }
            }
        }

        match counter {
            3 => true,
            2 => self.cells[row][col],
            _ => false,
        }
    }

    /// Calculates next phase by checking all of the cells
    /// and creating instance of new cellular automaton with results
    pub fn next(self) -> GameOfLife {
        let mut cells = self.cells.clone();

        for i in 0..cells.len() {
            for j in 0..cells[0].len() {
                cells[i][j] = self.check_surroundings(i, j);
            }
        }

        GameOfLife { cells }
    }

    /// Getter function
    ///
    /// Reads the values of a given cell
    ///
    /// # Arguments
    ///
    /// * `row` - row index
    /// * `col` - column index
    ///
    /// # Example
    ///
    /// ```
    /// let game = GameOfLife(10, 10);
    /// assert!(!game.get(5, 5));
    ///
    /// ```
    pub fn get(&self, row: usize, col: usize) -> bool {
        self.cells[row][col]
    }

    /// Setter function
    ///
    /// needed to set a cell to its opposite by clicking (GUI)
    ///
    /// # Arguments
    ///
    /// * `row` - row index
    /// * `col` - column index
    ///
    /// # Example
    ///
    /// ```
    /// let game = GameOfLife(10, 10);
    /// assert!(!game.get(5, 5));
    ///
    /// game.set(5, 5);
    /// assert!(game.get(5, 5));
    /// ```
    pub fn set(&mut self, row: usize, col: usize) {
        self.cells[row][col] = !self.cells[row][col];
    }
}

/// Implement the Display trait for GameOfLife struct
///
/// This automatically creates a ToString trait implementation
/// for the struct as well, which is needed in the write_to_file
/// function later
impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let contents = self.cells.iter().fold(String::new(), |mut acc, row| {
            acc.push_str(
                &row.iter()
                    .map(|col| match col {
                        true => '1',
                        false => '0',
                    })
                    .collect::<String>(),
            );
            acc
        });

        write!(f, "{}", contents)
    }
}

/// Read files instance to game
///
/// The file should contain a list of 0s and 1s for living and empty cells
/// this values get read and create an instance of the struct
///
/// # Arguments
///
/// * `filename` - name of the file containing the values
///
/// # Panics
///
/// Panics if the file wasn't found or its contents are not
/// simetrical in order to create a matrix
///
/// # Example
///
/// ```
/// let game = read_from_file("test.txt")?;
/// ```
pub fn read_from_file(filename: &str) -> std::io::Result<GameOfLife> {
    let mut cells = vec![vec![]];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            cells[i][j] = c == '1';
        }
    }

    // Only accept matrices so each line needs the same length
    assert!(cells.windows(2).all(|w| w[0].len() == w[1].len()));

    Ok(GameOfLife::from(cells))
}

/// write current state of cellular automaton to a file
///
/// # Arguments
///
/// * `instance` - an instance of the game
/// * `filename` - a string slice with the desired name
///
/// # Panics
///
/// Return a panic message if it wasn't able to write to file
///
/// # Example
///
/// ```
/// let game = GameOfLife::new(10, 10);
///
/// write_to_file(game, "test.txt");
/// ```
pub fn write_to_file(instance: GameOfLife, filename: &str) -> std::io::Result<()> {
    fs::write(filename, instance.to_string()).expect("Unable to write to file");

    Ok(())
}

#[cfg(test)]
mod tests;
