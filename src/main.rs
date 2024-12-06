use color_eyre::Result;

mod config;
mod tasks;

fn main() -> Result<()> {
    // Install panic hooks for pretty error messages.
    color_eyre::install()?;

    // Read the config file.
    let input = config::get()?;
    // Run the algorithms.
    tasks::solve(input);

    // Exit successfully.
    Ok(())
}
