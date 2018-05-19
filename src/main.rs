extern crate random_choice;

use random_choice::RandomResult;
use std::process::Command;

fn main() -> Result<(), String> {
    let content = random_choice::read_file("file.json");
    let js = random_choice::parse_json(content).unwrap();
    let re = format!("{}{}", js.argv.main, js.argv.choice());

    //println!("{}", re);

    Command::new(js.comm)
        .arg(re)
        .output()
        .expect("failed to execute process");

    Ok(())
}
