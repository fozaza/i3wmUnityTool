use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self},
    event::{EnableBracketedPaste, EnableFocusChange},
    terminal::{self, enable_raw_mode},
};
mod cmd;
mod ui;

fn main() -> anyhow::Result<()> {
    // let home = std::env!("HOME");
    // let path = format!("{home}/.config/bgrunner/imagePick");
    // let contents = std::fs::read_to_string(path)?;
    // println!("{contents}");
    // Ok(())

    // cmd::sh_reload_feh()?;
    // Ok(())

    enable_raw_mode()?;

    let mut app = ui::TuiApp::init()?;
    app.stdout
        .execute(EnableBracketedPaste)?
        .execute(EnableFocusChange)?
        .execute(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(app.pos.0, app.pos.1))?;

    loop {
        app.size_check()?;
        app.event()?;
        app.update(false)?;
    }
}
