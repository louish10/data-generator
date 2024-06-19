use clap::Parser;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::io::Write;

use std::fs::File;
use std::path::Path;
use std::io::Result;

/// A utility to put data files into a directory
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(required = true)]
    directory: String,

    #[arg(long, short='N', default_value_t = 10)]
    number_of_files: usize,
    #[arg(long, short='n', default_value_t = 10)]
    number_of_lines: usize,
    #[arg(long, short='l', default_value_t = 10)]
    filename_length: usize,
}

fn random_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

fn write_file(filepath: &Path, args: &Args) -> Result<()> {
    let mut file = File::create(filepath)?;
    let mut rng = thread_rng();
    for _ in 0..args.number_of_lines {
        file.write(format!("{}\n", rng.gen::<u32>()).as_bytes())?;
    }
    Ok(())
}

fn main() -> std::io::Result<()>{
    let args = Args::parse();
    let path = Path::new(&args.directory);
    let _ = fs::create_dir_all(path);

    for _ in 0..args.number_of_files{
        loop {
            let filepath = path.join(random_string(args.filename_length));
            if !filepath.exists() {
                write_file(&filepath, &args)?;
                break;
            }
        }
    }

    Ok(())

}
