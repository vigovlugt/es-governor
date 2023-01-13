use crate::{
    consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES},
    hardware::Component,
    pipe_all::{PipeAllArgs, PipeAllResult},
};
use serde::{Deserialize, Serialize};

use super::{Strategy, StrategyContext, StrategyResult};
use log::info;

#[derive(Debug, Default)]
pub struct PerformanceStageBenchmarkStrategy {}

#[derive(Serialize, Deserialize, Debug)]
struct PerformanceStageBenchmarkResult {
    component: String,
    frequency: i32,
    stage: i32,
    partition_point1: i32,
    partition_point2: i32,
    result: PipeAllResult,
}

impl PerformanceStageBenchmarkResult {
    pub fn new(
        component: String,
        frequency: i32,
        result: PipeAllResult,
        partition: i32,
        stage: i32,
    ) -> PerformanceStageBenchmarkResult {
        PerformanceStageBenchmarkResult {
            component,
            frequency,
            result,
            stage,
            partition_point1: partition,
            partition_point2: partition + 1,
        }
    }
    pub fn new_wide(
        component: String,
        frequency: i32,
        result: PipeAllResult,
        partition1: i32,
        partition2: i32,
        stage: i32,
    ) -> PerformanceStageBenchmarkResult {
        PerformanceStageBenchmarkResult {
            component,
            frequency,
            result,
            stage,
            partition_point1: partition1,
            partition_point2: partition2,
        }
    }
}

impl Strategy for PerformanceStageBenchmarkStrategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult> {
        let little_frequency = LITTLE_FREQUENCIES[LITTLE_FREQUENCIES.len() - 1];
        ctx.hardware.little.set_frequency(little_frequency);
        let big_frequency = BIG_FREQUENCIES[BIG_FREQUENCIES.len() - 1];
        ctx.hardware.big.set_frequency(big_frequency);

        let n_frames = 60;

        let mut results: Vec<PerformanceStageBenchmarkResult> = Vec::new();

        for (component, order, initial_order) in vec![
            (Component::Little, "G-L-B", "L-G-B"),
            (Component::Big, "G-B-L", "B-G-L"),
            (Component::Gpu, "B-G-L", "G-B-L"),
            (Component::Little, "B-L-G", "L-B-G"),
            (Component::Big, "L-B-G", "B-L-G"),
            (Component::Gpu, "L-G-B", "G-L-B"),
        ] {
            for partition1 in 0..ctx.partitions {
                let mut far_point = ctx.partitions + 1;
                if partition1 + 5 < far_point {
                    far_point = partition1 + 5;
                }
                for partition2 in (partition1 + 1)..far_point {
                    let args = PipeAllArgs {
                        graph: ctx.graph.to_string(),
                        n_frames,
                        partition_point1: match partition1 {
                            0 => partition2,
                            x => x,
                        },
                        partition_point2: match partition1 {
                            0 => ctx.partitions,
                            _ => partition2,
                        },
                        order: match partition1 {
                            0 => initial_order.to_string(),
                            _ => order.to_string(),
                        },
                    };
                    let pipe_all_result = ctx.pipe_all.run(&args);
                    results.push(PerformanceStageBenchmarkResult::new_wide(
                        component.to_string(),
                        match component {
                            Component::Little => little_frequency,
                            Component::Big => big_frequency,
                            Component::Gpu => 0,
                        },
                        pipe_all_result,
                        partition1,
                        partition2,
                        match partition1 {
                            0 => 1,
                            _ => 2,
                        },
                    ));
                    info!(
                        "Finished benchmarking {} partition: {} - {}",
                        component.to_string(),
                        partition1,
                        partition2
                    );
                }
            }
        }
        let output_str = serde_json::to_string(&results).unwrap();

        std::fs::write("benchmark.json", output_str).unwrap();

        Some(StrategyResult::default())
    }
}
