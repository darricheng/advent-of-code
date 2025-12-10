use std::{fs, str::Split};

fn part_one(mut rows_iter: Split<'_, char>) {
    let first_row = rows_iter.next().expect("Should have first row");
    let mut tachyon_beams: Vec<bool> = first_row.chars().map(|char| char == 'S').collect();
    let row_length = tachyon_beams.len();

    let result = rows_iter.fold(0, |total, row| {
        let row_total = row.chars().enumerate().fold(0, |acc, (i, char)| {
            if char == '^' && tachyon_beams[i] {
                tachyon_beams[i] = false;
                if i > 0 {
                    tachyon_beams[i - 1] = true;
                }
                if i < row_length - 1 {
                    tachyon_beams[i + 1] = true;
                }
                acc + 1
            } else {
                acc
            }
        });

        total + row_total
    });

    println!("Result: {}", result);
}

fn part_two(mut rows_iter: Split<'_, char>) {
    let first_row = rows_iter.next().expect("Should have first row");
    let mut tachyon_beams_count: Vec<u32> = first_row
        .chars()
        .map(|char| if char == 'S' { 1 } else { 0 })
        .collect();
    let row_length = tachyon_beams_count.len();

    rows_iter.for_each(|row| {
        println!("Row: {:?}", row);
        row.chars().enumerate().for_each(|(i, char)| {
            if char == '^' && tachyon_beams_count[i] > 0 {
                if i > 0 {
                    tachyon_beams_count[i - 1] += tachyon_beams_count[i];
                }
                if i < row_length - 1 {
                    tachyon_beams_count[i + 1] += tachyon_beams_count[i];
                }
                tachyon_beams_count[i] = 0;
            }
        });
        println!("beam count: {:?}", tachyon_beams_count);
    });

    let result = tachyon_beams_count
        .into_iter()
        .reduce(|acc, curr| acc + curr)
        .expect("Should have final u32 result");
    println!("Result: {}", result);
}

fn main() {
    let file_path = "sample.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end_matches('\n');

    let rows_iter = contents.split('\n');

    // part_one(rows_iter);
    part_two(rows_iter);
}
