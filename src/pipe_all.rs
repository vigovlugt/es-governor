use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Default)]
pub struct PipeAllArgs {
    pub graph: String,
    pub n_frames: i32,
    pub partition_point1: i32,
    pub partition_point2: i32,
    pub order: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PipeAllStageResult {
    pub input_time: f32,
    pub inference_time: f32,
    pub total_time: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PipeAllResult {
    pub fps: f32,
    pub latency: f32,
    pub stage_one: PipeAllStageResult,
    pub stage_two: PipeAllStageResult,
    pub stage_three: PipeAllStageResult,
}

impl PipeAllResult {
    pub fn parse_results(output: String) -> PipeAllResult {
        let mut fps: f32 = 0.0;
        let mut latency: f32 = 0.0;
        let mut stage_one = PipeAllStageResult::default();
        let mut stage_two = PipeAllStageResult::default();
        let mut stage_three = PipeAllStageResult::default();

        for line in output.lines() {
            let line = line.trim();

            fps = extract("Frame rate is:", line).unwrap_or(fps);
            latency = extract("Frame latency is:", line).unwrap_or(latency);

            extract_to_stage("1", &mut stage_one, line);
            extract_to_stage("2", &mut stage_two, line);
            extract_to_stage("3", &mut stage_three, line);
        }

        return PipeAllResult {
            fps,
            latency,
            stage_one,
            stage_two,
            stage_three,
        };
    }
}

pub struct PipeAll {}

impl PipeAll {
    pub fn new() -> PipeAll {
        PipeAll {}
    }

    pub fn run(&self, args: &PipeAllArgs) -> PipeAllResult {
        let cmd = format!(
            "./{} --threads=4 --threads2=2 --target=NEON --n={} --partition_point={} --partition_point2={} --order={}",
            args.graph, args.n_frames, args.partition_point1, args.partition_point2, args.order
        );
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to run the command");
        let output_str = String::from_utf8(output.stdout).unwrap();

        return PipeAllResult::parse_results(output_str);
    }
}

fn extract_to_stage(n: &str, stage: &mut PipeAllStageResult, line: &str) {
    stage.input_time =
        extract(&format!("stage{}_input_time:", n), line).unwrap_or(stage.input_time);
    stage.inference_time =
        extract(&format!("stage{}_inference_time:", n), line).unwrap_or(stage.inference_time);
    stage.total_time =
        extract(&format!("stage{}_total_time:", n), line).unwrap_or(stage.total_time);
}

fn extract(prefix: &str, line: &str) -> Option<f32> {
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
