use macroquad::miniquad::{gl, Shader};
use macroquad::prelude::{
    draw_mesh, gl_use_default_material, gl_use_material, vec2, vec3, Color, KeyCode, MaterialParams,
};
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::{models, prelude, window};
use simulation::SimulationSettings;

mod game;
mod simulation;
mod triangulation;

use game::Game;

const WIDTH: i32 = 1080;
const HEIGHT: i32 = 720;
const SIMULATION_FRAMERATE: u32 = 60;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Delaunay Triangulation".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let simulation_settings = SimulationSettings::new(50.0, 0.2, 5.0, 0.5);
    let mut game = Game::new(
        WIDTH as f32,
        HEIGHT as f32,
        SIMULATION_FRAMERATE as f32,
        simulation_settings,
    );
    game.init(500);

    let vert = String::from_utf8(prelude::load_file("assets/shader.vert").await.unwrap()).unwrap();
    let frag = String::from_utf8(prelude::load_file("assets/shader.frag").await.unwrap()).unwrap();
    let geom = String::from_utf8(prelude::load_file("assets/shader.geom").await.unwrap()).unwrap();

    let mut last_update = 0.0f32;
    let simulation_frame_increment = 1.0 / SIMULATION_FRAMERATE as f32;
    loop {
        if prelude::is_key_pressed(KeyCode::Escape) {
            break;
        }

        let current_time = macroquad::time::get_time() as f32;
        while current_time - last_update > simulation_frame_increment {
            game.update(simulation_frame_increment);
            last_update += simulation_frame_increment;
        }

        game.render();

        window::next_frame().await;
    }

    game.outit();
}
