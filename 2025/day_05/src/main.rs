use std::fs;

// Range(min, max)
#[derive(Clone)]
struct Range(u64, u64);
impl Range {
    fn new(range_str: &str) -> Self {
        let mut iter = range_str.split('-');
        let min = iter
            .next()
            .expect("Couldn't get min")
            .parse()
            .expect("Couldn't parse min");
        let max = iter
            .next()
            .expect("Couldn't get max")
            .parse()
            .expect("Couldn't parse max");

        Range(min, max)
    }

    fn can_merge(&self, b: &Self) -> bool {
        let Range(a_min, a_max) = *self;
        let Range(b_min, b_max) = b;

        !(a_max < *b_min || *b_max < a_min)
    }

    fn merge(&self, b: Self) -> Range {
        let a = self.clone();
        let Range(a_min, a_max) = a;
        let Range(b_min, b_max) = b;

        let a_min_smaller = a_min < b_min;
        let a_max_bigger = a_max > b_max;

        if a_min_smaller && a_max_bigger {
            a
        } else if !a_min_smaller && !a_max_bigger {
            b
        } else if a_min_smaller && !a_max_bigger {
            Range(a.0, b.1)
        } else {
            Range(b.0, a.1)
        }
    }
}

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let data: Vec<&str> = contents.split('\n').collect();
    let mut data_iter = data.split(|v| *v == "");

    let ranges_strs = data_iter.next().expect("Couldn't get ranges_strs");
    // let number_strs = data_iter.next().expect("Couldn't get number_strs");

    // 5-2
    let mut ranges_str_iter = ranges_strs.iter();
    let first_range_str = *ranges_str_iter.next().expect("Should have first range_str");

    let mut merged_ranges = vec![Range::new(first_range_str)];

    ranges_str_iter.for_each(|range_str| {
        let mut current_range = Range::new(range_str);

        loop {
            if let Some(can_merge_pos) = merged_ranges
                .iter()
                .position(|r| r.can_merge(&current_range))
            {
                // Remove the range, merge with current range,
                // update current range and loop
                let range_to_merge = merged_ranges.remove(can_merge_pos);
                current_range = current_range.merge(range_to_merge);
            } else {
                // No ranges to merge with
                merged_ranges.push(current_range);
                break;
            }
        }
    });

    let result = merged_ranges
        .iter()
        .fold(0, |total, Range(min, max)| total + max - min + 1);

    // 5-1
    // let result = number_strs.iter().fold(0, |total, num_str| {
    //     let number: u64 = num_str.parse().expect("Couldn't parse into number");
    //
    //     if ranges
    //         .iter()
    //         .any(|Range(min, max)| number >= *min && number <= *max)
    //     {
    //         total + 1
    //     } else {
    //         total
    //     }
    // });

    println!("Result: {}", result);
}
