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
    println!("{:?}", matches);
    let sum: u32 = matches.into_iter().sum();
    format!("{}", sum)
}


#[cfg(test)]
mod tests {
    use super::*;
    fn t(i: &str, o: &str) {
        assert_eq!(part_1(i), o.to_string());
    }

    #[test]
    fn ex_1() {
        t("1122", "3")
    }

    #[test]
    fn ex_2() {
        t("1111", "4")
    }

    #[test]
    fn ex_3() {
        t("1234", "0")
    }

    #[test]
    fn ex_4() {
        t("91212129", "9")
    }
}
