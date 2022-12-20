fn main() {
    let contents = std::fs::read_to_string("./src/input.txt").unwrap();

    println!(
        "{}",
        contents
            .split("\n\n")
            .map(|n| n.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap()
    );
}
