use std::{env, io, iter::zip, path::PathBuf};

use bootstrap_aws_lambdas::{build_paths, copy_file, get_executable_files, get_filenames};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_lenght = args.len();

    println!("app args: {args:?}");
    println!("args length: {args_lenght:?}");

    if args_lenght != 3 {
        print!("Usage: bootstrap [folder]");
        return Ok(());
    }

    println!("Bootstraping...");

    let files = get_executable_files(&args[1])?;
    let filenames = get_filenames(&files);
    let build_files = build_paths(&filenames, &args[2]);
    let zipped: Vec<(PathBuf, String)> = zip(files, build_files).collect();

    let result = zipped
        .iter()
        .map(|(from, to)| copy_file(from, to))
        .collect::<Vec<_>>();

    println!("Result: {:?}", result);

    Ok(())
}
