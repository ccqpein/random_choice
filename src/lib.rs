#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use serde_json::Error;

extern crate rand;
use rand::Rng;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub trait RandomResult {
    type Re;
    fn choice(&self) -> Self::Re;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argv {
    pub main: String,
    parts: Option<Vec<String>>,
    range_num: Option<RangeNum>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RangeNum {
    from: i32,
    to: i32,
    tail: Vec<String>,
}

impl RandomResult for Argv {
    type Re = String;
    fn choice(&self) -> Self::Re {
        if let Some(a) = &self.range_num {
            let re_num = rand::thread_rng().gen_range(a.from, a.to).to_string();
            let tail_radint = rand::thread_rng().gen_range(0, a.tail.len());
            let tail = a.tail[tail_radint].clone();
            return format!("{}{}", re_num, tail);
        }

        if let Some(a) = &self.parts {
            let radint = rand::thread_rng().gen_range(0, a.len());
            let re = a[radint].clone();
            return re;
        }
        return String::new();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleComm<T>
where
    T: RandomResult,
{
    pub comm: String,
    pub argv: T,
}

pub fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("cannot find file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn parse_json(content: String) -> Result<SingleComm<Argv>, Error> {
    let sc: SingleComm<Argv> = serde_json::from_str(&content)?;
    Ok(sc)
}
