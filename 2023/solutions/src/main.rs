use solutions::day_one::part_one::solution;
fn main() {
    let input = r"";
    let answer = match solution(input) {
        Ok(a) => a,
        Err(e) => panic!("{e}")
    };
    println!("{answer}");
}