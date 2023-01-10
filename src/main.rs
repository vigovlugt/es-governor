mod consts;
mod hardware;
mod pipe_all;
mod strategies;
mod utils;

use std::env;

use consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES};
use hardware::Hardware;
use pipe_all::PipeAll;
use utils::run_command;

use crate::{
    hardware::Cores,
    strategies::{demo::DemoStrategy, Strategy, StrategyContext},
};

fn setup_governor(hardware: &mut Hardware) {
    // Export OpenCL library path
    run_command("export LD_LIBRARY_PATH=/data/local/Working_dir")
        .expect("Failed to export library path");
    env::set_var("LD_LIBRARY_PATH", "/data/local/Working_dir");

    // Setup Performance Governor (CPU)
    run_command("echo performance > /sys/devices/system/cpu/cpufreq/policy0/scaling_governor")
        .expect("Failed to set governor of little cores");
    run_command("echo performance > /sys/devices/system/cpu/cpufreq/policy2/scaling_governor")
        .expect("Failed to set governor of big cores");

    // Initialize Little and Big CPU with Lowest Frequency
    hardware.little.set_frequency(LITTLE_FREQUENCIES[0]);
    hardware.big.set_frequency(BIG_FREQUENCIES[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        println!("Wrong number of input arguments.");
        return;
    }

    let graph = &args[1];
    let partitions = args[2].parse::<i32>().unwrap();
    let target_fps = args[3].parse::<i32>().unwrap();
    let target_latency = args[4].parse::<i32>().unwrap();

    let mut hardware = Hardware::new(
        Cores::new("Little".to_string(), 0),
        Cores::new("Big".to_string(), 2),
    );
    let pipe_all = PipeAll::new();

    // Checking if processor is available
    assert!(run_command("command -v true").unwrap().success());
    setup_governor(&mut hardware);

    // Setup strategy
    let mut ctx = StrategyContext {
        graph: graph.to_owned(),
        partitions,
        target_fps,
        target_latency,
        pipe_all,
        hardware,
    };

    let strategy = DemoStrategy::new();

    // Run strategy
    let result = strategy.run(&mut ctx);

    // Handle strategy result
    match result {
        Some(result) => {
            println!(
                "Solution Was Found.\n TargetBigFrequency:{} \t TargetLittleFrequency:{} \t PartitionPoint1:{} \t PartitionPoint2:{} \t Order:{}\n",
                result.hardware.big.frequency,
                result.hardware.little.frequency,
                result.args.partition_point1,
                result.args.partition_point2,
                result.args.order
            );
        }
        None => {
            println!("No Solution Found");
        }
    }
}
