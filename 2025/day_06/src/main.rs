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

fn part_two(rows: &mut Vec<&str>) {
    // 1. Get the symbols
    let mut symbols_row: Vec<char> = rows
        .pop()
        .expect("Couldn't get symbols row")
        .chars()
        .collect();
    // 2. Get the number rows
    let mut numbers_rows: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();

    assert!(
        numbers_rows
            .iter()
            .all(|num_row| num_row.len() == numbers_rows[0].len())
    );

    let row_length_diff = numbers_rows[0].len() - symbols_row.len();
    for _i in 0..row_length_diff {
        symbols_row.push(' ');
    }
    assert!(
        numbers_rows
            .iter()
            .all(|num_row| num_row.len() == symbols_row.len())
    );

    println!("symbols_row: {:?}", symbols_row);
    println!("numbers_rows: {:?}", numbers_rows);

    // 3. while let Some(symbol)
    // 4. if symbol == " ", build number only from iters
    // 5. if symbol == "*" or "+", build number then add/multiply
    let mut computed_total_numbers: Vec<u64> = Vec::new();
    let mut current_problem_numbers: Vec<u64> = Vec::new();
    while let Some(symbol) = symbols_row.pop() {
        // If numbers are all spaces, do nothing
        // construct number in column, push into current numbers
        let mut num_str = String::new();
        numbers_rows.iter_mut().for_each(|row| {
            let num_char = row.pop().expect("Should have char");
            if num_char != ' ' {
                num_str.push(num_char);
            }
        });

        if num_str.trim().len() == 0 {
            continue;
        }

        println!("num_str: {}", num_str);

        // if symbol is " ", continue the loop
        // if symbol is "*" or "+", compute the number, push into computed_total_numbers,
        // then empty current_problem_numbers
        let number: u64 = num_str.parse().expect("Should have gotten u64");
        current_problem_numbers.push(number);
        println!("current_problem_numbers: {:?}", current_problem_numbers);
        if symbol == '+' {
            let current_total = current_problem_numbers
                .clone()
                .into_iter()
                .reduce(|acc, curr| acc + curr)
                .expect("Couldn't add numbers");
            computed_total_numbers.push(current_total);
            current_problem_numbers = Vec::new();
        } else if symbol == '*' {
            let current_total = current_problem_numbers
                .clone()
                .into_iter()
                .fold(1, |acc, curr| acc * curr);
            computed_total_numbers.push(current_total);
            println!("{:?}", current_problem_numbers);
            current_problem_numbers = Vec::new();
        }
    }

    println!("{:?}", computed_total_numbers);

    let result = computed_total_numbers
        .into_iter()
        .reduce(|acc, curr| acc + curr)
        .expect("Couldn't get final result");
    println!("Result: {}", result);
}

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end_matches('\n');

    let mut rows: Vec<&str> = contents.split('\n').collect();

    part_two(&mut rows);
    // part_one(&mut rows);
}
