use crate::{
    consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES},
    pipe_all::PipeAllArgs,
};
use log::debug;

use super::{Strategy, StrategyContext, StrategyResult};

pub struct DemoStrategy {
    order: String,
}

impl DemoStrategy {
    pub fn new(order: String) -> DemoStrategy {
        DemoStrategy { order }
    }

    pub fn default() -> DemoStrategy {
        DemoStrategy {
            order: "L-G-B".to_owned(),
        }
    }
}

impl Strategy for DemoStrategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult> {
        ctx.hardware.little.set_frequency(LITTLE_FREQUENCIES[0]);
        ctx.hardware.big.set_frequency(BIG_FREQUENCIES[0]);

        let mut little_frequency_counter = 0;
        let mut big_frequency_counter = 0;

        let mut partition_point1 = ctx.partitions / 2;
        let mut partition_point2 = ctx.partitions / 2;

        loop {
            let n_frames = 10;
            let args = PipeAllArgs {
                graph: ctx.graph.to_string(),
                n_frames,
                partition_point1,
                partition_point2,
                order: self.order.to_string(),
            };
            let results = ctx.pipe_all.run(&args);
            debug!("{:#?}", results);

            if Self::is_valid_result(ctx, results.fps, results.latency) {
                // Both Latency and Throughput Requirements are Met.
                return Some(StrategyResult {
                    args,
                    results,
                    hardware: ctx.hardware.clone(),
                });
            }

            debug!("Target performance not yet satisfied");

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
                if results.stage_one.inference_time < results.stage_three.inference_time {
                    if partition_point2 < ctx.partitions {
                        /* Push Layers from Third Stage (Big CPU) to GPU to Meet Target Performance */
                        partition_point2 += 1;
                        debug!("Reducing the Size of Pipeline Partition 3")
                    } else {
                        return None;
                    }
                } else {
                    if partition_point1 > 1 {
                        /* Push Layers from First Stage (Little CPU) to GPU to Meet Target Performance */
                        partition_point1 -= 1;
                        debug!("Reducing the Size of Pipeline Partition 1");
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
