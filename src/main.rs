use std::env;

use bootstrap_aws_lambdas::{copy_pairs, create_source_destination_pairs, get_executable_files};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_lenght = args.len();

    if args_lenght != 3 {
        println!("\nUsage:\n");
        println!("bootstrap_aws_lambdas <source_path> <target_path>");
        return;
    }

    let result = get_executable_files(&args[1])
        .map(|path_bufs| {
            path_bufs
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<String>>()
        })
        .map(|file| create_source_destination_pairs(file, &args[2]))
        .and_then(copy_pairs);

    match result {
        Ok(_) => Some(()),
        Err(e) => {
            println!("{e}");
            None
        }
    };
}
