use std::fs;

fn part_one(rows: &mut Vec<&str>) {
    let symbols_row: Vec<&str> = rows
        .pop()
        .expect("Couldn't get symbols row")
        .split_ascii_whitespace()
        .collect();

    let number_rows: Vec<Vec<u64>> = rows
        .iter()
        .map(|row| {
            row.split_ascii_whitespace()
                .map(|num_str| num_str.parse::<u64>().expect("Couldn't parse into u64"))
                .collect()
        })
        .collect();

    let mut results_vec: Vec<u64> = Vec::new();

    println!("numbers: {:?}", number_rows);
    println!("symbols: {:?}", symbols_row);

    symbols_row.iter().enumerate().for_each(|(i, symbol)| {
        let total = number_rows.iter().fold(0, |acc, row| {
            let current_number = row.get(i).expect("Couldn't get number");
            if *symbol == "+" {
                acc + current_number
            } else {
                if acc == 0 {
                    1 * current_number
                } else {
                    acc * current_number
                }
            }
        });
        results_vec.push(total);
    });

    println!("results_vec: {:?}", results_vec);

    let result = results_vec
        .into_iter()
        .reduce(|acc, curr| acc + curr)
        .expect("???");

    println!("Result: {}", result);
}

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let mut rows: Vec<&str> = contents.split('\n').collect();

    part_one(&mut rows);
}
