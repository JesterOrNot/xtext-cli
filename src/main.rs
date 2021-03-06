use std::fs::create_dir;
use std::fs::File;
use std::path::PathBuf;
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
    New {
        #[structopt(parse(from_os_str))]
        directory: PathBuf,
        package_name: Option<String>,
        dsl_extension: Option<String>,
    },
}

fn main() {
    let args = Cli::from_args();
    match args.cmd {
        Command::Init => println!("WIP"),
        Command::Build => println!("WIP"),
        Command::Run => println!("WIP"),
        Command::New {
            directory: n,
            package_name: f,
            dsl_extension: m,
        } => {
            let dsl_extension = match m {
                Some(n) => String::from(".") + n.as_str(),
                None => String::from(".mydsl"),
            };
            let project_name = match f {
                Some(f) => f,
                None => String::from("org.xtext.example.mydsl"),
            };
            let option = create_dir(&n);

            match option {
                Ok(()) => {}
                Err(_n) => {
                    println!("\x1b[1;31mError:\x1b[0m directory already exists");
                    return;
                }
            }
            create_dir(&n.join(&project_name)).unwrap();
            create_dir(&n.join(&project_name).join("src")).unwrap();
            create_dir(&n.join(&project_name).join("src-gen")).unwrap();
            create_dir(&n.join(&project_name).join("xtend-gen")).unwrap();
            create_dir(&n.join(project_name.to_owned() + &String::from(".ide"))).unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ide"))
                    .join("src"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ide"))
                    .join("src-gen"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ide"))
                    .join("xtend-gen"),
            )
            .unwrap();
            create_dir(&n.join(project_name.to_owned() + &String::from(".tests"))).unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".tests"))
                    .join("src"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".tests"))
                    .join("src-gen"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".tests"))
                    .join("xtend-gen"),
            )
            .unwrap();
            create_dir(&n.join(project_name.to_owned() + &String::from(".ui"))).unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ui"))
                    .join("src"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ui"))
                    .join("src-gen"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ui"))
                    .join("xtend-gen"),
            )
            .unwrap();
            create_dir(&n.join(project_name.to_owned() + &String::from(".ui.tests"))).unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ui.tests"))
                    .join("src"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ui.tests"))
                    .join("src-gen"),
            )
            .unwrap();
            create_dir(
                &n.join(project_name.to_owned() + &String::from(".ui.tests"))
                    .join("xtend-gen"),
            )
            .unwrap();
            let mut file = File::create(
                n.to_str().unwrap().to_string()
                    + &String::from("/")
                    + &project_name.to_string()
                    + &String::from("/src/")
                    + &String::from(&dsl_extension[1..])
                    + &String::from(".xtext"),
            )
            .unwrap();
            return;
        }
    }
}
