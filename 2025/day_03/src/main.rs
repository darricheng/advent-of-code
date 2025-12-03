use std::fs;

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let data: Vec<&str> = contents.split('\n').collect();

    let result = data.iter().fold(0, |total, current_bank| {
        total + get_largest_two_digit_number(&current_bank)
    });

    println!("{}", result);
}

fn get_largest_two_digit_number(bank: &str) -> i32 {
    let bank_len = bank.len();
    let (first_digit_index, first_digit) =
        bank.chars()
            .enumerate()
            .fold((0, '0'), |largest, (i, current)| {
                if i == bank_len - 1 || current <= largest.1 {
                    largest
                } else {
                    (i, current)
                }
            });

    let (_, second_part) = bank.split_at(first_digit_index + 1);
    let second_digit =
        second_part.chars().fold(
            '0',
            |largest, current| {
                if current <= largest { largest } else { current }
            },
        );

    format!("{}{}", first_digit, second_digit)
        .parse::<i32>()
        .expect("Should be able to parse into number")
}

#[cfg(test)]
mod tests {
    use super::get_largest_two_digit_number;

    #[test]
    fn i_987654321111111() {
        assert_eq!(get_largest_two_digit_number("987654321111111"), 98);
    }
    #[test]
    fn i_811111111111119() {
        assert_eq!(get_largest_two_digit_number("811111111111119"), 89);
    }
    #[test]
    fn i_234234234234278() {
        assert_eq!(get_largest_two_digit_number("234234234234278"), 78);
    }
    #[test]
    fn i_818181911112111() {
        assert_eq!(get_largest_two_digit_number("818181911112111"), 92);
    }
}
