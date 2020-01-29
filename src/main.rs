use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[allow(dead_code)]
    #[structopt(subcommand)]
    cmd: Option<Command>
}

#[derive(StructOpt)]
enum Command {
    Init,
    Run,
    Build
}

fn main() {
    let _args = Cli::from_args();
}
