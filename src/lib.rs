use std::env;
use std::path::PathBuf;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use crate::error::Result;
use crate::parse::pap::ExtCmd;
use crate::state::Config;

pub mod error;
pub mod parse;
mod shell;
pub mod state;
pub mod utils;

pub use shell::{print_alias, shell_from_str, Shell};

pub fn create_modified_cmd(ext_cmd: &ExtCmd) -> Result<String> {
    let config = Config::load()?;

    let id_path = config.get(ext_cmd.id)?.path().to_string_lossy().to_owned();

    let mut args = ext_cmd.args.clone();
    if ext_cmd.use_pos {
        args = args
            .iter()
            .map(|x| {
                if x == "$" {
                    From::from(id_path.clone())
                } else {
                    From::from(x)
                }
            })
            .collect();
    } else {
        args.insert(0, From::from(id_path));
    }

    let mut cmd = ext_cmd.cmd.clone();
    cmd.push(' ');
    cmd.push_str(&args.join(" "));
    Ok(cmd)
}

pub fn list() -> Result<()> {
    let config = Config::load()?;

    println!("Clipboard:\n");

    let mut table = Table::new();

    table.set_titles(row!["Id", "Path"]);

    for (i, v) in config.iter().enumerate() {
        table.add_row(row![i, v.path().to_string_lossy()]);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.printstd();

    Ok(())
}

pub fn add(files: Vec<PathBuf>) -> Result<()> {
    let cur_dir = env::current_dir()?;

    let mut config = Config::load()?;
    config.extend(cur_dir, files)?;
    config.save()?;
    Ok(())
}

pub fn clear() -> Result<()> {
    Config::empty().save()?;
    Ok(())
}
