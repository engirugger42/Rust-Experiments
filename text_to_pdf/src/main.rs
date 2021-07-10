use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use structopt::StructOpt;

/// text_to_pdf is just a toy for learning Rust. At the moment, it's a glorified cat
/// that just takes a directory's contents and dumps them into one file.
/// TODO:
/// * Support other file types
/// * Bail out early if there are no files in the directory matching the extension filter
/// * Actually convert to a PDF
// Derived from: https://rust-cli.github.io/book/tutorial/cli-args.html
#[derive(StructOpt)]
struct Cli {
    /// The name of output file
    #[structopt(short = "o", long = "output", default_value = "output.txt")]
    output_file: String,
    /// The pattern for files to concatenate
    #[structopt(short = "f", long = "filter", default_value = "txt")]
    file_filter: String,
    /// The path to the files to read
    #[structopt(parse(from_os_str), short = "i", long = "input", default_value = "./")]
    input_path: std::path::PathBuf
}

fn main() -> std::result::Result<(), std::io::Error> {
    let args = Cli::from_args();
    let mut file = std::fs::File::create(&args.output_file).expect("create failed");

    let paths = fs::read_dir(&args.input_path).unwrap();
    for path in paths {
        let unwrapped_path = path.unwrap().path();
        if unwrapped_path.extension().unwrap().to_str().unwrap() == &args.file_filter {
            let this_line = unwrapped_path.display().to_string();
            println!("{}", this_line);
            let mut phrack_file = BufReader::new(File::open(this_line)?);
            let mut contents = String::new();
            phrack_file.read_to_string(&mut contents).unwrap();
            file.write_all(contents.as_bytes()).expect("write failed");
        }
    }

    Ok(())
}

// Now, we invoke the system commands to convert to enscript, then to pdf
// enscript --header='$n|Page $% of $=' -p - output.txt | ps2pdf - phrack68.pdf