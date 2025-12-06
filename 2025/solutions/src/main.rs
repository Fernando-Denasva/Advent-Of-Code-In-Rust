use solutions;

fn main() {
    let input_path = r"";
    let answer = match solutions::day_three::part_two::solution(input_path) {
        Ok(a) => a,
        Err(e) => panic!("{e}")
    };
    println!("{answer}");
}