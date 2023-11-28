use super::consts::{C_BLUE, C_DEFAULT_BG, C_DEFAULT_FG, C_GREEN, C_RED};
use super::utils::Styles;
use std::borrow::Cow;
use tmux_interface::{
    options::{SetGlobalSessionOptions, SetSessionOptionsTr},
    Align, Status, Style, TmuxCommands,
};

pub fn green_right_arrow<'a, S>(s: S) -> String
where
    S: Into<Cow<'a, str>>,
{
    let s = s.into();
    let default = Styles::from([Style::Bg(C_DEFAULT_BG), Style::Fg(C_DEFAULT_FG)]);
    let bg_green_fg_black = Styles::from([Style::Bg(C_GREEN), Style::Fg(C_DEFAULT_BG)]);
    let bg_green = Styles::from([Style::Bg(C_GREEN), Style::Fg(C_DEFAULT_FG)]);
    let fg_green = Styles::from([Style::Bg(C_DEFAULT_BG), Style::Fg(C_GREEN)]);
    format!("{bg_green_fg_black}\u{e0b0} {bg_green}{s} {fg_green}{default}")
}

pub fn options_command() -> TmuxCommands<'static> {
    let default = Styles::from([Style::Bg(C_DEFAULT_BG), Style::Fg(C_DEFAULT_FG)]);
    let left_accent = Styles::from([Style::Bg(C_BLUE), Style::Fg(C_DEFAULT_FG)]);
    let right_accent = Styles::from([Style::Bg(C_RED), Style::Fg(C_DEFAULT_FG)]);
    let session_name_display = green_right_arrow("#S");
    let windows_display = "#{W:#{E:window-status-format},#{E:window-status-current-format}}";
    let row0left = format!("{left_accent}  os {session_name_display} {windows_display}");
    let user_host = "#{E:user}@#H";
    let row0right = format!("{user_host}  {right_accent} ");
    let right = Styles::from([Style::Align(Align::Right)]);
    let row0 = format!("{row0left}{right}{row0right}");
    let tn: Option<Cow<'static, str>> = None;
    SetGlobalSessionOptions::new()
        .status(tn.clone(), Some(Status::TwoRows))
        .status_interval(tn.clone(), Some(1))
        .status_format(tn.clone(), Some([row0]))
        .build()
}