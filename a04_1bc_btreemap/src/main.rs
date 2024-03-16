use std::collections::BTreeMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Stats {
    min: f32,
    max: f32,
    sum: f32,
    count: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
            sum: Default::default(),
            count: Default::default(),
        }
    }
}

fn main() {
    let mut data = BTreeMap::<String, Stats>::new();

    let path = "../data/weather_stations.csv";
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    for line in f.lines().flatten().take(10000) {
        if let Some((city, temp)) = line.split_once(';') {
            let temp: f32 = temp.parse().unwrap();

            // let city_data = data.entry(city.to_string()).or_insert(Default::default());
            let city_data = data.entry(city.to_string()).or_default();

            city_data.min = temp.min(city_data.min);
            city_data.max = temp.max(city_data.max);
            city_data.sum += temp;
            city_data.count += 1.0;
        } else {
            continue;
        };
    }

    // let mut data = data.into_iter().collect::<Vec<_>>();
    // data.sort_unstable_by(|city_a, city_b| (city_a.0).cmp(&(city_b.0)));

    for (city, stats) in data.into_iter() {
        let avg: f32 = if stats.count == -0.0 {
            0.0
        } else {
            stats.sum / stats.count
        };
        println!("{city}: {}/{}/{avg}", stats.min, stats.max);
    }
}
