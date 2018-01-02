extern crate advent;
extern crate rprompt;
use advent::Stars;

fn main() {
    let progress = advent::Progress::read();
    println!("{}", progress.status());

    match progress.day {
        1 => {
            let s = &advent::read_input("1-1")[0];
            match progress.stars {
                Stars::Zero => println!("Part 1 answer: {}", advent::day_1::part_1(s)),
                Stars::One => println!("Part 2 answer: {}", advent::day_1::part_2(s)),
            }
        }
        2 => {
            let s = advent::read_input("2");
            match progress.stars {
                Stars::Zero => println!("Part 1 answer: {}", advent::day_2::part_1(s)),
                Stars::One => println!("Part 2 answer: {}", advent::day_2::part_2(s)),
            }
        }
        3 => match progress.stars {
            Stars::Zero => println!("Part 1 answer: {}", advent::day_3::part_1(325489)),
            Stars::One => println!("Part 2 answer: {}", advent::day_3::part_2(325489)),
        },
        4 => {
            let s = advent::read_input("4");
            match progress.stars {
                Stars::Zero => println!("Part 1 answer: {}", advent::day_4::part_1(s)),
                Stars::One => println!("Part 2 answer: {}", advent::day_4::part_2(s)),
            }
        }
        n => panic!("Please add a case for day {}", n),
    }

    let reply = rprompt::prompt_reply_stdout("Did it work?: ").unwrap();
    if let "y" = reply.as_ref() {
        advent::Progress::read().increment();
    }
}
