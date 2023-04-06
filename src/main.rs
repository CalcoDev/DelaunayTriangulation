use piston::input::{RenderEvent, UpdateEvent};
use simulation::SimulationSettings;

mod game;
mod simulation;
mod triangulation;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 720;
const GLOBAL_FRAMERATE: u32 = 60;
const SIMULATION_FRAMERATE: u32 = 60;

fn main() {
    let opengl = opengl_graphics::OpenGL::V3_2;

    let mut window: glutin_window::GlutinWindow =
        piston::WindowSettings::new("Delaunay Triangulation", [WIDTH, HEIGHT])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let simulation_settings = SimulationSettings::new(50.0, 0.2, 5.0, 0.5);
    let mut game = game::Game::new(
        opengl_graphics::GlGraphics::new(opengl),
        WIDTH as f32,
        HEIGHT as f32,
        SIMULATION_FRAMERATE as f32,
        simulation_settings,
    );

    let mut event_settings = piston::EventSettings::new();
    event_settings.ups = GLOBAL_FRAMERATE as u64;
    let mut events = piston::Events::new(event_settings);

    game.init(500);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
    game.outit();
}
