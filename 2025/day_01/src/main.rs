use std::fs;

#[derive(Debug)]
struct Dial {
    // Current value
    inner: i32,
    // Number of times the value is 0
    zero_counter: u32,
}

impl Dial {
    fn new() -> Self {
        Dial {
            inner: 50,
            zero_counter: 0,
        }
    }
    fn rotate(self: &mut Self, instruction: &str) {
        let (direction, clicks_str) = instruction.split_at(1);
        let clicks = clicks_str.parse::<i32>().unwrap();

        let new_value = match direction {
            "L" => self.inner - clicks,
            "R" => self.inner + clicks,
            _ => panic!("direction wasn't L or R"),
        };
        self.inner = new_value % 100;
        if self.inner < 0 {
            self.inner += 100;
        }

        if self.inner == 0 {
            self.zero_counter += 1;
        }
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
