use crossterm::{
  cursor,
  event::{self, KeyCode, KeyEvent},
  style::{self, Color, SetForegroundColor, ResetColor},
  terminal::{self, ClearType},
  ExecutableCommand,
};
use std::io::{self, Write};

fn main() -> io::Result<()> {
  let mut stdout = io::stdout();

  let menu_items = vec!["New game", "Load game", "Options", "Exit"];
  let mut selected_index = 0;
  let x_index: u16 = 40;

  terminal::enable_raw_mode()?;
  stdout.execute(terminal::EnterAlternateScreen)?;
  stdout.execute(terminal::SetSize(100,60))?;


  loop {
      let mut y_index: u16 = 30;
      stdout.execute(terminal::Clear(ClearType::All))?;
      stdout.execute(cursor::MoveTo(x_index, y_index))?;
      stdout.execute(SetForegroundColor(Color::Red))?;

      println!("Dungeon Expedition!");
      stdout.execute(ResetColor)?;
      y_index = y_index +2;
      for (i, item) in menu_items.iter().enumerate() {
          if i == selected_index {
              stdout.execute(style::SetForegroundColor(Color::Yellow))?;
              stdout.execute(cursor::MoveTo(x_index, y_index))?;
              writeln!(stdout, "> {}", item)?;
              stdout.execute(ResetColor)?;
          } else {
              stdout.execute(cursor::MoveTo(x_index, y_index))?;
              writeln!(stdout, "  {}", item)?;
          }
          y_index = y_index + 1;
      }

      stdout.flush()?;

      if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
          match code {
              KeyCode::Up => {
                  if selected_index > 0 {
                      selected_index -= 1;
                  }
              }
              KeyCode::Down => {
                  if selected_index < menu_items.len() - 1 {
                      selected_index += 1;
                  }
              }
              KeyCode::Enter => {
                  break;
              }
              KeyCode::Esc => {
                  break;
              }
              _ => {}
          }
      }
  }

  terminal::disable_raw_mode()?;
  stdout.execute(terminal::LeaveAlternateScreen)?;

  println!("Selected option: {}", menu_items[selected_index]);

  Ok(())
}
