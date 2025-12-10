use std::fs;

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end_matches('\n');

    let mut rows_iter = contents.split('\n');

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
