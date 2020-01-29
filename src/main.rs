use structopt::StructOpt;

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
}

fn main() {
    let args = Cli::from_args();
    match args.cmd {
        Command::Init => println!("WIP"),
        Command::Build => println!("WIP"),
        Command::Run => println!("WIP")
    }
}
