use crate::{
    consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES},
    pipe_all::PipeAllArgs,
};

use super::{Strategy, StrategyContext, StrategyResult};

pub struct DemoStrategy {}

impl DemoStrategy {
    pub fn new() -> DemoStrategy {
        DemoStrategy {}
    }
}

impl Strategy for DemoStrategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult> {
        let mut little_frequency_counter = 0;
        let mut big_frequency_counter = 0;

        let mut partition_point1 = ctx.partitions / 2;
        let mut partition_point2 = ctx.partitions / 2;

        let mut order = "L-G-B".to_string();

        loop {
            let n_frames = 10;
            let args = PipeAllArgs {
                graph: ctx.graph.to_string(),
                n_frames,
                partition_point1,
                partition_point2,
                order: order.to_string(),
            };
            let results = ctx.pipe_all.run(&args);
            if Self::is_valid_result(ctx, results.fps, results.latency) {
                // Both Latency and Throughput Requirements are Met.
                return Some(StrategyResult {
                    args,
                    results,
                    hardware: ctx.hardware.clone(),
                });
            }

            println!("Target Performance Not Satisfied\n\n");

            if little_frequency_counter < LITTLE_FREQUENCIES.len() - 1 {
                // Push Frequency of Little Cluster Higher to Meet Target Performance
                little_frequency_counter += 1;
                ctx.hardware
                    .little
                    .set_frequency(LITTLE_FREQUENCIES[little_frequency_counter]);
            } else if big_frequency_counter < BIG_FREQUENCIES.len() - 1 {
                // Push Frequency of Small Cluster Higher to Meet Target Performance
                big_frequency_counter += 1;
                ctx.hardware
                    .big
                    .set_frequency(BIG_FREQUENCIES[big_frequency_counter]);
            } else {
                // All Frequency levels have been tried, now try a different partitioning
                if partition_point1 < ctx.partitions - 1 {
                    partition_point1 += 1;
                } else if partition_point2 < ctx.partitions - 1 {
                    partition_point2 += 1;
                } else if order == "L-G-B" {
                    order = "B-G-L".to_owned();
                } else {
                    return None;
                }
            }
        }
    }
}
