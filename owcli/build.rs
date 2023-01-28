#[path = "src/cli_input.rs"]
mod cli_input;

use clap::CommandFactory;
use clap_complete::{
    generate_to,
    Shell::{Bash, Elvish, Fish, PowerShell, Zsh},
};

// TODO: refactor, split into smaller functions
fn main() -> std::io::Result<()> {
    let cargo_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or(std::io::ErrorKind::NotFound)?;

    let man_dir = std::path::PathBuf::from(&cargo_dir).join("man");
    std::fs::create_dir_all(&man_dir)?;

    let cmd = cli_input::CliInput::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_dir.join("owcli.1"), buffer)?;

    let comp_dir = std::path::PathBuf::from(&cargo_dir).join("completions");
    std::fs::create_dir_all(&comp_dir)?;
    
    // TODO: figure out if there is a way to not call comand() twice
    let mut cmd = cli_input::CliInput::command();

    for shell in [Bash, Elvish, Fish, PowerShell, Zsh] {
        generate_to(shell, &mut cmd, "owcli", &comp_dir)?;
    }

    Ok(())
}
