use std::process::Command;

pub struct PipeAllArgs {
    pub graph: String,
    pub n_frames: i32,
    pub partition_point1: i32,
    pub partition_point2: i32,
    pub order: String,
}

pub struct PipeAllResults {
    pub fps: f32,
    pub latency: f32,
    pub stage_one_inference_time: f32,
    pub stage_two_inference_time: f32,
    pub stage_three_inference_time: f32,
}

impl PipeAllResults {
    pub fn parse_results(output: String) -> PipeAllResults {
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

pub struct PipeAll {}

impl PipeAll {
    pub fn new() -> PipeAll {
        PipeAll {}
    }

    pub fn run(&self, args: &PipeAllArgs) -> PipeAllResults {
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
