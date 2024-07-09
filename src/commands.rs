mod build;
mod new;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    New {
        name: String,
    },
    Build {
        #[clap(short, long)]
        verbose: bool,
        #[clap(short, long)]
        run: bool,
    },
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::New { name } => {
                new::new_command(name.to_owned())?;
            }
            Commands::Build { verbose, run } => {
                build::build_command(verbose.to_owned(), run.to_owned())?;
            }
        }

        Ok(())
    }
}
