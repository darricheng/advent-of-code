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

            // 2-1
            // let value_to_add = if i_str_len % 2 == 0 {
            //     let mid = i_str_len / 2;
            //     let (first_half, second_half) = i_str.split_at(mid);
            //
            //     if first_half == second_half { i } else { 0 }
            // } else {
            //     0
            // };

            // 2-2
            let value_to_add = if i_str_len <= 1 {
                0
            } else {
                // Convert to bytes slice so that it's easier to work with.
                let bytes = i_str.as_bytes();

                // Get the factors, which are the ways to to split the slice equally.
                let chunk_sizes = find_factors(bytes.len());

                // The resulting byte slices implement Eq, so we should be able to
                // compare them directly. If they are all the same, we add this number
                // to the accumulator and skip all the other factors.
                let meets_criteria = chunk_sizes.into_iter().any(|size| {
                    let mut chunks = bytes.chunks(size);

                    let first_chunk = chunks.next().expect("Should have the first chunk");

                    chunks.all(|chunk| chunk == first_chunk)
                });

                if meets_criteria { i } else { 0 }
            };

            acc + value_to_add
        });

        total + range_total
    });

    println!("Final: {}", final_value);
}

fn find_factors(number: usize) -> Vec<usize> {
    // The factors are the possible chunk sizes.
    let mut factors = vec![1];

    // Floored sqrt returned by isqrt is the max that a factor can be
    // We add one because we want to iterate up till the floored number
    let sqrt_max_exclusive = number.isqrt() + 1;

    // We start at 2 because we don't want the possible chunk size of the entire slice.
    (2..sqrt_max_exclusive).for_each(|i| {
        if number % i == 0 {
            factors.push(i);

            // Both the divisor and the quotient are factors, unless they are the sqrt
            if i * i != number {
                factors.push(number / i);
            }
        }
    });

    factors.sort();
    factors
}
