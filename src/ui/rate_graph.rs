use egui_plot::Line;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct RateGraph {
    points: VecDeque<[f64; 2]>,
    max_points: usize,
    last_time: f64,
    time_increment: f64,
    name: String
}

impl RateGraph {
    pub fn new(name: &str, max_points: &usize) -> Self {
        Self {points: VecDeque::new(), max_points: *max_points, last_time: 0.0, time_increment: 2.0, name: String::from(name)}
    }

    pub fn add_point(&mut self, rate: f64) {
        if self.points.len() == self.max_points {
            self.points.pop_front();
        }
        let current_time = self.last_time + self.time_increment;
        self.points.push_back([rate, current_time]);
        self.last_time = current_time;
    }

    pub fn get_points_to_draw(&self) -> Line {
        let graph = Line::new(self.points.clone().into_iter().collect::<Vec<[f64; 2]>>()).name(&self.name);
        return graph;
    }

    pub fn reset(&mut self) {
        self.points.clear();
        self.last_time = 0.0;
    }

    pub fn change_max_points(&mut self, max_points: &usize) {
        self.max_points = *max_points;
        self.reset();
    }
}