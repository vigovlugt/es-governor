use std::env;
use std::process::Command;

const LITTLE_FREQUENCIES: [i32; 9] = [
    500000, 667000, 1000000, 1200000, 1398000, 1512000, 1608000, 1704000, 1800000,
];
const BIG_FREQUENCIES: [i32; 13] = [
    500000, 667000, 1000000, 1200000, 1398000, 1512000, 1608000, 1704000, 1800000, 1908000,
    2016000, 2100000, 2208000,
];

fn extract(line: &str, prefix: &str) -> Option<f32> {
    if !line.starts_with(prefix) {
        return None;
    }

    for word in line.split_whitespace() {
        if let Ok(parsed) = word.parse::<f32>() {
            return Some(parsed);
        }
    }

    None
}

fn run_command(command: &str) -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new("sh").arg("-c").arg(command).status()
}

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

struct Hardware {
    little: Cores,
    big: Cores,
}
impl Hardware {
    fn new(little: Cores, big: Cores) -> Hardware {
        Hardware { little, big }
    }
}

struct Cores {
    name: String,
    frequency_policy: i32,
    frequency: i32,
}
impl Cores {
    fn new(name: String, frequency_policy: i32) -> Cores {
        Cores {
            name,
            frequency_policy,
            frequency: 0,
        }
    }

    fn set_frequency(&mut self, freq: i32) {
        println!("Increasing Frequency of {} Cores to {}", self.name, freq);
        run_command(&format!(
            "echo {} > /sys/devices/system/cpu/cpufreq/policy{}/scaling_max_freq",
            freq, self.frequency_policy
        ))
        .expect(&format!(
            "Failed to increase the frequency of the {} cores",
            self.name
        ));

        self.frequency = freq;
    }
}

struct PipeAllArgs {
    graph: String,
    n_frames: i32,
    partition_point1: i32,
    partition_point2: i32,
    order: String,
}
struct PipeAllResults {
    fps: f32,
    latency: f32,
    stage_one_inference_time: f32,
    stage_two_inference_time: f32,
    stage_three_inference_time: f32,
}

impl PipeAllResults {
    fn parse_results(output: String) -> PipeAllResults {
        let mut fps: f32 = 0.0;
        let mut latency: f32 = 0.0;
        let mut stage_one_inference_time: f32 = 0.0;
        let mut stage_two_inference_time: f32 = 0.0;
        let mut stage_three_inference_time: f32 = 0.0;

        for line in output.lines() {
            let line = line.trim();

            fps = extract(line, "Frame rate is:").unwrap_or(fps);
            latency = extract(line, "Frame latency is:").unwrap_or(latency);

            stage_one_inference_time =
                extract(line, "stage1_inference_time:").unwrap_or(stage_one_inference_time);
            stage_two_inference_time =
                extract(line, "stage2_inference_time:").unwrap_or(stage_two_inference_time);
            stage_three_inference_time =
                extract(line, "stage3_inference_time:").unwrap_or(stage_three_inference_time);
        }

        println!("Throughput is: {:.2} FPS", fps);
        println!("Latency is: {:.2} ms", latency);

        return PipeAllResults {
            fps,
            latency,
            stage_one_inference_time,
            stage_two_inference_time,
            stage_three_inference_time,
        };
    }
}

struct PipeAll {}

impl PipeAll {
    fn new() -> PipeAll {
        PipeAll {}
    }

    fn run(&self, args: &PipeAllArgs) -> PipeAllResults {
        let cmd = format!(
            "./{} --threads=4 --threads2=2 --target=NEON --n={} --partition_point={} --partition_point2={} --order={} > output.txt",
            args.graph, args.n_frames, args.partition_point1, args.partition_point2, args.order
        );
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to run the command");
        let output_str = String::from_utf8(output.stdout).unwrap();

        return PipeAllResults::parse_results(output_str);
    }
}

struct StrategyContext {
    graph: String,
    partitions: i32,
    target_fps: i32,
    target_latency: i32,
    pipe_all: PipeAll,
    hardware: Hardware,
}

struct StrategyResult {
    args: PipeAllArgs,
    results: PipeAllResults,
}

trait Strategy {
    fn run(&self, ctx: &mut StrategyContext) -> Option<StrategyResult>;

    fn is_valid_result(ctx: &StrategyContext, fps: f32, latency: f32) -> bool {
        return fps >= ctx.target_fps as f32 && latency <= ctx.target_latency as f32;
    }
}

struct DemoStrategy {}
impl DemoStrategy {
    fn new() -> DemoStrategy {
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
                return Some(StrategyResult { args, results });
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
                ctx.hardware.big.frequency,
                ctx.hardware.little.frequency,
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
