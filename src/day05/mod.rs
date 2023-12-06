use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

const INPUT: &str = include_str!("input.txt");

pub(crate) fn main() {
    let location = part_one(INPUT);
    println!("lowest location number: {location}");
    let instant = Instant::now();
    let location = part_two(INPUT);
    let elapsed = instant.elapsed();
    println!("lowest location number (range): {location}");
    println!("completed in {} µs", elapsed.as_micros());
}

fn part_one(input: &str) -> u64 {
    let almanac = parse_input(input);
    let mut locations: Vec<u64> = almanac.get_locations();
    locations.sort();
    locations[0]
}

fn part_two(input: &str) -> u64 {
    let almanac = parse_input(input);
    let seed_ranges = almanac.get_location_ranges();

    seed_ranges
        .into_iter()
        .min_by_key(|sr| sr.start)
        .map(|sr| sr.start)
        .unwrap()
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();
    // let parts: Vec<_> = input.split("\n\n").collect();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|num| {
            num.parse::<u64>()
                .map_err(|e| format!("could not parse {num}: {e}"))
                .unwrap()
        })
        .collect();

    let mut maps = HashMap::new();

    // for map in &parts[1..] {
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        let (from, to) = line.split_once(' ').unwrap().0.split_once("-to-").unwrap();

        let mut ranges: Vec<MapRange> = Vec::new();
        for range_line in lines.by_ref() {
            if range_line.is_empty() {
                break;
            }
            let range_vals: Vec<u64> = range_line
                .split(' ')
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            ranges.push(MapRange {
                source: range_vals[1],
                dest: range_vals[0],
                length: range_vals[2],
            })
        }

        let parsed = Map {
            to: to.to_string(),
            ranges,
        };
        maps.insert(from.to_string(), parsed);
    }

    Almanac { seeds, maps }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MapRange {
    source: u64,
    dest: u64,
    length: u64,
}
impl MapRange {
    fn convert(&self, current: u64) -> Option<u64> {
        let big_boi = self.source + self.length;
        if current >= self.source && current < big_boi {
            let res = (current - self.source) + self.dest;
            return Some(res);
        }
        None
    }
    fn last_source(&self) -> u64 {
        self.source + self.length - 1
    }

    fn map_range(&self, seed_range: &SeedRange) -> Vec<MapResult> {
        let mut res: Vec<MapResult> = Vec::new();
        if seed_range.start < self.source {
            if seed_range.last() < self.source {
                // no map, seed_range precedes map range
                return vec![MapResult::Unmapped(*seed_range)];
            }

            if let Some(mapped_end) = self.convert(seed_range.last()) {
                // end of range is within map range
                let before_length = self.source - seed_range.start;
                if before_length > 0 {
                    res.push(MapResult::Unmapped(SeedRange {
                        start: seed_range.start,
                        length: before_length,
                    }));
                }

                let in_length = mapped_end - self.dest + 1;
                if in_length > 0 {
                    res.push(MapResult::Mapped(SeedRange {
                        start: self.dest,
                        length: in_length,
                    }));
                }
            } else {
                // map range intersects completely with seed_range
                let before_length = self.source - seed_range.start;
                if before_length > 0 {
                    res.push(MapResult::Unmapped(SeedRange {
                        start: seed_range.start,
                        length: before_length,
                    }));
                }

                res.push(MapResult::Mapped(SeedRange {
                    start: self.dest,
                    length: self.length,
                }));

                let after_length = seed_range.length - (before_length + self.length);
                if after_length > 0 {
                    res.push(MapResult::Unmapped(SeedRange {
                        start: self.source + self.length,
                        length: after_length,
                    }));
                }
            }
        } else if let Some(mapped_start) = self.convert(seed_range.start) {
            // seed start is inside map range
            if let Some(mapped_end) = self.convert(seed_range.last()) {
                // seed_range is included in map range
                let length = (mapped_end - mapped_start) + 1;
                if length > 0 {
                    res.push(MapResult::Mapped(SeedRange {
                        start: mapped_start,
                        length,
                    }));
                }
            } else {
                // seed range starts in map, and ends outside of map

                let in_length = (self.dest + self.length) - mapped_start;
                if in_length > 0 {
                    res.push(MapResult::Mapped(SeedRange {
                        start: mapped_start,
                        length: in_length,
                    }));
                }

                let out_length = seed_range.length - in_length;
                if out_length > 0 {
                    let out_range = SeedRange {
                        start: self.last_source() + 1,
                        length: out_length,
                    };
                    res.push(MapResult::Unmapped(out_range));
                }
            }
        } else {
            // no map, seed_range is fully after map range
            return vec![MapResult::Unmapped(*seed_range)];
        }

        res
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MapResult {
    Mapped(SeedRange),
    Unmapped(SeedRange),
}

struct Map {
    to: String,
    ranges: Vec<MapRange>,
}
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}
impl Almanac {
    fn get_locations(&self) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::new();
        for seed in &self.seeds {
            let mut current = *seed;
            let mut next_map = "seed";
            while let Some(map) = self.maps.get(next_map) {
                for range in &map.ranges {
                    if let Some(converted) = range.convert(current) {
                        current = converted;
                        break;
                    }
                }
                next_map = &map.to;
            }
            locations.push(current);
        }
        locations
    }

    fn get_location_ranges(&self) -> Vec<SeedRange> {
        let mut seed_ranges: HashSet<_> = self
            .seeds
            .chunks_exact(2)
            .map(|c| SeedRange {
                start: c[0],
                length: c[1],
            })
            .collect();

        let mut next_map = "seed";

        while let Some(map) = self.maps.get(next_map) {
            let mut tmp_seed_ranges = HashSet::new();

            for sr in seed_ranges {
                let mut unmapped_set = VecDeque::new();
                unmapped_set.push_front(sr);

                while let Some(seed_range) = unmapped_set.pop_back() {
                    let mut mapped_set = HashSet::new();

                    for map_range in &map.ranges {
                        let map_results = map_range.map_range(&seed_range);
                        for map_result in map_results {
                            match map_result {
                                MapResult::Mapped(mapped) => {
                                    mapped_set.insert(mapped);
                                }
                                MapResult::Unmapped(unmapped) => {
                                    if unmapped != seed_range {
                                        unmapped_set.push_back(unmapped)
                                    }
                                }
                            }
                        }
                    }

                    if mapped_set.is_empty() {
                        // this seed range has no mappings
                        tmp_seed_ranges.insert(seed_range);
                    } else {
                        for mapped in mapped_set {
                            tmp_seed_ranges.insert(mapped);
                        }
                    }
                }
            }
            seed_ranges = tmp_seed_ranges;
            next_map = &map.to;
        }

        let mut sorted: Vec<_> = seed_ranges.into_iter().collect();
        sorted.sort_by_key(|sr| sr.start);
        sorted
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SeedRange {
    start: u64,
    length: u64,
}
impl SeedRange {
    fn last(&self) -> u64 {
        self.start + self.length - 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        let location = part_one(EXAMPLE_INPUT);
        assert_eq!(location, 35);
    }

    #[test]
    fn test_part_two() {
        let lowest_location = part_two(EXAMPLE_INPUT);
        assert_eq!(lowest_location, 46);
    }

    #[test]
    fn test_range_is_out_before() {
        /*
        SEED: |------|
        MAP:            |-----|
        EXP:  |------|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 4,
            length: 5,
        };
        let expected = vec![MapResult::Unmapped(seed_range)];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_left_intersects_map_start_only() {
        /*
        SEED: |------|
        MAP:        |-----|
        EXP:  |-----||
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 4,
            length: 7,
        };

        let expected = vec![
            MapResult::Unmapped(SeedRange {
                start: 4,
                length: 6,
            }),
            MapResult::Mapped(SeedRange {
                start: 1,
                length: 1,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_left_intersects_map_from_left() {
        /*
        SEED: |--------|
        MAP:        |-----|
        EXP:  |-----|xx|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 4,
            length: 9,
        };

        let expected = vec![
            MapResult::Unmapped(SeedRange {
                start: 4,
                length: 6,
            }),
            MapResult::Mapped(SeedRange {
                start: 1,
                length: 3,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_left_intersects_map_until_end() {
        /*
        SEED: |-----------|
        MAP:        |-----|
        EXP:  |-----|xxxxx|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 4,
            length: 16,
        };

        let expected = vec![
            MapResult::Unmapped(SeedRange {
                start: 4,
                length: 6,
            }),
            MapResult::Mapped(SeedRange {
                start: 1,
                length: 10,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_left_intersects_map_until_one_after_end() {
        /*
        SEED: |------------|
        MAP:        |-----|
        EXP:  |-----|xxxxx|| (3 results)
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 4,
            length: 17,
        };

        let expected = vec![
            MapResult::Unmapped(SeedRange {
                start: 4,
                length: 6,
            }),
            MapResult::Mapped(SeedRange {
                start: 1,
                length: 10,
            }),
            MapResult::Unmapped(SeedRange {
                start: 20,
                length: 1,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_left_intersects_map_after_end() {
        /*
        SEED: |---------------|
        MAP:        |-----|
        EXP:  |-----|xxxxx|---|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 4,
            length: 20,
        };

        let expected = vec![
            MapResult::Unmapped(SeedRange {
                start: 4,
                length: 6,
            }),
            MapResult::Mapped(SeedRange {
                start: 1,
                length: 10,
            }),
            MapResult::Unmapped(SeedRange {
                start: 20,
                length: 4,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_exact_intersects() {
        /*
        SEED:       |-----|
        MAP:        |-----|
        EXP:        |xxxxx|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 10,
            length: 10,
        };

        let expected = vec![MapResult::Mapped(SeedRange {
            start: 1,
            length: 10,
        })];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_subset_from_start() {
        /*
        SEED:       |----|
        MAP:        |-----|
        EXP:        |xxxx|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 10,
            length: 9,
        };

        let expected = vec![MapResult::Mapped(SeedRange {
            start: 1,
            length: 9,
        })];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_subset() {
        /*
        SEED:        |---|
        MAP:        |-----|
        EXP:         |xxx|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 11,
            length: 8,
        };

        let expected = vec![MapResult::Mapped(SeedRange {
            start: 2,
            length: 8,
        })];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_subset_incl_end() {
        /*
        SEED:         |---|
        MAP:        |-----|
        EXP:          |xxx|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 12,
            length: 8,
        };

        let expected = vec![MapResult::Mapped(SeedRange {
            start: 3,
            length: 8,
        })];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_right_intersect_one_over_end() {
        /*
        SEED:         |----|
        MAP:        |-----|
        EXP:          |xxx|| (2 results)
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 12,
            length: 9,
        };

        let expected = vec![
            MapResult::Mapped(SeedRange {
                start: 3,
                length: 8,
            }),
            MapResult::Unmapped(SeedRange {
                start: 20,
                length: 1,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_right_intersect() {
        /*
        SEED:         |-------|
        MAP:        |-----|
        EXP:          |xxx|---|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 12,
            length: 11,
        };

        let expected = vec![
            MapResult::Mapped(SeedRange {
                start: 3,
                length: 8,
            }),
            MapResult::Unmapped(SeedRange {
                start: 20,
                length: 3,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_right_intersect_start_at_end() {
        /*
        SEED:            |-------|
        MAP:        |-----|
        EXP:             ||------|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 19,
            length: 11,
        };

        let expected = vec![
            MapResult::Mapped(SeedRange {
                start: 10,
                length: 1,
            }),
            MapResult::Unmapped(SeedRange {
                start: 20,
                length: 10,
            }),
        ];

        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_right_start_one_after_end() {
        /*
        SEED:              |------|
        MAP:        |-----|
        EXP:               |------|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 20,
            length: 11,
        };

        let expected = vec![MapResult::Unmapped(seed_range)];
        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }

    #[test]
    fn test_range_after_end() {
        /*
        SEED:                       |------|
        MAP:        |-----|
        EXP:                        |------|
         */
        let map_range = MapRange {
            source: 10,
            length: 10,
            dest: 1,
        };

        let seed_range = SeedRange {
            start: 99,
            length: 11,
        };

        let expected = vec![MapResult::Unmapped(seed_range)];
        let convert = map_range.map_range(&seed_range);

        assert_eq!(convert, expected);
    }
}
