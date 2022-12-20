fn main() {
    let mut elves = include_str!("./input.txt")
        .split("\n\n")
        .map(|n| n.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    elves.sort_by(|a, b| b.cmp(a));
    elves.truncate(3);

    println!("{:?}", elves);
}
