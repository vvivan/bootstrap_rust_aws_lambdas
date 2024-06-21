use std::{env, io, iter::zip};

use bootstrap_aws_lambdas::{
    build_paths, copy_file, get_executable_files, get_filenames, CopyFileError,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_lenght = args.len();

    if args_lenght != 3 {
        print!("Usage: bootstrap [folder]");
        return Ok(());
    }

    println!("Bootstraping...");

    let files = get_executable_files(&args[1])?;
    let filenames = get_filenames(&files);
    let build_files = build_paths(&filenames, &args[2]);

    let bootstrap_result: Result<Vec<u64>, CopyFileError> = zip(files, build_files)
        .map(|(from, to)| copy_file(from, to))
        .collect();

    let result = match bootstrap_result {
        Ok(_) => "Lambdas bootstrapped successfully!".to_owned(),
        Err(e) => format!("Error bootstrapping lambdas: {:?}", e),
    };

    println!("{:?}", result);

    Ok(())
}
