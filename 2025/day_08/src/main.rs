use std::{fs, str::Split};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coords(i32, i32, i32);

impl Coords {
    fn new(coords_str: &str) -> Self {
        let mut coords_iter = coords_str
            .split(',')
            .map(|num_str| num_str.parse::<i32>().expect("Couldn't parse into i32"));
        let x = coords_iter.next().expect("Couldn't get x");
        let y = coords_iter.next().expect("Couldn't get y");
        let z = coords_iter.next().expect("Couldn't get z");

        Coords(x, y, z)
    }
}

#[derive(Debug)]
struct CompareCoords {
    a: Coords,
    b: Coords,
    distance: i32,
}

impl CompareCoords {
    fn new(a: Coords, b: Coords) -> Self {
        let distance: i32 = ((a.0 - b.0).pow(2)) + ((a.1 - b.1).pow(2)) + ((a.2 - b.2).pow(2));

        CompareCoords { a, b, distance }
    }
}

#[derive(Debug, Clone)]
struct DisjointSetUnionNode {
    parent: Option<Box<DisjointSetUnionNode>>,
    size: Option<u32>,
    coords: Coords,
}

impl DisjointSetUnionNode {
    fn make_set(coords: Coords) -> Self {
        DisjointSetUnionNode {
            parent: None,
            size: Some(1),
            coords,
        }
    }
    fn find_set(&self) -> Coords {
        if let Some(p) = &self.parent {
            p.find_set()
        } else {
            self.coords
        }
    }
    fn union_sets(&self, other_set: &mut Self) {
        let a_coord = self.find_set();
        let b_coord = other_set.find_set();

        if a_coord != b_coord {
            other_set.parent = self.parent.clone()
        }
    }
}

fn part_one(rows_iter: Split<'_, char>) {
    let coords_list: Vec<Coords> = rows_iter.map(|row| Coords::new(row)).collect();
    let coords_list_len = coords_list.len();

    let mut compare_list: Vec<CompareCoords> = Vec::new();

    for i in 0..coords_list_len - 1 {
        let next_index = i + 1;
        for j in next_index..coords_list_len {
            compare_list.push(CompareCoords::new(coords_list[i], coords_list[j]));
        }
    }

    compare_list.sort_by(|a, b| a.distance.cmp(&b.distance));
    println!("{:?}", compare_list);

    // 1. Create a Disjoint Set Union data structure.
    // 2. Put each Coords into a Set.
    // 3. Make n number of connections according to the question.
    // 4. Somehow track the sizes of the sets???
    // 5. Get the result by multiplying together the sizes of x largest sets.
}

fn main() {
    let file_path = "sample.txt";

    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim_end_matches('\n');

    let rows_iter = contents.split('\n');

    part_one(rows_iter);
}
