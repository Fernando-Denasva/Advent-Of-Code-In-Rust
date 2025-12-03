use solutions;

fn main() {
    let input = r"";
    let answer = match solutions::day_two::part_two::solution(input) {
        Ok(a) => a,
        Err(e) => panic!("{e}")
    };
    println!("{answer}");
}