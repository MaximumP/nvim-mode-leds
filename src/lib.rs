use nvim_oxi::api::opts::{CreateAutocmdOpts, CreateCommandOpts};
use nvim_oxi::api::types::{AutocmdCallbackArgs, CommandArgs, CommandNArgs};
use nvim_oxi::{self as oxi, api, Result};

fn on_mode_change(args: AutocmdCallbackArgs) -> Result<bool> {
    let mode_info = args.r#match;
    let parts: Vec<&str> = mode_info.split(':').collect();
    if parts.len() == 2 {
        let old_mode = parts[0];
        let new_mode = parts[1];
        oxi::print!("Changed mode from: {} to: {}", old_mode, new_mode);
    } else {
        oxi::print!("Unexpected mode info: {}", mode_info);
    }

    Ok::<_, oxi::Error>(false)
}

#[oxi::plugin]
fn nvim_mode_leds() -> oxi::Result<bool> {
    // Create a new `Greetings` command.
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("shows a greetings message")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();

    let greetings = |args: CommandArgs| {
        let who = args.args.unwrap_or("from Rust".to_owned());
        let bang = if args.bang { "!" } else { "" };
        print!("Hello {}{}", who, bang);
    };

    api::create_user_command("Greetings", greetings, &opts)?;

    let opts = CreateAutocmdOpts::builder()
        .callback(on_mode_change)
        .desc("Rust plugin for mode change")
        .once(false)
        .build();
    api::create_autocmd(["ModeChanged"], &opts).expect("Error while creating auto command");

    return Ok(false);
}
