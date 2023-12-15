#[derive(Debug)]
struct Puzzle {
    matrix: Vec<Vec<char>>,
}

#[derive(Clone)]
enum Symmetry {
    Vertical,
    Horizontal,
}

fn main() {
    let input = include_str!("input.txt");
    part1(&input);
    part2(&input);
}

fn part2(input: &str) {
    let puzzles = read_puzzles(input);

    // To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that,
    // also add 100 multiplied by the number of rows above each horizontal line of reflection.
    // In the above example, the first pattern's vertical line has 5 columns to its left
    // and the second pattern's horizontal line has 4 rows above it, a total of 405.
    let mut vertical_line_reflection_sum = 0;
    let mut horizontal_line_reflection_sum = 0;
    for puzzle in puzzles {
        let (p1, sym_type) = find_symmetric_point_2(&puzzle);
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

fn print_puzzle(puzzle: &Puzzle) {
    for row in &puzzle.matrix {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn find_symmetric_point_2(puzzle: &Puzzle) -> (usize, Symmetry) {
    let mut symmetric_points: Vec<(usize, Symmetry, usize)> = Vec::new();

    // keep track of which columns we're checking
    let col_sym_pt = check_columns_symmetry_2(puzzle);
    // print col if found
    let col_sym = col_sym_pt;
    for col in col_sym {
        println!("Vertical symmetry found at {} and {}", col.0, col.0 + 1);
        symmetric_points.push((col.0, Symmetry::Vertical, col.1));
    }


    let row_sym_pt = check_rows_symmetry_2(puzzle);

    let row_sym = row_sym_pt;
    for row in row_sym {
        println!("Horizontal symmetry found at {} and {}", row.0, row.0 + 1);
        symmetric_points.push((row.0, Symmetry::Horizontal, row.1));
    }


    for (p1, sym_type, mismatch_count) in &symmetric_points {
        if *mismatch_count == 1 {
            return (*p1, sym_type.clone());
        }
    }

    panic!("No symmetry found");
}

fn check_rows_symmetry_2(puzzle: &Puzzle) -> Vec<(usize, usize)> {
    let mut p1_index = 0;
    let row_max = puzzle.matrix.len();
    let mut symmetric_points: Vec<(usize, usize)> = Vec::new();

    while p1_index + 1 < row_max {
        let mut p1 = p1_index;
        let mut p2 = p1 + 1;
        let mut mismatch_count = 0;
        while p2 < row_max {

            let (rows_match, mismatch) = rows_match_2(p1, p2, puzzle);
            mismatch_count += mismatch;

            if !rows_match || mismatch_count > 1 {
                break;
            }

            if p1 == 0 || p2 == row_max - 1 {
                symmetric_points.push((p1_index, mismatch_count as usize));
                break;
            }

            // move p1 left and move p2 right
            p1 -= 1;
            p2 += 1;
        }

        p1_index += 1;
    }
    symmetric_points
}

fn rows_match_2(p1: usize, p2: usize, puzzle: &Puzzle) -> (bool, i32) {
    let mut mismatch_count = 0;
    for col in 0..puzzle.matrix[0].len() {
        if puzzle.matrix[p1][col] != puzzle.matrix[p2][col] {
            mismatch_count += 1;
        }

        if mismatch_count > 1 {
            return (false, mismatch_count);
        }
    }
    (true, mismatch_count)
}

fn check_columns_symmetry_2(puzzle: &Puzzle) -> Vec<(usize, usize)> {
    let mut p1_index = 0;
    let col_max = puzzle.matrix[0].len();
    let mut symmetric_points: Vec<(usize, usize)> = Vec::new();

    while p1_index + 1 < col_max {
        let mut p1 = p1_index;
        let mut p2 = p1 + 1;
        let mut mismatch_count = 0;

        while p2 < col_max {
            let (cols_match, mismatch) = columns_match_2(p1, p2, puzzle);
            mismatch_count += mismatch;

            if !cols_match || mismatch_count > 1 {
                break;
            }

            if p1 == 0 || p2 == col_max - 1 {
                symmetric_points.push((p1_index, mismatch_count as usize));
                break;
            }

            // move p1 left and move p2 right
            p1 -= 1;
            p2 += 1;
        }

        p1_index += 1;
    }
    symmetric_points
}

fn columns_match_2(p1: usize, p2: usize, puzzle: &Puzzle) -> (bool, i32) {
    let mut mismatch_count = 0;
    for row in 0..puzzle.matrix.len() {
        if puzzle.matrix[row][p1] != puzzle.matrix[row][p2] {
            mismatch_count += 1;
        }

        if mismatch_count > 1 {
            return (false, mismatch_count);
        }
    }
    (true, mismatch_count)
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
