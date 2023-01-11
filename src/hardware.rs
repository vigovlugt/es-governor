use crate::utils::run_command;
use log::debug;

#[derive(Debug, Clone, Default)]
pub struct Hardware {
    pub little: Cores,
    pub big: Cores,
}
impl Hardware {
    pub fn new(little: Cores, big: Cores) -> Hardware {
        Hardware { little, big }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Cores {
    pub name: String,
    pub frequency_policy: i32,
    pub frequency: i32,
}
impl Cores {
    pub fn new(name: String, frequency_policy: i32) -> Cores {
        Cores {
            name,
            frequency_policy,
            frequency: 0,
        }
    }

    pub fn set_frequency(&mut self, freq: i32) {
        debug!("Setting Frequency of {} Cores to {}", self.name, freq);
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
