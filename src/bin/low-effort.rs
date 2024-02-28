use std::{collections::HashMap, fs::File};

use memmap::Mmap;
use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Statistics {
    min: i16,
    max: i16,
    sum: i64,
    count: usize,
}

impl Statistics {
    #[inline]
    fn new(measurement: i16) -> Self {
        Statistics {
            min: measurement,
            max: measurement,
            sum: measurement as i64,
            count: 1,
        }
    }

    #[inline]
    fn average(&self) -> f64 {
        (self.sum / self.count as i64) as f64 / 10.
    }

    #[inline]
    fn add_measurement(&mut self, measurement: i16) {
        self.count += 1;
        self.min = measurement.min(self.min);
        self.max = measurement.max(self.max);
        self.sum += measurement as i64;
    }

    #[inline]
    fn merge(&mut self, other: &Statistics) {
        self.count += other.count;
        self.min = other.min.min(self.min);
        self.max = other.max.max(self.max);
        self.sum += other.sum;
    }
}

#[inline]
fn hacky_i16_float_parse(buf: &[u8]) -> i16 {
    let (sign, buf) = if buf[0] == b'-' {
        (-1i16, &buf[1..])
    } else {
        (1i16, buf)
    };

    let num: i16 = buf
        .iter()
        .filter(|b| b.is_ascii_digit())
        .fold(0i16, |acc, b| {
            let decimal = (b - b'0') as i16;
            acc * 10 + decimal
        });
    num * sign
}

fn main() -> anyhow::Result<()> {
    let file = File::options().read(true).open("./measurement_data.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let result: HashMap<&[u8], Statistics> = mmap
        .par_split(|b| b == &b'\n')
        .filter(|buf| !buf.is_empty())
        .fold_with(HashMap::<&[u8], Statistics>::new(), |mut map, buf| {
            let pos = buf.iter().position(|b| b == &b';').unwrap();
            let (station, temperature_u8) = buf.split_at(pos);
            let temperature = hacky_i16_float_parse(&temperature_u8[1..]);

            if let Some(stored_stats) = map.get_mut(station) {
                stored_stats.add_measurement(temperature);
            } else {
                let stats = Statistics::new(temperature);
                map.insert(station, stats);
            }
            map
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

    let mut sorted: Vec<_> = result.into_iter().collect();
    sorted.par_sort_by_key(|i| i.0);

    for (station, stats) in sorted {
        println!(
            "{}, count: {}, min: {:.1}, max: {:.1}, avg: {:.1}",
            std::str::from_utf8(station)?,
            stats.count,
            stats.min as f64 / 10.0,
            stats.max as f64 / 10.0,
            stats.average(),
        );
    }

    Ok(())
}

#[test]
fn parse_positive() {
    assert_eq!(hacky_i16_float_parse("12.0".as_bytes()), 120);
    assert_eq!(hacky_i16_float_parse("99.9".as_bytes()), 999);
}

#[test]
fn parse_negative() {
    assert_eq!(hacky_i16_float_parse("-12.0".as_bytes()), -120);
    assert_eq!(hacky_i16_float_parse("-99.9".as_bytes()), -999);
}
