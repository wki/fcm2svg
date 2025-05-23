use std::{fs, path::PathBuf};
use std::any::Any;
use std::io::Error;
use clap::{Parser};
use fcmlib;

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    /// Specifies the input file
    #[arg(short, long, value_name = "FILE")]
    input_file: PathBuf,

    /// Specifies the output file
    #[arg(short, long, value_name = "FILE")]
    output_file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    let fcm = fcmlib::FcmFile::from_file(cli.input_file.as_path());
    dbg!(&fcm);
    //dbg!(fcm.unwrap().piece_table.pieces);
    for piece in fcm.unwrap().piece_table.pieces {
        dbg!(piece.type_id());
    }
    dbg!(&cli);

    // let result = load(&cli.input_file)
    //     .and_then(convert)
    //     .and_then(|c| save(&cli.output_file, c));
    // 
    // match result {
    //     Ok(_) => println!("Done"),
    //     Err(e) => println!("Could not process, error occured: '{}'", e),
    // }
}

fn load(file: &PathBuf) -> Result<String, Error> {
    fs::read_to_string(file)
}

fn convert(content: String) -> Result<String, Error> {
    // TODO: convert to SVG
    Ok(content + " / converted")
}

fn save(file: &PathBuf, content: String) -> Result<(), Error> {
    fs::write(file, content)
}
