use std::{fs::File, io::BufReader};

fn main() {
    let path = "../data/weather_stations.csv";
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    dbg!(f);

    // for line in f.lines().flatten() {
    //     line.split_once(';');
    // }
}
