use std::collections::HashMap;
use std::path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let mut data = HashMap::<String, f32>::new();

    let path = "../data/weather_stations.csv";
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    for line in f.lines().flatten().take(10000) {
        if let Some((city, temp)) = line.split_once(';') {
            let temp = temp.parse().unwrap();

            let city_data = data.entry(city.to_string()).or_insert(temp);

            *city_data = temp.max(*city_data);
        } else {
            continue;
        };
    }

    for (city, max_temp) in data.into_iter() {
        println!("{city}: {max_temp}");
    }
}
