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

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const SIMULATION_FRAMERATE: u32 = 120;

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

    let vert = prelude::load_file("assets/shader.vert").await.unwrap();
    let frag = prelude::load_file("assets/shader.frag").await.unwrap();

    let material = prelude::load_material(
        String::from_utf8(vert).unwrap().as_ref(),
        String::from_utf8(frag).unwrap().as_ref(),
        MaterialParams {
            uniforms: vec![
                ("gradient_a".to_string(), prelude::UniformType::Float3),
                ("gradient_b".to_string(), prelude::UniformType::Float3),
            ],
            ..Default::default()
        },
    )
    .unwrap();

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

        let grad_a: [u8; 3] = [255, 242, 211];
        let grad_b: [u8; 3] = [11, 70, 82];
        let point_colour: [u8; 3] = [245, 240, 228];

        material.set_uniform("gradient_a", grad_b.map(|x| x as f32 / 255.0));
        material.set_uniform("gradient_b", grad_a.map(|x| x as f32 / 255.0));

        game.render(
            &material,
            Color::new(
                point_colour[0] as f32 / 255.0,
                point_colour[1] as f32 / 255.0,
                point_colour[2] as f32 / 255.0,
                1.0,
            ),
        );

        // Draw the FPS counter
        // let fps = macroquad::time::get_fps();
        // let fps_text = format!("FPS: {}", fps);
        // prelude::draw_text(&fps_text, 10.0, 60.0, 60.0, Color::new(0.0, 1.0, 0.0, 1.0));

        window::next_frame().await;
    }

    game.outit();
}
