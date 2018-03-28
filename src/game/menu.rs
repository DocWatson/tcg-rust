use std::io::{stdin,stdout,Write};

pub fn get_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");

    // remove new line characters
    let len_withoutcrlf = s.trim_right().len();
    s.truncate(len_withoutcrlf);

    s
}