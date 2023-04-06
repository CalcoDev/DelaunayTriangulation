use macroquad::prelude::draw_mesh;

use crate::triangulation::TriangulationPoint;

pub struct Game {
    simulation: crate::simulation::Simulation,
}

impl Game {
    pub fn new(
        width: f32,
        height: f32,
        simulation_framerate: f32,
        simulation_settings: crate::simulation::SimulationSettings,
    ) -> Self {
        Self {
            simulation: crate::simulation::Simulation::new(
                width,
                height,
                simulation_framerate,
                simulation_settings,
            ),
        }
    }

    pub fn init(&mut self, num_points: u32) {
        println!("Simulation started. Press ESC to exit.");

        self.simulation.add_static_point(0.0, 0.0);
        self.simulation.add_static_point(self.simulation.width, 0.0);
        self.simulation
            .add_static_point(self.simulation.width, self.simulation.height);
        self.simulation
            .add_static_point(0.0, self.simulation.height);

        for _ in 0..num_points - 4 {
            self.simulation.add_point(
                rand::random::<f32>() * self.simulation.width,
                rand::random::<f32>() * self.simulation.height,
            );
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.simulation.tick(delta_time);
    }

    pub fn render(&mut self) {
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);

        // Build a mesh from the simulation
        let mut mesh = macroquad::prelude::Mesh {
            vertices: vec![],
            indices: vec![],
            texture: None,
        };

        for point in self.simulation.points.iter() {
            mesh.vertices.push(macroquad::models::Vertex {
                position: macroquad::prelude::vec3(point.x(), point.y(), 0.0),
                color: macroquad::prelude::Color::new(1.0, 1.0, 1.0, 1.0),
                uv: macroquad::prelude::vec2(0.0, 0.0),
            });
        }

        for triangle in self.simulation.triangles.iter() {
            mesh.indices.push(triangle.0 as u16);
            mesh.indices.push(triangle.1 as u16);
            mesh.indices.push(triangle.2 as u16);
        }

        draw_mesh(&mesh);

        // Cannot apply geometry shader lmao
    }

    pub fn outit(&mut self) {
        println!("Simulation ended. Hope you enojyed!");
    }
}
