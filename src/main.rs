use color_eyre::Result;

mod config;
mod tasks;

fn main() -> Result<()> {
    color_eyre::install()?;

    tasks::solve(config::get()?);

    Ok(())
}
