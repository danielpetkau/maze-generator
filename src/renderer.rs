use crate::{
    generator, solver,
    maze::{
        Maze,
        Direction
    }
};
use ggez::{
    graphics::{self, Color},
    Context, GameResult
};

const MAZE_WIDTH: usize = 30;
const MAZE_HEIGHT: usize = 20;
pub const WINDOW_WIDTH: f32 = 1500.0;
pub const WINDOW_HEIGHT: f32 = 1000.0;
const WALL_WIDTH : f32 = 2.0;
const SOLUTION_WIDTH: f32 = 10.0;

pub fn create_meshes(ctx: &mut Context) -> GameResult<Vec<graphics::Mesh>> {
    let mut maze = Maze::new(MAZE_WIDTH, MAZE_HEIGHT);
    let mut meshes = Vec::new();

    let tile_width = (WINDOW_WIDTH - WALL_WIDTH * (MAZE_WIDTH as f32 + 1.0))
        / MAZE_WIDTH as f32;
    let tile_height = (WINDOW_HEIGHT - WALL_WIDTH * (MAZE_HEIGHT as f32 + 1.0))
        / MAZE_HEIGHT as f32;

    generator::generate_paths(&mut maze);
    solver::find_solution(&mut maze);

    for i in 0..=MAZE_WIDTH {
        meshes.push(graphics::Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(
                i as f32 / MAZE_WIDTH as f32 * (WINDOW_WIDTH - WALL_WIDTH),
                0.0,
                WALL_WIDTH,
                WINDOW_HEIGHT,
            ),
            Color::WHITE
        )?);
    }
    for i in 0..=MAZE_HEIGHT {
        meshes.push(graphics::Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.0,
                i as f32 / MAZE_HEIGHT as f32 * (WINDOW_HEIGHT - WALL_WIDTH),
                WINDOW_WIDTH,
                WALL_WIDTH,
            ),
            Color::WHITE
        )?);
    }

    let mut temp_solution_meshes = Vec::new();
    for i in 0..maze.num_connections() {
        let connection = maze.get_connection(i);

        let tile_corner_x = WALL_WIDTH * (connection.get_start().x as f32 + 1.0)
        + tile_width * connection.get_start().x as f32;
        let tile_corner_y = WALL_WIDTH * (connection.get_start().y as f32 +1.0)
        + tile_height * connection.get_start().y as f32;

        if connection.is_included() {
            let rect = graphics::Rect::new(
                tile_corner_x,
                tile_corner_y,
                match connection.get_dir() {
                    Direction::Right => tile_width + WALL_WIDTH,
                    Direction::Down => tile_width,
                },
                match connection.get_dir() {
                    Direction::Right => tile_height,
                    Direction::Down => tile_height + WALL_WIDTH,
                }
            );

            meshes.push(graphics::Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(),
                rect,
                Color::BLACK
            )?);
        }
        if connection.is_in_solution() {
            let rect = graphics::Rect::new(
                tile_corner_x +
                (tile_width - SOLUTION_WIDTH) / 2.0,
                tile_corner_y +
                (tile_height - SOLUTION_WIDTH) / 2.0,
                match connection.get_dir() {
                    Direction::Right => SOLUTION_WIDTH + tile_width + WALL_WIDTH,
                    Direction::Down => SOLUTION_WIDTH,
                },
                match connection.get_dir() {
                    Direction::Right => SOLUTION_WIDTH,
                    Direction::Down => SOLUTION_WIDTH + tile_height + WALL_WIDTH,
                }
            );

            temp_solution_meshes.push(graphics::Mesh::new_rounded_rectangle(
                ctx, graphics::DrawMode::fill(),
                rect,
                SOLUTION_WIDTH / 2.0,
                Color::GREEN
            )?);
        }
    }
    for m in temp_solution_meshes {
        meshes.push(m);
    }

    let start_wall_height = WALL_WIDTH * (maze.get_start().y as f32 + 1.0)
    + tile_height * maze.get_start().y as f32;
    meshes.push(graphics::Mesh::new_rectangle(
        ctx, graphics::DrawMode::fill(),
        graphics::Rect::new(
            0.0,
            start_wall_height,
            WALL_WIDTH, tile_height
        ),
        Color::BLACK
    )?);
    meshes.push(graphics::Mesh::new_rectangle(
        ctx, graphics::DrawMode::fill(),
        graphics::Rect::new(
            0.0,
            start_wall_height + (tile_height - SOLUTION_WIDTH) / 2.0,
            WALL_WIDTH + tile_width / 2.0,
            SOLUTION_WIDTH
        ),
        Color::GREEN
    )?);

    let end_wall_height = WALL_WIDTH * (maze.get_end().y as f32 + 1.0)
    + tile_height * maze.get_end().y as f32;
    meshes.push(graphics::Mesh::new_rectangle(
        ctx, graphics::DrawMode::fill(),
        graphics::Rect::new(
            WINDOW_WIDTH - WALL_WIDTH,
            end_wall_height,
            WALL_WIDTH, tile_height
        ),
        Color::BLACK
    )?);
    meshes.push(graphics::Mesh::new_rectangle(
        ctx, graphics::DrawMode::fill(),
        graphics::Rect::new(
            WINDOW_WIDTH - WALL_WIDTH - tile_width / 2.0,
            end_wall_height + (tile_height - SOLUTION_WIDTH) / 2.0,
            WALL_WIDTH + tile_width / 2.0,
            SOLUTION_WIDTH
        ),
        Color::GREEN
    )?);

    Ok(meshes)
}