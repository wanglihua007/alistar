extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;
fn main() {
  let matches = App::new("alistar")
    .version("0.1.0")
    .author("wanglihua wanglihua@innobuddy.com")
    .about("A drop in cat replacement written in Rust")
    .arg(
      Arg::with_name("FILE")
        .help("File to print.")
        .empty_values(false),
    )
    .get_matches();
  if let Some(file) = matches.value_of("FILE") {
    println!("Value for file argument: {}", file);
    if Path::new(&file).exists() {
      match File::open(file) {
        Ok(mut f) => {
          let mut data = String::new();
          f.read_to_string(&mut data)
            .expect("[alistar Error] Unable to read the  file.");
          let stdout = std::io::stdout();
          let mut handle = std::io::BufWriter::new(stdout);
          match writeln!(handle, "{}", data) {
            Ok(_res) => {}
            Err(err) => {
              eprintln!(
                "[alistar Error] Unable to display the file contents. {:?}",
                err
              );
              process::exit(1);
            }
          }
        }
        Err(err) => {
          eprintln!("[alistar Error] Unable to read the file. {:?}", err);
          process::exit(1);
        }
      }
    } else {
      eprintln!("[alistar Error] No such file or directory.");
      process::exit(1);
    }
  }
}
