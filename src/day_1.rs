pub fn part_1(input: &str) -> String {
    let l = input.len();
    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut num_iter = nums.into_iter().cycle().take(l + 1);
    let first = num_iter.next().unwrap();
    let (matches, _) = num_iter.fold((vec![], first), |(mut acc, prev), i| {
        if i == prev {
            acc.push(i);
        }
        (acc, i)
    });
    let sum: u32 = matches.into_iter().sum();
    format!("{}", sum)
}

pub fn part_2(input: &str) -> String {
    let l = input.len();
    let step = l / 2;
    let nums: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut num_iter = nums.into_iter().cycle().take(l + step);
    let first = num_iter.next().unwrap();
    let (matches, _) = num_iter.fold((vec![], first), |(mut acc, prev), i| {
        if i == prev {
            acc.push(i);
        }
        (acc, i)
    });
    let sum: u32 = matches.into_iter().sum();
    format!("{}", sum)
}


#[cfg(test)]
mod tests {
    use super::*;
    fn t1(i: &str, o: &str) {
        assert_eq!(part_1(i), o.to_string());
    }
    fn t2(i: &str, o: &str) {
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
