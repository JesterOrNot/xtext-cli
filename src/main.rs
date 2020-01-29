use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::create_dir;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Init,
    Run,
    Build,
    New {
        #[structopt(parse(from_os_str))]
        directory: PathBuf,
        dsl_extension: Option<String>
    },
}

fn main() {
    let args = Cli::from_args();
    match args.cmd {
        Command::Init => println!("WIP"),
        Command::Build => println!("WIP"),
        Command::Run => println!("WIP"),
        Command::New {directory: n, dsl_extension: _m} => {
            let dsl_extension = match _m {
                Some(n) => String::from(".") + n.as_str(),
                None => String::from(".mydsl")
            };
            let option = create_dir(n);
            match option {
                Ok(()) => {},
                Err(_n) => println!("\x1b[1;31mError:\x1b[0m directory already exists"),
            }
            return;
        }
    }
}
