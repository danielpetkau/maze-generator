use crate::maze::{
    Maze,
    Point,
};
use std::cmp;

fn get_neighbors(maze: &mut Maze, points: &Vec<Point>, target_point: usize) -> Vec<usize> {
    let mut neighbors = Vec::new();
    if points[target_point].x > 0
        && maze.get_connection(maze.connection_by_points(
            points[target_point],
            points[target_point - 1]
        ).unwrap()).is_included() {
        neighbors.push(target_point - 1);
    }
    if points[target_point].x < maze.get_width() as u32 - 1
        && maze.get_connection(maze.connection_by_points(
            points[target_point],
            points[target_point + 1]
        ).unwrap()).is_included() {
        neighbors.push(target_point + 1);
    }
    if points[target_point].y > 0
        && maze.get_connection(maze.connection_by_points(
            points[target_point],
            points[target_point - maze.get_width()]
        ).unwrap()).is_included() {
        neighbors.push(target_point - maze.get_width());
    }
    if points[target_point].y < maze.get_height() as u32 - 1
        && maze.get_connection(maze.connection_by_points(
            points[target_point],
            points[target_point + maze.get_width()]
        ).unwrap()).is_included() {
        neighbors.push(target_point + maze.get_width());
    }
    
    neighbors
}

pub fn find_solution(maze: &mut Maze) {
    let mut points: Vec<Point> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();
    let mut unvisited_points: Vec<usize> = Vec::new();
    for i in 0..maze.get_height() {
        for j in 0..maze.get_width() {
            points.push(Point {
                x: j as u32,
                y: i as u32,
            });
        }
    }
    for i in 0..points.len() {
        distances.push(
            (maze.get_width() * maze.get_height()) as u32 + 1
        );
        unvisited_points.push(i);
    }

    distances[maze.get_width() * maze.get_start().y as usize] = 0;

    while unvisited_points.len() > 0 {
        let current_point = *unvisited_points.iter().min_by_key(|&i| distances[*i]).unwrap();
        let neighbors = get_neighbors(maze, &points, current_point);

        for n in neighbors {
            distances[n] = cmp::min(
                distances[n],
                distances[current_point] + 1
            );
        }
        unvisited_points.retain(|&p| p != current_point);
    }

    let mut current_point = maze.get_end().index(maze.get_width());

    while distances[current_point] > 0 {
        let next_point = *get_neighbors(maze, &points, current_point)
        .iter().min_by_key(|&i| distances[*i]).unwrap();
        
        maze.add_to_solution(maze.connection_by_points(points[current_point], points[next_point])
        .unwrap());

        current_point = next_point;
    }
}