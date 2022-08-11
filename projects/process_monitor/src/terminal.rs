


pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

// get window size with
// https://stackoverflow.com/questions/58892528/get-console-width-in-rust