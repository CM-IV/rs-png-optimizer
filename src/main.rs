use color_eyre::eyre::Result;
use clap::Parser;
use oxipng::{OutFile, InFile, Options};


#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Path to PNG image needing optimization
    input_path: std::path::PathBuf,

    /// Output path of optimized PNG
    output_path: std::path::PathBuf,
}


fn main() -> Result<()> {

    let args = Args::parse();

    let input_file = args.input_path;
    let output_file = args.output_path;

    let options = Options::from_preset(2);

    oxipng::optimize(&InFile::Path(input_file), &OutFile::Path(Some(output_file)), &options)?;

    println!("Your image was optimized!");

    Ok(())
}
