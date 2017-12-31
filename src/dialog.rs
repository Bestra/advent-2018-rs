use cursive::Cursive;
use cursive::views::{Dialog};

pub fn progress_dialog(day: &str) {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::new();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::text(format!("Day {}", day))
                  .title("Did it work?")
                  .button("Yes", |s| pick_file(s, "Select an input file"))
                  .button("No", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}
