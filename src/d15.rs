use std::cmp;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Sensor {
    postion: (i32, i32),
    beacon_postion: (i32, i32),
}

impl Sensor {
    pub fn distance(&self, other_position: (i32, i32)) -> i32 {
        (self.postion.0 - other_position.0).abs() + (self.postion.1 - other_position.1).abs()
    }

    pub fn closest_beacon_distance(&self) -> i32 {
        self.distance(self.beacon_postion)
    }
}

pub fn part1(input: String, target: i32) -> i32 {
    let mut sensors: Vec<Sensor> = Vec::new();

    input.lines().for_each(|line| {
        let (sensor_str, beacon_str) = line.split_once(": ").unwrap();
        let (sensor_x_str, sensor_y) = sensor_str.split_once(", y=").unwrap();
        let (_, sensor_x) = sensor_x_str.split_once("Sensor at x=").unwrap();
        let sensor_y: i32 = sensor_y.parse().unwrap();
        let sensor_x: i32 = sensor_x.parse().unwrap();
        let (beacon_x_str, beacon_y) = beacon_str.split_once(", y=").unwrap();
        let (_, beacon_x) = beacon_x_str.split_once("closest beacon is at x=").unwrap();
        let beacon_y: i32 = beacon_y.parse().unwrap();
        let beacon_x: i32 = beacon_x.parse().unwrap();
        sensors.push(Sensor {
            postion: (sensor_x, sensor_y),
            beacon_postion: (beacon_x, beacon_y),
        });
    });

    let mut beacons_found = 0;

    let mut beacons = HashSet::new();
    let mut candidates: Vec<&Sensor> = vec![];

    let mut min = 0;
    let mut max = 0;

    for sensor in sensors.iter() {
        let closest_beacon_distance = sensor.closest_beacon_distance();

        let position_to_min = sensor.postion.1 - closest_beacon_distance;
        let position_to_max = sensor.postion.1 + closest_beacon_distance;

        min = cmp::min(min, position_to_min);
        max = cmp::max(max, position_to_max);

        if position_to_max >= target && position_to_min <= target {
            candidates.push(&sensor);

            if sensor.beacon_postion.1 == target {
                beacons.insert(sensor.beacon_postion);
            }
        }
    }

    (min..max).for_each(|i| {
        for sensor in candidates.iter() {
            if sensor.distance((i, target)) <= sensor.closest_beacon_distance() {
                beacons_found += 1;
                break;
            }
        }
    });

    (beacons_found - beacons.len()) as i32
}

pub fn part2(input: String, target: i32) -> i64 {
    let mut sensors: Vec<Sensor> = Vec::new();

    input.lines().for_each(|line| {
        let (sensor_str, beacon_str) = line.split_once(": ").unwrap();
        let (sensor_x_str, sensor_y) = sensor_str.split_once(", y=").unwrap();
        let (_, sensor_x) = sensor_x_str.split_once("Sensor at x=").unwrap();
        let sensor_y: i32 = sensor_y.parse().unwrap();
        let sensor_x: i32 = sensor_x.parse().unwrap();
        let (beacon_x_str, beacon_y) = beacon_str.split_once(", y=").unwrap();
        let (_, beacon_x) = beacon_x_str.split_once("closest beacon is at x=").unwrap();
        let beacon_y: i32 = beacon_y.parse().unwrap();
        let beacon_x: i32 = beacon_x.parse().unwrap();
        sensors.push(Sensor {
            postion: (sensor_x, sensor_y),
            beacon_postion: (beacon_x, beacon_y),
        });
    });

    fn get_range_vec(y: i32, sensors: &Vec<Sensor>) -> Vec<RangeInclusive<i32>> {
        let mut ranges: Vec<RangeInclusive<i32>> = sensors
            .iter()
            .flat_map(|sensor| {
                let distance = sensor.closest_beacon_distance();
                let offset = distance - (sensor.postion.1 - y).abs();
                if offset >= 0 {
                    Some(sensor.postion.0 - offset..=sensor.postion.0 + offset)
                } else {
                    None
                }
            })
            .collect();

        ranges.sort_unstable_by_key(|range| *range.start());

        let mut range_vec = vec![ranges[0].clone()];

        for next in ranges.iter().skip(1) {
            let last_idx = range_vec.len() - 1;
            let last = &range_vec[last_idx];
            if next.start() <= last.end() || last.end() + 1 == *next.start() {
                if next.end() > last.end() {
                    range_vec[last_idx] = *range_vec[last_idx].start()..=*next.end();
                }
            } else {
                range_vec.push(next.clone());
            }
        }
        range_vec
    }

    let (y, x_ranges) = (0..=target)
        .map(|y| (y, get_range_vec(y, &sensors)))
        .find(|(_i, ranges)| ranges.len() > 1)
        .unwrap();

    (1 + *x_ranges[0].end() as i64) * 4000000_i64 + (y as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3".to_string();
        assert_eq!(part1(input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let input =
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3".to_string();
        assert_eq!(part2(input, 20), 56000011);
    }
}
