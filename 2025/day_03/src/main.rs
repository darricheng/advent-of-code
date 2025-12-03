use std::fs;

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let data: Vec<&str> = contents.split('\n').collect();

    let result = data.iter().fold(0, |total, current_bank| {
        // 3-1
        // total + get_largest_two_digit_number(&current_bank)

        // 3-2
        total + get_largest_twelve_digit_number(&current_bank)
    });

    println!("{}", result);
}

// This is the final byte slice
struct Joltage([char; 12]);
impl Joltage {
    fn new(b: [char; 12]) -> Self {
        Joltage(b)
    }

    // index: the index of the char to be compared
    // new_values: the next slice of values
    //
    // The returned bool indicates whether the value at the
    // current index was updated.
    fn compare(self: &mut Self, index: usize, new_values: [char; 12]) -> bool {
        let current_slice = self.0;

        let current_value = current_slice
            .get(index)
            .expect("Failed to index into current_slice");
        let new_value = new_values
            .get(index)
            .expect("Failed to index into new_values");

        if new_value > current_value {
            // Update the stored slice from the index to the end with the new_values
            let current_sub_slice_to_keep = self
                .0
                .get(0..index)
                .expect("Failed to get current_sub_slice_to_keep");
            let new_sub_slice = new_values
                .get(index..12)
                .expect("Failed to get new_sub_slice");

            self.0 = [current_sub_slice_to_keep, new_sub_slice]
                .concat()
                .try_into()
                .expect("Failed to convert new slice into Joltage");

            true
        } else {
            false
        }
    }

    fn as_u64(self: &Self) -> u64 {
        let num_str: String = self.0.iter().collect();
        num_str
            .parse::<u64>()
            .expect("Failed to convert string to valid u64")
    }
}

struct BankIter {
    batteries: Vec<char>,
    current_index: usize,
    max_index: usize,
}
impl BankIter {
    fn new(bank: &str) -> Self {
        let batteries: Vec<char> = bank.chars().collect();
        let max_index = batteries.len() - 12;

        BankIter {
            batteries,
            current_index: 0,
            // When current_index reaches max_index, that's the last value
            max_index,
        }
    }
}
impl Iterator for BankIter {
    type Item = [char; 12];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index > self.max_index {
            None
        } else {
            let start_index = self.current_index;
            let end_index = start_index + 12;
            let item = self
                .batteries
                .get(start_index..end_index)
                .expect("Indexed out of range");

            self.current_index += 1;

            Some(item.try_into().expect("Didn't have 12 items to return"))
        }
    }
}

fn get_largest_twelve_digit_number(bank: &str) -> u64 {
    let mut bank_iter = BankIter::new(bank);
    let first_set = bank_iter
        .next()
        .expect("Couldn't get first_set from bank_iter");

    let mut joltage = Joltage::new(first_set);

    bank_iter.for_each(|slice| {
        slice
            .iter()
            .enumerate()
            .find(|(index, _)| joltage.compare(*index, slice));
    });

    joltage.as_u64()
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
