#![windows_subsystem = "windows"]

mod generator;
mod solver;
mod renderer;
mod maze;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
    conf::{WindowSetup, WindowMode},
    input::keyboard::{KeyCode, KeyInput}
};

struct MainState {
    meshes: Vec<graphics::Mesh>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            meshes: renderer::create_meshes(ctx)?,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, Color::BLACK);

        for mesh in &self.meshes {
            canvas.draw(mesh, Vec2::new(0.0, 0.0));
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(keycode) => match keycode {
                KeyCode::Space => self.meshes = renderer::create_meshes(ctx)?,
                _ => {}
            }
            None => {}
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("maze_generator", "Daniel Petkau");
    let ws = WindowSetup {
        title: "Maze Generator".to_owned(),
        ..WindowSetup::default()
    };
    let wm = WindowMode {
        width: renderer::WINDOW_WIDTH,
        height: renderer::WINDOW_HEIGHT,
        ..WindowMode::default()
    };
    let (mut ctx, event_loop) = cb.window_setup(ws).window_mode(wm).build()?;
    let ms = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, ms)
}