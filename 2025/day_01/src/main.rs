use std::fs;

#[derive(Debug)]
struct Dial {
    // Current value
    inner: i32,
    // Number of times the value is 0
    zero_counter: i32,
}

impl Dial {
    fn new() -> Self {
        Dial {
            inner: 50,
            zero_counter: 0,
        }
    }
    fn rotate(self: &mut Self, instruction: &str) {
        let initial_value = self.inner;

        let (direction, clicks_str) = instruction.split_at(1);
        let clicks = clicks_str.parse::<i32>().unwrap();

        let new_value = match direction {
            "L" => self.inner - clicks,
            "R" => self.inner + clicks,
            _ => panic!("direction wasn't L or R"),
        };

        // 1-1
        //
        // self.inner = new_value % 100;
        // if self.inner < 0 {
        //     self.inner += 100;
        // }
        //
        // if self.inner == 0 {
        //     self.zero_counter += 1;
        // }

        // 1-2
        let (additional_zeroes, new_inner) = match new_value {
            v if v == 0 => (1, 0),
            v if v > 0 => (v / 100, v % 100),
            v if v < 0 => {
                // Value is -ve, so we need to turn it +ve
                let mut zeroes = v / 100 * -1;

                // If we start at 0, we didn't go past zero the first time
                if initial_value != 0 {
                    zeroes += 1;
                }

                let mut remainder = v % 100;

                // We only need to reset the range if the remainder is not 0
                if remainder != 0 {
                    remainder += 100
                }

                (zeroes, remainder)
            }
            _ => panic!("shouldn't happen as all instances covered"),
        };

        // Debugging purposes
        // println!(
        //     "instruction: {}, additional_zeroes: {}, new_inner: {}",
        //     instruction, additional_zeroes, new_inner
        // );

        self.inner = new_inner;
        self.zero_counter += additional_zeroes;
    }
}

fn main() {
    let file_path = "data.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let data: Vec<&str> = contents.split('\n').collect();

    let mut dial = Dial::new();

    data.iter().for_each(|instruction| {
        if instruction.len() > 0 {
            dial.rotate(instruction);
        }
    });

    println!("{:?}", dial.zero_counter);
}
