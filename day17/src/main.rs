use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (i32, i32),
    f_score: i32,  // Total cost (g_score + h_score)
    g_score: i32,  // Cost so far to reach the node
    direction: (i32, i32),
    consecutive_moves: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(node: &Node, grid: &Vec<Vec<i32>>) -> Vec<Node> {
    let mut neighbors = Vec::new();
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];  // Four directions: up, down, left, right

    for dir in directions.iter() {
        let new_pos = (node.position.0 + dir.0, node.position.1 + dir.1);
        if new_pos.0 >= 0 && new_pos.0 < grid.len() as i32
            && new_pos.1 >= 0 && new_pos.1 < grid[0].len() as i32 {
            let new_direction = *dir;
            let new_consecutive_moves = if node.direction == new_direction {
                node.consecutive_moves + 1
            } else {
                1
            };
            neighbors.push(Node {
                position: new_pos,
                f_score: 0,
                g_score: 0,
                direction: new_direction,
                consecutive_moves: new_consecutive_moves,
            });
        }
    }

    neighbors
}


fn heuristic(node: &Node, end: (i32, i32)) -> i32 {
    if node.consecutive_moves > 3 {
        return i32::MAX;
    }
    (node.position.0 - end.0).abs() + (node.position.1 - end.1).abs()
}

fn a_star_search(start: (i32, i32), end: (i32, i32), grid: &Vec<Vec<i32>>) -> Option<Vec<(i32, i32)>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        position: start,
        f_score: 0,
        g_score: 0,
        direction: (0, 0),
        consecutive_moves: 0
    });

    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.position == end {
            let mut path = Vec::new();
            let mut current_pos = end;
            while current_pos != start {
                path.push(current_pos);
                current_pos = came_from[&current_pos];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for neighbor in get_neighbors(&current, grid) {
            let tentative_g_score = if current.position == start {
                // If the current node is the start node, don't include its cost
                g_score[&current.position] + 0
            } else {
                // For other nodes, include the cost normally
                g_score[&current.position] + grid[neighbor.position.0 as usize][neighbor.position.1 as usize]
            };

            if tentative_g_score < *g_score.get(&neighbor.position).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.position, current.position);
                g_score.insert(neighbor.position, tentative_g_score);

                if let Some(f_score) = tentative_g_score.checked_add(heuristic(&neighbor, end)) {
                    open_set.push(Node {
                        position: neighbor.position,
                        f_score,
                        g_score: tentative_g_score,
                        direction: neighbor.direction,
                        consecutive_moves: neighbor.consecutive_moves,
                    });
                }
            }
        }
    }

    None
}


fn main() {
    part1();
}

fn draw_path_on_grid(path: &Vec<(i32, i32)>, grid: &Vec<Vec<i32>>) {
    let mut grid_chars = grid.iter().map(|row| row.iter().map(|&cell| cell.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

    for window in path.windows(2) {
        if let [from, to] = *window {
            let arrow = match (to.0 - from.0, to.1 - from.1) {
                (0, 1) => ">",
                (0, -1) => "<",
                (1, 0) => "v",
                (-1, 0) => "^",
                _ => " ", // This should not happen in a valid path
            };
            grid_chars[to.0 as usize][to.1 as usize] = arrow.to_string();
        }
    }

    for line in grid_chars {
        println!("{}", line.join(""));
    }
}

fn part1() {
    let matrix: Vec<Vec<i32>> = include_str!("input.txt").lines().map(|line| line.chars().filter_map(|c| c.to_digit(10)).map(|n| n as i32).collect()).collect();
    let path = a_star_search((0, 0), (matrix.len() as i32 - 1, matrix[0].len() as i32 - 1), &matrix).unwrap();
    println!("Path: {:?}", path);
    println!("Path length: {}", path.len() - 1);
    println!("Path cost: {}", path.iter().map(|(x, y)| matrix[*x as usize][*y as usize]).sum::<i32>() - matrix[0][0]);
    draw_path_on_grid(&path, &matrix);
}
