use macroquad::prelude::Vec2;

use crate::triangulation;

#[derive(Debug)]
struct VariedValue {
    value: f32,
    variance: f32,
}

impl VariedValue {
    fn new(value: f32, variance: f32) -> Self {
        Self { value, variance }
    }

    fn compute_random(&self) -> f32 {
        self.value + (self.variance * (rand::random::<f32>() - 0.5f32))
    }
}

#[derive(Debug)]
pub struct SimulationSettings {
    point_speed: VariedValue,
    point_time: VariedValue,
}

impl SimulationSettings {
    pub fn new(
        point_speed: f32,
        point_speed_variance: f32,
        point_time: f32,
        point_time_variance: f32,
    ) -> Self {
        Self {
            point_speed: VariedValue::new(point_speed, point_speed_variance),
            point_time: VariedValue::new(point_time, point_time_variance),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub position: Vec2,
    pub size: f32,
    speed: f32,
    dir_timer: f32,
    dir: Vec2,
}

impl triangulation::TriangulationPoint for Point {
    fn x(&self) -> f32 {
        self.position.x
    }

    fn y(&self) -> f32 {
        self.position.y
    }
}

pub struct Simulation {
    pub width: f32,
    pub height: f32,
    pub framerate: f32,

    last_tick: f32,
    current_tick: f32,

    settings: SimulationSettings,
    pub points: Vec<Point>,
    pub triangles: Vec<triangulation::Triangle>,
}

impl Simulation {
    pub fn new(
        width: f32,
        height: f32,
        framerate: f32,
        settings: SimulationSettings,
    ) -> Simulation {
        Simulation {
            width,
            height,
            framerate,

            last_tick: 0.0,
            current_tick: 0.0,

            settings,
            points: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push(Point {
            position: Vec2::new(x, y),
            size: Self::get_random_point_size(),
            speed: self.settings.point_speed.compute_random(),
            dir_timer: self.settings.point_time.compute_random(),
            dir: Self::get_random_point_dir(),
        });
    }

    pub fn add_static_point(&mut self, x: f32, y: f32) {
        self.points.push(Point {
            position: Vec2::new(x, y),
            size: Self::get_random_point_size(),
            speed: 0.0,
            dir_timer: f32::MAX,
            dir: Vec2::new(0.0, 0.0),
        });
    }

    pub fn tick(&mut self, delta: f32) {
        self.current_tick += delta;

        let frame_increment = 1f32 / self.framerate;
        const MAX_FRAMES: u32 = 10;
        let mut frames = 0;
        while (self.current_tick - self.last_tick) >= frame_increment && frames < MAX_FRAMES {
            self.last_tick += frame_increment;
            frames += 1;

            self.update_points(frame_increment);
            self.triangulate();
        }
    }
}

impl Simulation {
    fn update_points(&mut self, delta: f32) {
        for point in self.points.iter_mut() {
            point.position.x += point.dir.x * (point.speed * delta);
            point.position.y += point.dir.y * (point.speed * delta);

            if point.position.x < 0f32 {
                point.position.x += self.width;
            } else if point.position.x > self.width {
                point.position.x -= self.width;
            }

            if point.position.y < 0f32 {
                point.position.y += self.height;
            } else if point.position.y > self.height {
                point.position.y -= self.height;
            }

            point.dir_timer -= delta;
            if point.dir_timer <= 0f32 {
                point.dir = Self::get_random_point_dir();
                point.dir_timer = self.settings.point_time.compute_random();
            }
        }
    }

    fn triangulate(&mut self) {
        self.triangles = triangulation::triangulate(&self.points);
    }

    fn get_random_point_dir() -> Vec2 {
        // let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0f32;
        let angle =
            rand::random::<f32>() * std::f32::consts::PI * 0.5 + std::f32::consts::PI * 0.25;
        Vec2::new(angle.cos(), (-angle).sin())
    }

    fn get_random_point_size() -> f32 {
        let size = rand::random::<f32>() + 0.5f32;
        size
    }
}
