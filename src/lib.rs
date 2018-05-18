extern crate serde_json;

use serde_json::{Error, Value};
use std::fs::File;
use std::io::Read;

trait RandomResult {
    type Re;
    fn choice(&self) -> Self::Re;
}

struct Argv<'a> {
    main: &'a str,
    part: serde_json::Value,
}

impl<'a> RandomResult for Argv<'a> {
    type Re = String;
    fn choice(&self) -> Self::Re {
        String::new()
    }
}

struct SingleComm<'a, T>
where
    T: RandomResult,
{
    Comm: &'a str,
    Argv: T,
}

pub fn read_file(filename: &str) -> &str {
    let mut f = File::open(filename).expect("cannot find file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.as_str()
}

pub fn parse_json<'a>(content: &str) -> SingleComm<'a, Argv<'a>> {
    let v: serde_json::Value = serde_json::from_str(content).unwrap();
    let a: serde_json::Value = v["argv"];
    let am = a["main"];
    let part = a["part"];

    return SingleComm {
        Comm: v["comm"].as_str().unwrap(),
        Argv: Argv {
            main: am.as_str().unwrap(),
            part: part,
        },
    };
}
