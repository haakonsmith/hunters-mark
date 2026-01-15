use crate::error::Result;
use clap_complete::Shell;

pub fn init(shell: Shell, prefix: String) -> Result<()> {
    let script = match shell {
        Shell::Bash => include_str!("../../shell/hm.bash").replace("hm", &prefix),
        Shell::Zsh => include_str!("../../shell/hm.zsh").replace("hm", &prefix),
        Shell::Fish => include_str!("../../shell/hm.fish").replace("hm", &prefix),
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
    let bin_name = "hunters-mark";

    generate(shell, &mut cmd, bin_name, &mut io::stdout());

    Ok(())
}
