mod consts;
mod hardware;
mod pipe_all;
mod strategies;
mod utils;

use std::env;

use crate::{
    hardware::Cores,
    strategies::{
        demo::DemoStrategy, performance_benchmark::PerformanceBenchmarkStrategy, Strategy,
        StrategyContext,
    },
};
use consts::{BIG_FREQUENCIES, LITTLE_FREQUENCIES};
use hardware::Hardware;
use log::{error, info};
use pipe_all::PipeAll;
use std::io::Write;
use utils::run_command;

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
    hardware
        .little
        .set_frequency(LITTLE_FREQUENCIES[LITTLE_FREQUENCIES.len() - 1]);
    hardware
        .big
        .set_frequency(BIG_FREQUENCIES[BIG_FREQUENCIES.len() - 1]);
}

fn main() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .init();

    //format_timestamp_secs()

    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        error!("Wrong number of input arguments.");
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

    let strategy = PerformanceBenchmarkStrategy::new();

    // Run strategy
    info!("Starting strategy");
    let result = strategy.run(&mut ctx);

    // Handle strategy result
    match result {
        Some(result) => {
            info!(
                "Solution Was Found.\n TargetBigFrequency:{} \t TargetLittleFrequency:{} \t PartitionPoint1:{} \t PartitionPoint2:{} \t Order:{}\n",
                result.hardware.big.frequency,
                result.hardware.little.frequency,
                result.args.partition_point1,
                result.args.partition_point2,
                result.args.order
            );
            info!("{:#?}", result);
        }
        None => {
            error!("No Solution Found");
        }
    }
}
