#[derive(Debug, Clone, Copy)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn number_of_winning_combos(&self) -> u64 {
        // (time - hold_time) * hold_time = distance
        let min_hold = self.min_hold_time();
        let max_hold = self.max_hold_time();

        max_hold - min_hold + 1
    }

    fn min_hold_time(&self) -> u64 {
        let t = self.time as f64;
        // need to beat distance, not match it
        let d = self.distance as f64 + 1.0;

        // hold_time = (time - sqrt(time^2 - 4*distance)) / 2
        let min = (t - (t.powi(2) - (d * 4.0)).sqrt()) / 2.0;
        min.ceil() as u64
    }

    fn max_hold_time(&self) -> u64 {
        let t = self.time as f64;
        // need to beat distance, not match it
        let d = self.distance as f64 + 1.0;

        // hold_time = (sqrt(time^2 - 4*distance) + time) / 2
        let max = ((t.powi(2) - (d * 4.0)).sqrt() + t) / 2.0;
        max.floor() as u64
    }
}

pub fn parse_day6_input(input: &str) -> Vec<Race> {
    let (time, distance) = input.split_once('\n').expect("failed to parse input");

    let (_, times) = time.split_once(':').expect("failed to split time line");
    let (_, distances) = distance
        .split_once(':')
        .expect("failed to split distance line");

    let parsed_times = times.split_ascii_whitespace().map(|t| t.parse().unwrap());
    let parsed_distances = distances
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap());

    parsed_times
        .zip(parsed_distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

pub fn parse_day6_input_pt2(input: &str) -> Race {
    let (time, distance) = input.split_once('\n').expect("failed to parse input");

    let (_, time) = time.split_once(':').expect("failed to split time line");
    let (_, distance) = distance
        .split_once(':')
        .expect("failed to split distance line");

    let time = time.replace([' ', '\n'], "").parse().unwrap();
    let distance = distance.replace([' ', '\n'], "").parse().unwrap();

    Race { time, distance }
}
