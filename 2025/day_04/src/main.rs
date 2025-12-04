use std::fs;

/**
* 4-2
*
* On each pass of the grid to remove paper rolls, we get a fresh ascii placeholder.
* This placeholder represents paper rolls that will be removed in this pass. That means
* when checking neighbours, we consider this pass' ascii placeholder as a paper roll
* along with the default '@' char. For the current paper roll that we are checking,
* only the '@' char is a valid paper roll, though this shouldn't matter.
*
* On subsequent passes, these placeholders are as good as empty slots, i.e. they are
* semantically equivalent to the '.' char.
*
* HYPOTHESIS: With this implementation, we should be able to do just a single pass of
* the grid to remove paper rolls for this round without needing to maintain a separate
* data structure to check which rolls were removed, then doing a second pass to remove
* those rolls for that round of removals.
*
* We might have to do cleanup passes if we run out of placeholder characters. In such a
* case, we'll have to convert everything that is not an '@' into '.', then continue
* with the next pass of removals.
*/

struct AsciiPlaceholder(u8);
impl AsciiPlaceholder {
    fn new() -> Self {
        AsciiPlaceholder(0)
    }
}
impl Iterator for AsciiPlaceholder {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        // Doesn't matter what we start from
        self.0 += 1;
        let ascii_char = self.0 as char;

        // Skip '@' and '.' as they already mean something significant for this puzzle
        if ascii_char == '@' || ascii_char == '.' {
            self.0 += 1;
        }

        if self.0 == u8::MAX {
            None
        } else {
            Some(self.0 as char)
        }
    }
}

struct OffsetCounter(usize);
impl OffsetCounter {
    fn new() -> Self {
        OffsetCounter(0)
    }
}
impl Iterator for OffsetCounter {
    // (row_offset, column_offset)
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        match self.0 {
            1 => Some((-1, -1)),
            2 => Some((-1, 0)),
            3 => Some((-1, 1)),
            4 => Some((0, -1)),
            5 => Some((0, 1)),
            6 => Some((1, -1)),
            7 => Some((1, 0)),
            8 => Some((1, 1)),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PaperRolls {
    grid: Vec<Vec<char>>,
    row_count: usize,
    column_count: usize,
}

impl PaperRolls {
    fn new(data: Vec<&str>) -> Self {
        PaperRolls {
            grid: data.iter().map(|row| row.chars().collect()).collect(),
            row_count: data
                .len()
                .try_into()
                .expect("Couldn't covert row_count to i32"),
            column_count: data
                .get(0)
                .expect("Failed to get first row")
                .len()
                .try_into()
                .expect("Couldn't convert column_count into i32"),
        }
    }
    fn check_is_paper_roll(
        self: &Self,
        row_index: usize,
        column_index: usize,
        other_roll_char: char,
    ) -> bool {
        if let Some(item) = self
            .grid
            .get(row_index)
            .and_then(|row| row.get(column_index))
        {
            *item == '@' || *item == other_roll_char
        } else {
            false
        }
    }
    fn into_iter(self: &Self) -> PaperRollIter {
        PaperRollIter::new(self.row_count, self.column_count)
    }
    fn check_number_of_neighbour_rolls(
        self: &Self,
        row_index: usize,
        column_index: usize,
        removed_roll_placeholder: char,
    ) -> usize {
        let offset_counter = OffsetCounter::new();
        let number_of_neighbour_rolls =
            offset_counter.fold(0, |total_neighbour_rolls, (row_offset, column_offset)| {
                let i32_row_index: i32 = row_index.try_into().expect("Couldn't covert to i32");
                let i32_column_index: i32 =
                    column_index.try_into().expect("Couldn't covert to i32");

                let row_index_to_check = i32_row_index + row_offset;
                let column_index_to_check = i32_column_index + column_offset;

                if row_index_to_check < 0
                    || column_index_to_check < 0
                    || row_index_to_check
                        >= self.row_count.try_into().expect("Couldn't covert to i32")
                    || column_index_to_check
                        >= self
                            .column_count
                            .try_into()
                            .expect("Couldn't covert to i32")
                {
                    total_neighbour_rolls
                } else if self.check_is_paper_roll(
                    row_index_to_check
                        .try_into()
                        .expect("Couldn't convert back to usize"),
                    column_index_to_check
                        .try_into()
                        .expect("Couldn't convert back to usize"),
                    removed_roll_placeholder,
                ) {
                    total_neighbour_rolls + 1
                } else {
                    total_neighbour_rolls
                }
            });

        number_of_neighbour_rolls
    }

    fn remove_roll(
        self: &mut Self,
        row_index: usize,
        column_index: usize,
        removed_roll_placeholder: char,
    ) {
        self.grid[row_index][column_index] = removed_roll_placeholder
    }
}

struct PaperRollIter {
    row_index: usize,
    column_index: usize,
    row_count: usize,
    column_count: usize,
}
impl PaperRollIter {
    fn new(row_count: usize, column_count: usize) -> Self {
        PaperRollIter {
            row_index: 0,
            column_index: 0,
            row_count,
            column_count,
        }
    }
}

impl Iterator for PaperRollIter {
    // char, row_index, column_index
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let item_row = self.row_index;
        let item_column = self.column_index;

        self.column_index += 1;
        if self.column_index == self.column_count {
            self.column_index = 0;
            self.row_index += 1;
        }

        if item_row == self.row_count {
            None
        } else {
            Some((item_row, item_column))
        }
    }
}

fn main() {
    let file_path = "data.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();

    let data: Vec<&str> = contents.split('\n').collect();

    let mut paper_rolls = PaperRolls::new(data);

    let mut ascii_placeholder_iter = AsciiPlaceholder::new();

    let mut result = 0;
    loop {
        let paper_roll_iter = paper_rolls.into_iter();
        let removed_roll_placeholder = ascii_placeholder_iter
            .next()
            .expect("Ran out of ascii placeholders");

        let number_of_removed_rolls =
            paper_roll_iter.fold(0, |total, (row_index, column_index)| {
                // Just pass in '@' again
                if paper_rolls.check_is_paper_roll(row_index, column_index, '@')
                    && paper_rolls.check_number_of_neighbour_rolls(
                        row_index,
                        column_index,
                        removed_roll_placeholder,
                    ) < 4
                {
                    paper_rolls.remove_roll(row_index, column_index, removed_roll_placeholder);
                    total + 1
                } else {
                    total
                }
            });

        println!("{}", number_of_removed_rolls);

        if number_of_removed_rolls == 0 {
            break;
        }
        result += number_of_removed_rolls;
    }

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_paper_roll() {
        let paper_rolls = PaperRolls::new(vec!["@."]);
        let is_paper_roll = paper_rolls.check_is_paper_roll(0, 0, '@');
        let is_not_paper_roll = paper_rolls.check_is_paper_roll(0, 1, '@');
        let is_row_too_big = paper_rolls.check_is_paper_roll(1, 1, '@');
        let is_column_too_big = paper_rolls.check_is_paper_roll(0, 2, '@');

        assert_eq!(
            (
                is_paper_roll,
                !is_not_paper_roll,
                !is_row_too_big,
                !is_column_too_big
            ),
            (true, true, true, true)
        );
    }
}
