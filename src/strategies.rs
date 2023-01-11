pub mod demo;
pub mod performance_benchmark;

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

    fn is_valid_result(ctx: &StrategyContext, fps: f32, latency: f32) -> bool {
        return fps >= ctx.target_fps as f32 && latency <= ctx.target_latency as f32;
    }
}
