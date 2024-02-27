use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
};

use memmap::Mmap;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Statistics {
    min: f64,
    max: f64,
    sum: f64,
    count: usize,
}

impl Statistics {
    fn new(measurement: f64) -> Self {
        Statistics {
            min: measurement,
            max: measurement,
            sum: measurement,
            count: 1,
        }
    }

    fn average(&self) -> f64 {
        self.sum / self.count as f64
    }

    fn add_measurement(&mut self, measurement: f64) {
        self.count += 1;
        self.min = measurement.min(self.min);
        self.max = measurement.max(self.max);
        self.sum += measurement;
    }

    fn merge(&mut self, other: &Statistics) {
        self.count += other.count;
        self.min = other.min.min(self.min);
        self.max = other.max.max(self.max);
        self.sum += other.sum;
    }
}

fn main() -> anyhow::Result<()> {
    let file = File::options().read(true).open("./measurement_data.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let maps: Vec<HashMap<&[u8], Statistics>> = mmap
        .par_split(|b| b == &b'\n')
        .filter(|buf| !buf.is_empty())
        .fold_with(HashMap::<&[u8], Statistics>::new(), |mut map, buf| {
            let pos = buf.iter().position(|b| b == &b';').unwrap();
            let (station, temperature_u8) = buf.split_at(pos);
            let temperature = std::str::from_utf8(&temperature_u8[1..])
                .unwrap()
                .parse::<f64>()
                .unwrap();

            if let Some(stored_stats) = map.get_mut(station) {
                stored_stats.add_measurement(temperature);
            } else {
                let stats = Statistics::new(temperature);
                map.insert(station, stats);
            }
            map
        })
        .collect();

    let result: BTreeMap<&[u8], Statistics> = maps
        .par_chunks(50)
        .map(|chunk| {
            let mut result: BTreeMap<&[u8], Statistics> = BTreeMap::new();

            for map in chunk.into_iter() {
                map.into_iter().for_each(|(station, stats)| {
                    if let Some(stored_stats) = result.get_mut(station) {
                        stored_stats.merge(stats);
                    } else {
                        result.insert(station, stats.to_owned());
                    }
                });
            }
            result
        })
        .reduce_with(|mut map_left, map_right| {
            map_right.into_iter().for_each(|(station, stats)| {
                if let Some(stored_stats) = map_left.get_mut(station) {
                    stored_stats.merge(&stats);
                } else {
                    map_left.insert(station, stats);
                }
            });
            map_left
        })
        .unwrap();

    for (station, stats) in result {
        println!(
            "{:?}, count: {}, min: {:.2}, max: {:.2}, avg: {:.2}",
            std::str::from_utf8(station)?,
            // std::borrow::Cow::Borrowed(station),
            // String::from_utf8_lossy(station),
            stats.count,
            stats.min,
            stats.max,
            stats.average(),
        );
    }

    Ok(())
}
