#[path = "src/cli_input.rs"]
mod cli_input;

use std::{error::Error, path::Path};

use clap::{Command, CommandFactory};
use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_mangen::Man;
use cli_input::CliInput;

fn main() -> Result<(), Box<dyn Error>> {
    let cargo_dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;

    let cargo_dir = Path::new(&cargo_dir);

    let mut cmd = CliInput::command();

    generate_completions(cargo_dir, &mut cmd)?;

    render_man(cargo_dir, cmd)?;

    Ok(())
}

fn generate_completions(cargo_dir: &Path, cmd: &mut Command) -> Result<(), Box<dyn Error>> {
    let comp_dir = cargo_dir.join("completions");
    std::fs::create_dir_all(&comp_dir)?;

    for shell in [Bash, Elvish, Fish, PowerShell, Zsh] {
        clap_complete::generate_to(shell, cmd, "owcli", &comp_dir)?;
    }

    Ok(())
}

fn render_man(cargo_dir: &Path, cmd: Command) -> Result<(), Box<dyn Error>> {
    let man_dir = cargo_dir.join("man");
    std::fs::create_dir_all(&man_dir)?;

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_dir.join("owcli.1"), buffer)?;

    Ok(())
}
