use anyhow::Result;
use cargo_generate::{ generate, Args};
use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct PublicArgs {
    /// Git repository to clone template from. Can be a URL (like
    /// `https://github.com/rust-cli/cli-template`), a path (relative or absolute), or an
    /// `owner/repo` abbreviated GitHub URL (like `rust-cli/cli-template`).
    /// Note that cargo generate will first attempt to interpret the `owner/repo` form as a
    /// relative path and only try a GitHub URL if the local path doesn't exist.
    pub git: String,
    /// Branch to use when installing from git
    pub branch: Option<String>,
    /// Directory to create / project name; if the name isn't in kebab-case, it will be converted
    /// to kebab-case unless `--force` is given.
    pub name: Option<String>,
    /// Don't convert the project name to kebab-case before creating the directory.
    /// Note that cargo generate won't overwrite an existing directory, even if `--force` is given.
    pub force: bool,
    /// Enables more verbose output.
    
    pub verbose: bool,
}

fn main() -> Result<()> {
    const FUNCTION: &str = include_str!("template/function.rs");
    const _RELEASE_BUILD: &str = include_str!("template/Release.Dockerfile");
    let _build_image = "quay.io/roche/default:1.0.0";
    let default_project = "https://github.com/roche-rs/default";
    let mongodb_project = "https://github.com/roche-rs/mongodb";

    let matches = App::new("roche")
        .subcommand(
            App::new("init").about("Generates a project").arg(
                Arg::new("template")
                    .about("template name 'default', 'mongodb' or git location. If not supplied a functions.rs file is generated.")
                    .index(1)
                    .required(false)
                    .multiple_values(true)
            )
            .arg(
                Arg::new("name")
                    .about("Name of the project")
                    .required(false)
                    .takes_value(true)
                    .short('n')
                    .long("name"),
            )
            .arg(
                Arg::new("branch")
                    .about("Branch to use when installing from git. Defaults to main.")
                    .required(false)
                    .takes_value(true)
                    .short('b')
                    .long("branch"),
            )
            .arg(
                Arg::new("force")
                    .about("Don't convert the project name to kebab-case before creating the directory.")
                    .required(false)
                    .takes_value(false)
                    .short('f')
                    .long("force"),
            )
            .arg(
                Arg::new("verbose")
                    .about("Enables more verbose output.")
                    .required(false)
                    .takes_value(true)
                    .short('v')
                    .long("verbose"),
            )
            ,
        )
        .get_matches();

    if matches.is_present("init") {
        if let Some(init_matches) = matches.subcommand_matches("init") {
            match init_matches.value_of("template").unwrap_or_default() {
                "default" => {
                    //let branch = String::from("main").unwrap();
                    let args_exposed : PublicArgs = PublicArgs { 
                        git : default_project.to_string(),
                        branch : Some(String::from("main")),
                        name : None,
                        force : true,
                        verbose : true,
                    };

                    let args: Args = unsafe {
                        std::mem::transmute(args_exposed)
                    };
                    
                    generate(args)?

                },
                "mongodb" => {
                    //let branch = String::from("main").unwrap();
                    let args_exposed : PublicArgs = PublicArgs { 
                        git : mongodb_project.to_string(),
                        branch : Some(String::from("main")),
                        name : None,
                        force : true,
                        verbose : true,
                    };

                    let _args: Args = unsafe {
                        std::mem::transmute(args_exposed)
                    };
                    println!("MONGODB TEMPLATE NOT IMPLEMENTED")
                    //generate(args)?

                }
                &_ => {
                    if init_matches.value_of("template").unwrap_or_default().contains("https://") {
                        let name = init_matches.value_of("name").map(ToOwned::to_owned);
                        let branch = match init_matches.value_of("name").map(ToOwned::to_owned) {
                            Some(b) =>  b,
                            None => String::from("main")
                        };

                        let force : bool = init_matches.value_of_t("force").unwrap_or(false);
                        let verbose : bool = init_matches.value_of_t("verbose").unwrap_or(false);
                        
                        let args_exposed : PublicArgs = PublicArgs { 
                            git : init_matches.value_of("template").unwrap_or_default().to_string(),
                            branch : Some(branch),
                            name : name,
                            force : force,
                            verbose : verbose,
                        };  
                        let args: Args = unsafe {
                            std::mem::transmute(args_exposed)
                        };
                        
                        generate(args)?
                    } else {
                        // init called but with no options so just generating a function.
                        let mut file = File::create("functions.rs")?;
                        file.write_all(FUNCTION.as_bytes())?
                    }
                }
            }
        }
    }

    Ok(())
}
