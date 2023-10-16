use super::rate_graph::RateGraph;
use crate::envoy::message::{EmbassyMessage, MessageKind};
use crate::envoy::error::EmbassyError;
use crate::envoy::constants::NUMBER_OF_MODULES;
use crate::envoy::surveyor_envoy::SurveyorResponse;

#[derive(Debug)]
pub struct GraphManager {
    graphs: Vec<RateGraph>,
    max_points: usize
}

impl GraphManager {
    pub fn new(max_points: usize) -> Self {
        let mut graphs: Vec<RateGraph> = vec![];
        for i in 0..(NUMBER_OF_MODULES-1) {
            graphs.push(RateGraph::new(&format!("envoy_{i}"), &max_points));
        }
        return Self { graphs, max_points }
    }

    pub fn handle_messages(&mut self, messages: &[EmbassyMessage]) -> Result<(), EmbassyError> {

        for message in messages {
            match message.kind {
                MessageKind::Surveyor => {
                    if let Some(graph) = self.graphs.get_mut(message.id as usize) {
                        let response: SurveyorResponse = message.try_into()?;
                        graph.add_point(response.data_rate);
                    }
                },
                _ => continue,
            };
        }

        Ok(())
    }

    pub fn get_line_graphs(&self) -> Vec<egui_plot::Line> {
        self.graphs.iter()
            .map(|g| {
                g.get_points_to_draw()
            })
            .collect()
    }

    pub fn reset_graphs(&mut self) {
        for graph in self.graphs.iter_mut() {
            graph.reset();
        }
    }

    pub fn set_max_points(&mut self, max_points: &usize) {
        self.max_points = *max_points;
        for graph in self.graphs.iter_mut() {
            graph.change_max_points(max_points);
        }
    }

    pub fn get_max_points(&self) -> &usize {
        &self.max_points
    }
}