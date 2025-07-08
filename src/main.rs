
#![allow(unused_imports)]

use std::{io};

fn main() -> io::Result<()> {
    let key = dotenv::var("API_KEY").unwrap();
    println!("{key}");

    Ok(())
}