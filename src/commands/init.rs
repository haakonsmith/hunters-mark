use crate::error::Result;
use clap_complete::Shell;

pub fn init(shell: Shell) -> Result<()> {
    let script = match shell {
        Shell::Bash => include_str!("../../shell/hm.bash"),
        Shell::Zsh => include_str!("../../shell/hm.zsh"),
        Shell::Fish => include_str!("../../shell/hm.fish"),
        _ => {
            eprintln!("Unsupported shell: {:?}", shell);
            eprintln!("Supported shells: bash, zsh, fish");
            return Err(color_eyre::eyre::eyre!("Unsupported shell"));
        }
    };

    print!("{}", script);

    Ok(())
}

pub fn completions(shell: Shell) -> Result<()> {
    use clap_complete::generate;
    use std::io;

    let mut cmd = crate::cli::Cli::command();
    let bin_name = "hm";

    generate(shell, &mut cmd, bin_name, &mut io::stdout());

    Ok(())
}
