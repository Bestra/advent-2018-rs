pub fn part_1(input: Vec<String>) -> String {
    let n: u32 = input.iter()
        .map( |s| nums(s))
        .map( |nums| checksum(nums))
        .sum();
    format!("{}", n)
}

pub fn part_2(input: Vec<String>) -> String {
    let n: u32 = input.iter()
        .map( |s| nums(s))
        .map( |nums| divisor(nums))
        .sum();
    format!("{}", n)
}

fn nums(line: &str) -> Vec<u32> {
    line.trim().split_whitespace().map(|c| c.parse::<u32>().unwrap()).collect()
}

fn checksum(n: Vec<u32>) -> u32 {
    n.iter().max().unwrap() - n.iter().min().unwrap()
}

fn divisor(nums: Vec<u32>) -> u32 {
    for i in nums.iter() {
        for j in nums.iter() {
            if i % j == 0  && i != j{
                // println!("{} / {}", i, j);
                return i / j;
            } else if j % i == 0  && i != j {
                return j / i;
                // println!("{} / {}", j, i);
            }
        }
    }
    panic!("No divisor found");
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

    #[test]
    fn pt_2() {
        assert_eq!(part_2(split("5 9 2 8
9 4 7 3
3 8 6 5")), "9")
    }

    fn split(s: &str) -> Vec<String> {
        s.lines().map( |s| String::from(s)).collect()
    }


}
