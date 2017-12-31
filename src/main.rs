extern crate advent;
extern crate rprompt;

fn main() {
    let progress = advent::Progress::read();
    println!("{}", progress.status());
    let s = &advent::read_input("1-1")[0];
    println!("{}", advent::day_1::part_1(s));
    let reply = rprompt::prompt_reply_stdout("Did it work?: ").unwrap();
    match reply.as_ref() {
        "y" => advent::Progress::read().increment(),
        _ => ()
    }
}
