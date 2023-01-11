use crate::{
    consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES},
    pipe_all::{PipeAllArgs, PipeAllResult},
};
use serde::{Deserialize, Serialize};

use super::{Strategy, StrategyContext, StrategyResult};
use log::info;

pub struct PerformanceBenchmarkStrategy {}

impl PerformanceBenchmarkStrategy {
    pub fn new() -> PerformanceBenchmarkStrategy {
        PerformanceBenchmarkStrategy {}
    }
}

/* What we want to benchmark:
 * graph_mobilene
 * Each of 6 component orders (L, B, G)
 * Big frequencies: [500000, 1000000, 1512000, 2016000, 2208000]
 * Little frequencies: [500000, 1000000, 1398000, 1800000]
 * partitions? (28 parts)
 */

#[derive(Serialize, Deserialize, Debug)]
struct PerformanceBenchmarkResult {
    component: String,
    frequency: i32,
    result: PipeAllResult,
}

impl PerformanceBenchmarkResult {
    pub fn new(
        component: String,
        frequency: i32,
        result: PipeAllResult,
    ) -> PerformanceBenchmarkResult {
        PerformanceBenchmarkResult {
            component,
            frequency,
            result,
        }
    }
}

impl Strategy for PerformanceBenchmarkStrategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult> {
        let partition_point1 = ctx.partitions;
        let partition_point2 = ctx.partitions;
        let n_frames = 60;

        let little_frequencies = LITTLE_FREQUENCIES;
        let big_frequencies = BIG_FREQUENCIES;

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
            let pipe_all_result = ctx.pipe_all.run(&args);
            results.push(PerformanceBenchmarkResult::new(
                "L".to_owned(),
                ctx.hardware.little.frequency,
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
            let pipe_all_result = ctx.pipe_all.run(&args);
            results.push(PerformanceBenchmarkResult::new(
                "B".to_owned(),
                ctx.hardware.big.frequency,
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
            let pipe_all_result = ctx.pipe_all.run(&args);
            results.push(PerformanceBenchmarkResult::new(
                "G".to_owned(),
                0,
                pipe_all_result,
            ));
            info!("Finished benchmarking GPU");
        }

        let output_str = serde_json::to_string(&results).unwrap();

        std::fs::write("benchmark.json", output_str).unwrap();

        Some(StrategyResult::default())
    }
}
