pub enum FG {
  Black = 30,
  Red = 31,
  Green = 32,
  Yellow = 33,
  Blue = 34,
  Magenta = 35,
  Cyan = 36,
  White = 37,
  Default = 39,
  BrightBlack = 90,
  BrightRed = 91,
  BrightGreen = 92,
  BrightYellow = 93,
  BrightBlue = 94,
  BrightMagenta = 95,
  BrightCyan = 96,
  BrightWhite = 97,
}

pub enum BG {
  Black = 40,
  Red = 41,
  Green = 42,
  Yellow = 43,
  Blue = 44,
  Magenta = 45,
  Cyan = 46,
  White = 47,
  Default = 49,
  BrightBlack = 100,
  BrightRed = 101,
  BrightGreen = 102,
  BrightYellow = 103,
  BrightBlue = 104,
  BrightMagenta = 105,
  BrightCyan = 106,
  BrightWhite = 107,
}


pub mod seq {
  pub const ESC: &str = "\x1b";
  pub const CLEAR: &str = "\x1bc";
  pub const RESET: &str = "\x1b[0m";
  pub const BOLD: &str = "\x1b[1m";
  pub const CURSOR_HIDE: &str = "\x1b[?25l";
  pub const CURSOR_SHOW: &str = "\x1b[?25h";
  pub const CURSOR_START: &str = "\x1b[1;1H";

  pub fn fg(color: super::FG) -> String {
    format!("{ESC}[{foreground}m", foreground = color as u8)
  }

  pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("{ESC}[38;2;{r};{g};{b}m")
  }

  pub fn bg(color: super::BG) -> String {
    format!("{ESC}[{background}m", background = color as u8)
  }

  pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("{ESC}[48;2;{r};{g};{b}m")
  }

  pub fn goto(x: u16, y: u16) -> String {
    format!("{ESC}[{y};{x}H")
  }
}

pub fn clear() {
  print!("{}", seq::CLEAR);
}

pub fn fg(color: FG) {
  print!("{}", seq::fg(color));
}

pub fn fg_rgb(r: u8, g: u8, b: u8) {
  print!("{}", seq::fg_rgb(r, g, b));
}

pub fn bg(color: BG) {
  print!("{}", seq::bg(color));
}

pub fn bg_rgb(r: u8, g: u8, b: u8) {
  print!("{}", seq::bg_rgb(r, g, b));
}

pub fn reset() {
  print!("{}", seq::RESET);
}

pub fn show_cursor() {
  print!("{}", seq::CURSOR_SHOW);
}

pub fn goto(x: u16, y: u16) {
  print!("{}", seq::goto(x, y));
}

pub fn size() -> Option<(u16, u16)> {
  let size = terminal_size::terminal_size();

  if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
    Some((w, h))
  } else {
    None
  }
}
