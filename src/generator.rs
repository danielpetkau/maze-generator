use crate::maze::{
    Maze,
    Point,
};

struct Subset {
    parent: usize,
    rank: u32,
}

fn find(subsets: &mut Vec<Subset>, i: usize) -> usize {
    if subsets[i].parent != i {
        subsets[i].parent = find(subsets, subsets[i].parent);
    }
    subsets[i].parent
}

fn union(subsets: &mut Vec<Subset>, x: usize, y: usize) {
    let x_root = find(subsets, x);
    let y_root = find(subsets, y);

    if subsets[x_root].rank < subsets[y_root].rank {
        subsets[x_root].parent = y_root;
    } else if subsets[x_root].rank > subsets[y_root].rank {
        subsets[y_root].parent = x_root;
    } else {
        subsets[y_root].parent = x_root;
        subsets[x_root].rank += 1;
    }
}

pub fn generate_paths(maze: &mut Maze) {
    let mut points: Vec<Point> = Vec::new();
    let mut subsets: Vec<Subset> = Vec::new();

    for i in 0..maze.get_height() {
        for j in 0..maze.get_width() {
            points.push(Point {
                x: j as u32,
                y: i as u32,
            });
        }
    }

    for i in 0..points.len() {
        subsets.push(Subset {
            parent: i,
            rank: 0,
        });
    }

    let mut next_connection: usize = 0;
    let mut included_connections: usize = 0;

    while included_connections < maze.get_width() * maze.get_height() - 1 {
        let x = find(&mut subsets, maze.get_connection(next_connection).get_start().index(maze.get_width()));
        let y = find(&mut subsets, maze.get_connection(next_connection).get_end().index(maze.get_width()));

        if x != y {
            maze.include_connection(next_connection);
            union(&mut subsets, x, y);
            included_connections += 1;
        }

        next_connection += 1;
    }
}