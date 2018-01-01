pub fn part_1(input: Vec<String>) -> String {
    let n: u32 = input.iter()
        .map( |s| nums(s))
        .map( |nums| checksum(nums))
        .sum();
    format!("{}", n)
}

pub fn part_2(input: Vec<String>) -> String {
    "foo".to_string()
}

fn nums(line: &str) -> Vec<u32> {
    line.trim().split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect()
}

fn checksum(n: Vec<u32>) -> u32 {
    n.iter().max().unwrap() - n.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt_1() {
        assert_eq!(part_1(split("5 1 9 5
     7 5 3  
     2 4 6 8")), "18")
    }

    fn split(s: &str) -> Vec<String> {
        s.lines().map( |s| String::from(s)).collect()
    }


}
