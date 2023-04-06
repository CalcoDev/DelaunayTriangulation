use graphics::{clear, rectangle};

pub struct Game {
    gl: opengl_graphics::GlGraphics,
    simulation: crate::simulation::Simulation,
}

impl Game {
    pub fn new(
        gl: opengl_graphics::GlGraphics,
        width: f32,
        height: f32,
        simulation_framerate: f32,
        simulation_settings: crate::simulation::SimulationSettings,
    ) -> Self {
        Self {
            gl,
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

        for _ in 0..num_points {
            self.simulation.add_point(
                rand::random::<f32>() * self.simulation.width,
                rand::random::<f32>() * self.simulation.height,
            );
        }
    }

    pub fn update(&mut self, args: &piston::UpdateArgs) {
        self.simulation.tick(args.dt as f32);
    }

    pub fn render(&mut self, args: &piston::RenderArgs) {
        self.gl.draw(args.viewport(), |ctx, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for point in &self.simulation.points {
                graphics::ellipse(
                    [1.0, 1.0, 1.0, 1.0],
                    graphics::ellipse::circle(
                        point.position.x as f64,
                        point.position.y as f64,
                        point.size as f64,
                    ),
                    ctx.transform,
                    gl,
                );
            }

            for triangle in &self.simulation.triangles {
                graphics::line(
                    [1.0, 1.0, 1.0, 0.1],
                    1.0,
                    [
                        self.simulation.points[triangle.0].position.x as f64,
                        self.simulation.points[triangle.0].position.y as f64,
                        self.simulation.points[triangle.1].position.x as f64,
                        self.simulation.points[triangle.1].position.y as f64,
                    ],
                    ctx.transform,
                    gl,
                );

                graphics::line(
                    [1.0, 1.0, 1.0, 0.1],
                    1.0,
                    [
                        self.simulation.points[triangle.1].position.x as f64,
                        self.simulation.points[triangle.1].position.y as f64,
                        self.simulation.points[triangle.2].position.x as f64,
                        self.simulation.points[triangle.2].position.y as f64,
                    ],
                    ctx.transform,
                    gl,
                );

                graphics::line(
                    [1.0, 1.0, 1.0, 0.1],
                    1.0,
                    [
                        self.simulation.points[triangle.2].position.x as f64,
                        self.simulation.points[triangle.2].position.y as f64,
                        self.simulation.points[triangle.0].position.x as f64,
                        self.simulation.points[triangle.0].position.y as f64,
                    ],
                    ctx.transform,
                    gl,
                );
            }
        })
    }

    pub fn outit(&mut self) {
        println!("Simulation ended. Hope you enojyed!");
    }
}
