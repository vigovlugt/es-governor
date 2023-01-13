pub mod basic_performance_benchmark;
pub mod demo;
pub mod performance_benchmark;
pub mod performance_stage_benchmark;

use crate::{
    hardware::Hardware,
    pipe_all::{PipeAll, PipeAllArgs, PipeAllResult},
};

pub struct StrategyContext {
    pub graph: String,
    pub partitions: i32,
    // Frames per second
    pub target_fps: i32,
    // Milliseconds
    pub target_latency: i32,
    pub pipe_all: PipeAll,
    pub hardware: Hardware,
}

#[derive(Default, Debug)]
pub struct StrategyResult {
    pub args: PipeAllArgs,
    pub hardware: Hardware,
    pub results: PipeAllResult,
}

pub trait Strategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult>;

    fn is_valid_result(&self, ctx: &StrategyContext, fps: f32, latency: f32) -> bool {
        return fps >= ctx.target_fps as f32 && latency <= ctx.target_latency as f32;
    }
}

pub fn get_strategy_by_name(name: &str) -> Option<Box<dyn Strategy>> {
    match name {
        "demo" => Some(Box::new(demo::DemoStrategy::default())),
        "perf" => Some(Box::new(
            performance_benchmark::PerformanceBenchmarkStrategy::default(),
        )),
        "perf-stage" => Some(Box::new(
            performance_stage_benchmark::PerformanceStageBenchmarkStrategy::default(),
        )),
        "perf-basic" => Some(Box::new(
            basic_performance_benchmark::BasicPerformanceBenchmarkStrategy::default(),
        )),
        _ => None,
    }
}
