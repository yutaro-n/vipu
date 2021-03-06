use clap::Clap;
use git2::{Repository, Error};

#[derive(Clap)]
#[clap(version = "0.1", author = "Yutaro N. <yutaro.ngsw@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about="Install new plugins")]
    Install(Install),
}

#[derive(Clap)]
struct Install {
    #[clap(long, name="BRANCH_NAME", about="Select sepecific branch")]
    branch: Option<String>,
    #[clap(long, name="BUILD_CMD", about="Build command for build plugin")]
    build: Option<String>,
    #[clap(name = "plugin", multiple=true)]
    plugin: Vec<String>
}



fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Install(op) => {
            match op.branch {
                Some(br) => println!("branch : {}", br),
                None => println!("branch : main or master")
            }
            match op.build {
                Some(bu) => println!("build cmd : {}", bu),
                None => println!("build cmd : nashi")
            }
            if op.plugin.is_empty() {
                println!("please input plugin name");
            } else {
                println!("plugin : {:?}", op.plugin);
            }
        },
    }
}
