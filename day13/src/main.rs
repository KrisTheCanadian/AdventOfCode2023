#[derive(Debug)]
struct Puzzle {
    matrix: Vec<Vec<char>>,
}

enum Symmetry {
    Vertical,
    Horizontal,
}

fn main() {
    let input = include_str!("input.txt");
    part1(&input);
}

fn part1(input: &str) {
    let puzzles = read_puzzles(input);

    // To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that,
    // also add 100 multiplied by the number of rows above each horizontal line of reflection.
    // In the above example, the first pattern's vertical line has 5 columns to its left
    // and the second pattern's horizontal line has 4 rows above it, a total of 405.
    let mut vertical_line_reflection_sum = 0;
    let mut horizontal_line_reflection_sum = 0;
    for puzzle in puzzles {
        let (p1, sym_type) = find_symmetric_point(&puzzle);
        match sym_type {
            Symmetry::Vertical => {
                vertical_line_reflection_sum += p1 + 1;
            },
            Symmetry::Horizontal => {
                horizontal_line_reflection_sum += p1 + 1;
            }
        }
    }

    println!("Vertical line reflection sum: {}", vertical_line_reflection_sum);
    println!("Horizontal line reflection sum: {}", horizontal_line_reflection_sum);

    println!("Total: {}", vertical_line_reflection_sum + (horizontal_line_reflection_sum * 100))
}
//   | 012345678
// 0 | #.##..##.
// 1 | ..#.##.#.
// 2 | ##......#
// 3 | ##......#
// 4 | ..#.##.#.
// 5 | ..##..##.
// 6 | #.#.##.#.

// always start after first column/row
fn find_symmetric_point(puzzle: &Puzzle) -> (usize, Symmetry) {
    // perform column vertical checks

    // keep track of which columns we're checking
    let col_sym_pt = check_columns_symmetry(puzzle);
    // print col if found
    if let Some(p1) = col_sym_pt {
        println!("Vertical symmetry found at {} and {}", p1, p1 + 1);
        return (p1, Symmetry::Vertical);
    }

    let row_sym_pt = check_rows_symmetry(puzzle);

    if let Some(p1) = row_sym_pt {
        println!("Horizontal symmetry found at {} and {}", p1, p1 + 1);
        return (p1, Symmetry::Horizontal);
    }

    panic!("No symmetry found");
}

fn check_rows_symmetry(puzzle: &Puzzle) -> Option<usize> {
    let mut p1_index = 0;
    let row_max = puzzle.matrix.len();

    while p1_index + 1 < row_max {
        let mut p1 = p1_index;
        let mut p2 = p1 + 1;
        while p2 < row_max {
            if !rows_match(p1, p2, puzzle) {
                // this isn't the point of in
                break;
            }

            if p1 == 0 || p2 == row_max - 1 {
                return Some(p1_index);
            }

            // move p1 left and move p2 right
            p1 -= 1;
            p2 += 1;
        }

        p1_index += 1;
    }
    None
}

fn rows_match(p1: usize, p2: usize, puzzle: &Puzzle) -> bool {
    for col in 0..puzzle.matrix[0].len() {
        if puzzle.matrix[p1][col] != puzzle.matrix[p2][col] {
            return false;
        }
    }
    true
}

fn check_columns_symmetry(puzzle: &Puzzle) -> Option<usize> {
    let mut p1_index = 0;
    let col_max = puzzle.matrix[0].len();

    while p1_index + 1 < col_max {
        let mut p1 = p1_index;
        let mut p2 = p1 + 1;
        while p2 < col_max {
            if !columns_match(p1, p2, puzzle) {
                // this isn't the point of in
                break;
            }

            if p1 == 0 || p2 == col_max - 1 {
                return Some(p1_index);
            }

            // move p1 left and move p2 right
            p1 -= 1;
            p2 += 1;
        }

        p1_index += 1;
    }
    None
}

fn columns_match(p1: usize, p2: usize, puzzle: &Puzzle) -> bool {
    for row in 0..puzzle.matrix.len() {
        if puzzle.matrix[row][p1] != puzzle.matrix[row][p2] {
            return false;
        }
    }
    true
}

fn read_puzzles(input: &str) -> Vec<Puzzle> {
    let mut puzzles: Vec<Puzzle> = Vec::new();
    let mut current_puzzle = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current_puzzle.is_empty() {
                puzzles.push(Puzzle { matrix: current_puzzle });
                current_puzzle = Vec::new();
            }
        } else {
            current_puzzle.push(line.chars().collect());
        }
    }

    if !current_puzzle.is_empty() {
        puzzles.push(Puzzle { matrix: current_puzzle });
    }
    puzzles
}
