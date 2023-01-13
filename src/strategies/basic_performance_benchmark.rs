use crate::{
    consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES},
    pipe_all::{PipeAllArgs, PipeAllResult},
};
use serde::{Deserialize, Serialize};

use super::{Strategy, StrategyContext, StrategyResult};
use log::info;

#[derive(Debug, Default)]
pub struct BasicPerformanceBenchmarkStrategy {}

#[derive(Serialize, Deserialize, Debug)]
struct PerformanceBenchmarkResult {
    component: String,
    frequency: i32,
    n_frames: i32,
    result: PipeAllResult,
}

impl PerformanceBenchmarkResult {
    pub fn new(
        component: String,
        frequency: i32,
        n_frames: i32,
        result: PipeAllResult,
    ) -> PerformanceBenchmarkResult {
        PerformanceBenchmarkResult {
            component,
            frequency,
            n_frames,
            result,
        }
    }
}

impl Strategy for BasicPerformanceBenchmarkStrategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult> {
        let partition_point1 = ctx.partitions;
        let partition_point2 = ctx.partitions;

        let little_frequencies = LITTLE_FREQUENCIES;
        let big_frequencies = BIG_FREQUENCIES;

        let n_frames = 90;

        let mut results: Vec<PerformanceBenchmarkResult> = Vec::new();

        for little_frequency in little_frequencies {
            ctx.hardware.little.set_frequency(little_frequency);
            let args = PipeAllArgs {
                graph: ctx.graph.to_string(),
                n_frames,
                partition_point1,
                partition_point2,
                order: "L-B-G".to_string(),
            };
            info!("Starting benchmark for L frequency: {}", little_frequency);
            let pipe_all_result = ctx.pipe_all.run(&args);
            results.push(PerformanceBenchmarkResult::new(
                "L".to_owned(),
                ctx.hardware.little.frequency,
                n_frames,
                pipe_all_result,
            ));
            info!("Finished benchmarking L frequency: {}", little_frequency);
        }

        for big_frequency in big_frequencies {
            ctx.hardware.big.set_frequency(big_frequency);
            let args = PipeAllArgs {
                graph: ctx.graph.to_string(),
                n_frames,
                partition_point1,
                partition_point2,
                order: "B-L-G".to_string(),
            };
            info!("Starting benchmark for B frequency: {}", big_frequency);
            let pipe_all_result = ctx.pipe_all.run(&args);
            results.push(PerformanceBenchmarkResult::new(
                "B".to_owned(),
                ctx.hardware.big.frequency,
                n_frames,
                pipe_all_result,
            ));
            info!("Finished benchmarking B frequency: {}", big_frequency);
        }

        {
            let args = PipeAllArgs {
                graph: ctx.graph.to_string(),
                n_frames,
                partition_point1,
                partition_point2,
                order: "G-B-L".to_string(),
            };
            info!("starting gpu benchmark");
            let pipe_all_result = ctx.pipe_all.run(&args);
            results.push(PerformanceBenchmarkResult::new(
                "G".to_owned(),
                0,
                n_frames,
                pipe_all_result,
            ));
            info!("Finished benchmarking GPU");
        }

        let output_str = serde_json::to_string(&results).unwrap();

        std::fs::write("benchmark.json", output_str).unwrap();

        Some(StrategyResult::default())
    }
}
