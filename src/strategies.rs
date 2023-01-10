pub mod demo;

use crate::{
    hardware::Hardware,
    pipe_all::{PipeAll, PipeAllArgs, PipeAllResults},
};

pub struct StrategyContext {
    pub graph: String,
    pub partitions: i32,
    pub target_fps: i32,
    pub target_latency: i32,
    pub pipe_all: PipeAll,
    pub hardware: Hardware,
}

pub struct StrategyResult {
    pub args: PipeAllArgs,
    pub results: PipeAllResults,
}

pub trait Strategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult>;

    fn is_valid_result(ctx: &StrategyContext, fps: f32, latency: f32) -> bool {
        return fps >= ctx.target_fps as f32 && latency <= ctx.target_latency as f32;
    }
}
