use std::collections::HashMap;
use std::path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
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
    let mut data = HashMap::<String, Stats>::new();

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

    for (city, stats) in data.into_iter() {
        let avg = stats.sum / stats.count;
        println!("{city}: {}/{}/{avg}", stats.min, stats.max);
    }
}
