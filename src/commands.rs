mod add;
mod build;
mod new;
mod remove;
mod version;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    New {
        name: String,
    },
    Build {
        #[clap(short, long)]
        run: bool,
    },
    Add {
        module: String,
    },
    Remove {
        module: String,
    },
    Version {},
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::New { name } => {
                new::new_command(name.to_owned())?;
            }
            Commands::Build { run } => {
                build::build_command(run.to_owned())?;
            }
            Commands::Add { module } => {
                add::add_command(module.to_owned())?;
            }
            Commands::Remove { module } => {
                remove::remove_command(module.to_owned())?;
            }
            Commands::Version {} => {
                version::version_command();
            }
        }

        Ok(())
    }
}
