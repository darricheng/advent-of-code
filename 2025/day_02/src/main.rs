use std::fs;

fn main() {
    let file_path = "data.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // remove the last new_line character
    let no_new_line = contents.trim();

    let ranges: Vec<&str> = no_new_line.split(',').collect();

    let final_value = ranges.iter().fold(0, |total, range_str| {
        let (start_str, end_str) = range_str
            .split_once('-')
            .expect("Invalid range encountered");

        let range_start = start_str.parse::<u64>().expect("Invalid range start");
        let end = end_str.parse::<u64>().expect("Invalid range end");
        let range_end = end + 1;

        let range_total = (range_start..range_end).fold(0, |acc, i| {
            let i_str = i.to_string();

            let i_str_len = i_str.len();
            let value_to_add = if i_str_len % 2 == 0 {
                let mid = i_str_len / 2;
                let (first_half, second_half) = i_str.split_at(mid);

                if first_half == second_half { i } else { 0 }
            } else {
                0
            };

            acc + value_to_add
        });

        total + range_total
    });

    println!("Final: {}", final_value);
}
