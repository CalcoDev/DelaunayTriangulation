use macroquad::{
    miniquad::gl,
    prelude::{draw_mesh, gl_use_default_material, gl_use_material, Color, Material},
};

use crate::triangulation::TriangulationPoint;

pub struct Game {
    simulation: crate::simulation::Simulation,
    colours: Vec<macroquad::prelude::Color>,
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
                width + 90.0,
                height + 90.0,
                simulation_framerate,
                simulation_settings,
            ),
            colours: vec![],
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

        for _ in 0..self.simulation.points.len() {
            self.colours.push(macroquad::prelude::Color::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
                1.0,
            ));
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.simulation.tick(delta_time);
    }

    pub fn render(&mut self, material: &Material, point_col: Color) {
        macroquad::prelude::clear_background(macroquad::prelude::BLACK);

        let mut mesh = macroquad::prelude::Mesh {
            vertices: vec![],
            indices: vec![],
            texture: None,
        };

        for (i, point) in self.simulation.points.iter().enumerate() {
            mesh.vertices.push(macroquad::models::Vertex {
                position: macroquad::prelude::vec3(point.x(), point.y(), 0.0),
                color: self.colours[i],
                uv: macroquad::prelude::vec2(0.0, 0.0),
            });
        }

        for triangle in self.simulation.triangles.iter() {
            mesh.indices.push(triangle.0 as u16);
            mesh.indices.push(triangle.1 as u16);
            mesh.indices.push(triangle.2 as u16);
        }

        // Move each mesh vertex so that it's in the center of the screen
        for vertex in mesh.vertices.iter_mut() {
            vertex.position.x -= 60.0;
            vertex.position.y -= 60.0;
        }

        gl_use_material(*material);
        draw_mesh(&mesh);
        gl_use_default_material();

        for (i, point) in self.simulation.points.iter().enumerate() {
            macroquad::prelude::draw_circle(
                point.x() - 60.0,
                point.y() - 60.0,
                self.simulation.points[i].size * 2.5,
                point_col,
            );
        }
    }

    pub fn outit(&mut self) {
        println!("Simulation ended. Hope you enojyed!");
    }
}
