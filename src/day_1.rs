pub fn part_1(input: &str) -> String {
    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    run(nums, 1)
}

pub fn part_2(input: &str) -> String {
    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let l = nums.len();
    run(nums, l / 2)
}

fn run(input: Vec<u32>, step: usize) -> String {
    let num_iter = input.iter();
    let mut look_ahead = input.iter().cycle();
    look_ahead.nth(step - 1);
    let mut matches =  vec![];
    for i in num_iter {
        let ahead = look_ahead.next().unwrap();
        if ahead == i {
            matches.push(i);
        }
    }

    let sum: u32 = matches.into_iter().sum();
    format!("{}", sum)
}


#[cfg(test)]
mod tests {
    use super::*;
    fn t1(i: &str, o: &str) {
        println!("{} should be {}", i, o);
        assert_eq!(part_1(i), o.to_string());
    }
    fn t2(i: &str, o: &str) {
        println!("{} should be {}", i, o);
        assert_eq!(part_2(i), o.to_string());
    }

    #[test]
    fn pt_1() {
        t1("1122", "3");
        t1("1111", "4");
        t1("1234", "0");
        t1("91212129", "9");
    }

    #[test]
    fn pt_2() {
        t2("1212", "6");
        t2("1221", "0");
        t2("123425", "4");
        t2("123123", "12");
        t2("12131415", "4");
    }

}
