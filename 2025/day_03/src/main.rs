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

fn get_largest_twelve_digit_number(bank: &str) -> u64 {
    0
}

/**
* 3-1
*
* (UNTESTED)
* I think it's possible to do this in a single pass instead.
* We iterate over pairs of the numbers, meaning indices 0 and 1, then 1 and 2,
* until the second number reaches the last index.
*
* At the start, we take the first number and the second number as the largest
* digit for each position respectively.
* On each iteration, if the first number is the new largest first digit, we set
* the second number as the new largest second digit. Otherwise, we check if the
* second number is the new largest second digit and set it if so.
*/
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
    use super::*;

    #[test]
    fn two_987654321111111() {
        assert_eq!(get_largest_two_digit_number("987654321111111"), 98);
    }
    #[test]
    fn two_811111111111119() {
        assert_eq!(get_largest_two_digit_number("811111111111119"), 89);
    }
    #[test]
    fn two_234234234234278() {
        assert_eq!(get_largest_two_digit_number("234234234234278"), 78);
    }
    #[test]
    fn two_818181911112111() {
        assert_eq!(get_largest_two_digit_number("818181911112111"), 92);
    }

    #[test]
    fn twelve_987654321111111() {
        assert_eq!(
            get_largest_twelve_digit_number("987654321111111"),
            987654321111
        );
    }
    #[test]
    fn twelve_811111111111119() {
        assert_eq!(
            get_largest_twelve_digit_number("811111111111119"),
            811111111119
        );
    }
    #[test]
    fn twelve_234234234234278() {
        assert_eq!(
            get_largest_twelve_digit_number("234234234234278"),
            434234234278
        );
    }
    #[test]
    fn twelve_818181911112111() {
        assert_eq!(
            get_largest_twelve_digit_number("818181911112111"),
            888911112111
        );
    }
}
