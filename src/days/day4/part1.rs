use std::char;

fn handle_inputs(puzzle: Vec<Vec<char>>) -> u32 {
    let puzzle = WordSearch::new(puzzle);
    let mut total_words = 0;
    for i in 1..puzzle.get_height() {
        for j in 1..puzzle.get_length() {
            let character: char = puzzle.get_cell(i, j).unwrap().to_ascii_lowercase();
            if character == 'x' {
                // if {
                //     println!("found xmas at coordinates {} {}, with first letter {}", i-1 , j-1, character);
                //     total_words += 1;
                // }
                total_words += puzzle.find_from(i, j, "xmas", None);
            }
        }
    }
    total_words
}

struct WordSearch {
    grid: Vec<Vec<char>>
}

#[derive(Clone, Copy, Debug)]
enum WordSearchDirection {
    NorthWest = 0,
    North = 1,
    NorthEast = 2,
    West = 3,
    East = 4,
    SouthWest = 5,
    South = 6,
    SouthEast = 7,
}

impl WordSearch {

    fn new(grid: Vec<Vec<char>>) -> Self {
        WordSearch { grid }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&char> {
        self.grid.get(x).unwrap().get(y)
    }

    fn get_height(&self) -> usize {
        self.grid.len()
    }

    fn get_length(&self) -> usize {
        let row = self.grid.get(0);
        row.unwrap().len()
    }

    fn get_next_neighbour(cell_with_neighbours: Vec<(usize, usize)>, direction: WordSearchDirection) -> (usize, usize){
        cell_with_neighbours.get(direction as usize).unwrap().to_owned()
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let max_height = self.get_height();
        let max_length = self.get_length();
        let zero = 0usize;

        let possible_values = vec![
            (x-1, y-1), (x-1, y), (x-1, y+1),
            (x  , y-1),           (x  , y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1),
        ];
        possible_values
            .into_iter()
            .filter(|(x, y) | {
                if x.lt(&zero) || x.gt(&max_height) {
                    return false
                }
                if y.lt(&zero) || y.gt(&max_length) {
                    return false
                }
                return true
            })
            .collect()
    }

    fn find_from(&self, x: usize, y: usize, search: &str, direction: Option<WordSearchDirection>) -> u32 {
        // dbg!(direction);
        fn remove_first(s: &str) -> Option<&str> {
            s.chars().next().map(|c| &s[c.len_utf8()..])
        }

        if search.len().eq(&0_usize){
            return 1;
        }

        let char_at_cell = self.get_cell(x, y);
        if char_at_cell.unwrap().to_ascii_lowercase() != search.chars().collect::<Vec<_>>()[0] {
            return 0
        }
        if direction.is_some() {
            let neighbours = self.get_neighbours(x, y);
            let direction = WordSearchDirection::from(direction.unwrap());
            let (x, y) = Self::get_next_neighbour(neighbours, direction);
            self.find_from(x.to_owned(), y.to_owned(), remove_first(search).unwrap(), Some(direction))
        } else {
            let directions = vec!([
                WordSearchDirection::NorthWest,
                WordSearchDirection::North,
                WordSearchDirection::NorthEast,
                WordSearchDirection::West,
                WordSearchDirection::East,
                WordSearchDirection::SouthWest,
                WordSearchDirection::South,
                WordSearchDirection::SouthEast
            ]);
            let mut count: u32 = 0;
            directions
                .iter()
                .for_each(|direction| {
                    direction.iter().for_each(|direction|{
                        let neighbours = self.get_neighbours(x, y);
                        let direction = WordSearchDirection::from(direction.to_owned());
                        let (x, y) = Self::get_next_neighbour(neighbours, direction);
                        count += self.find_from(x.to_owned(), y.to_owned(), remove_first(search).unwrap(), Some(direction));
                    })
                });
            count
        }
    }
}

fn executed_timed(inputs: Vec<Vec<char>>) -> (u32, u128) {
    let start_time = std::time::SystemTime::now();
    let result = handle_inputs(inputs);
    let end_time = std::time::SystemTime::now();
    (result, end_time.duration_since(start_time).unwrap().as_millis())
}

pub(crate) fn run(inputs: Vec<Vec<char>>) {
    let (result, duration) = executed_timed(inputs);
    println!("Result = {}\nCompleted in {}ms", result, duration);
}
