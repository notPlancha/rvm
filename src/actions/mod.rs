use lazy_static::lazy_static;
use cli_prompts::{
  DisplayPrompt,
  style::{Color, ConfirmationStyle, Formatting, InputStyle, LabelStyle},
};
pub mod init;
pub mod add;
pub mod run;
mod lock;
mod install;


// https://docs.rs/cli-prompts/0.1.0/src/styling/styling.rs.html#38
// Functions because default cant be called in const and static cant move
// So create them everytime is needed works around that
fn label_style() -> LabelStyle {
  LabelStyle::default()
    .prefix("*")
    .prefix_formatting(Formatting::default().foreground_color(Color::Cyan))
    .prompt_formatting(Formatting::default()
      .bold()
      .underline()
      .foreground_color(Color::Magenta),
    )
}
fn input_formatting() -> Formatting {
  Formatting::default().foreground_color(Color::Cyan)
}
fn submitted_formatting() -> Formatting {
  Formatting::default().foreground_color(Color::DarkCyan)
}
pub fn input_style() -> InputStyle {
  InputStyle::default()
    .label_style(label_style())
    .input_formatting(input_formatting())
    .submitted_formatting(submitted_formatting())
}
pub fn confirmation_style() -> ConfirmationStyle {
  ConfirmationStyle::default()
    .label_style(label_style())
    .input_formatting(input_formatting())
    .submitted_formatting(submitted_formatting())
}
