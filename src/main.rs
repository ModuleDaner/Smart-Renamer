
#![allow(unused_imports)]

use std::{io};
mod lib;

fn main() -> io::Result<()> {
    let key = dotenv::var("API_KEY").unwrap();
    println!("{}", lib::fetch_ai());

    Ok(())
}