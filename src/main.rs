use dialoguer::{theme::ColorfulTheme, console::Style, Select};

fn main() -> anyhow::Result<()> {
    eprintln!("Welcome to the test program!");

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let choice = Select::with_theme(&theme)
            .with_prompt("Do you want to install with all default configuration values?")
            .default(0)
            .item("Yes, install with defaults")
            .item("No, let me chose custom install options")
            .interact_opt()?;

    eprintln!("The choice is {:?}.", choice);

    Ok(())
}
