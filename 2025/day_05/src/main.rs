use std::fs;

// Range(min, max)
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
}

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let data: Vec<&str> = contents.split('\n').collect();
    let mut data_iter = data.split(|v| *v == "");

    let ranges_strs = data_iter.next().expect("Couldn't get ranges_strs");
    let number_strs = data_iter.next().expect("Couldn't get number_strs");

    let ranges: Vec<Range> = ranges_strs
        .iter()
        .map(|range_str| Range::new(range_str))
        .collect();

    let result = number_strs.iter().fold(0, |total, num_str| {
        let number: u64 = num_str.parse().expect("Couldn't parse into number");

        if ranges
            .iter()
            .any(|Range(min, max)| number >= *min && number <= *max)
        {
            total + 1
        } else {
            total
        }
    });

    println!("Result: {}", result);
}
