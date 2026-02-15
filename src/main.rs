use crossterm::event::{self, Event, KeyEvent, KeyCode, KeyEventKind};
use ratatui::{self, DefaultTerminal, Frame, layout::Constraint, text::{Line, Text}, widgets::{Block, Paragraph}, style::{Color, Stylize}};
use std::process::Command;

struct App {
    cursor: usize,
    input: String,
    editing: bool,
    exit: bool,
    items: Vec<String>,
    current_screen: CurrentScreen,
}

enum CurrentScreen {
    List,
    Status,
    Input,
}

impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let screen = ratatui::layout::Layout::default()
            .constraints([Constraint::Min(0)])
            .split(frame.area());

        match self.current_screen {
            CurrentScreen::List => {
                let options : Vec<Line> =  self.items.iter()
                    .enumerate()
                    .map(|(i, list_option)| {
                        if i == self.cursor {
                            Line::from(vec![
                                " > ".fg(Color::Rgb(129, 161, 193)).into(),
                                list_option.as_str().bold().underlined().fg(Color::Rgb(129, 161, 193)).into(),
                            ])
                        } else {
                            Line::from(format!("  {}", list_option))
                        }
                    })
                    .collect();

                let list_area = Paragraph::new(Text::from(options))
                    .block(Block::bordered().fg(Color::Rgb(163, 190, 140)).title(" Home "));
                frame.render_widget(list_area, screen[0]);
            }
            CurrentScreen::Status => {
                let git_output = Command::new("git")
                    .arg("status")
                    .output()
                    .expect("Failed to execute command");
                let git = String::from_utf8_lossy(&git_output.stdout);
                let status_area = Paragraph::new(git)
                    .block(Block::bordered().fg(Color::Rgb(163, 190, 140)).title(" Git Status "));

                frame.render_widget(status_area, screen[0]);
            }
            CurrentScreen::Input => {
                let input = Paragraph::new(self.input.as_str())
                    .block(Block::bordered().fg(Color::Rgb(163, 190, 140)).title(" Commit Message "));

                frame.render_widget(input, screen[0]);
            }
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key(key)
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match (self.editing, key.code) {
            (true, KeyCode::Enter) => self.submit(),
            (true, KeyCode::Esc) => self.editing = false,
            (true, KeyCode::Char(c)) => self.input_push(c),
            (true, KeyCode::Backspace) => { self.input.pop(); },

            (false, KeyCode::Char('q')) => self.exit = true,
            (false, KeyCode::Char('j')) => self.move_cursor_down(),
            (false, KeyCode::Char('k')) => self.move_cursor_up(),
            (false, KeyCode::Enter) => self.confirm(),
            (false, KeyCode::Esc) => self.current_screen = CurrentScreen::List,

            _ => {}
        }
    }

    fn submit(&mut self) {
        if !self.input.trim().is_empty() {
            let _add = Command::new("git")
                .arg("add")
                .arg("-A")
                .status();

            let _commit = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("{self.input}")
                .status();
            let _push = Command::new("git")
                .arg("origin")
                .arg("main")
                .status();
            self.input.clear();
            self.editing = false;
            self.current_screen = CurrentScreen::List
        }
    }
    fn input_push(&mut self, c: char) {
        self.input.push(c);
    }
    fn move_cursor_up(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.items.len().saturating_sub(1)
        } else {
            self.cursor -= 1
        }
    }
    fn move_cursor_down(&mut self) {
        if self.cursor == self.items.len().saturating_sub(1) {
            self.cursor = 0
        } else {
            self.cursor += 1 
        }
    }
    fn confirm(&mut self) {
        match self.cursor {
            0 => {
                self.current_screen = CurrentScreen::Input;
                self.editing = true;
            }
            1 => self.current_screen = CurrentScreen::Status,
            _ => {}

        }
    }
}

impl Default for App {

    fn default() -> Self{
        Self {
            cursor: 0,
            input: String::new(),
            editing: false,
            exit: false,
            items: vec!["Push".to_string(), "Status".to_string()],
            current_screen: CurrentScreen::List
        }
    }
    
}


fn main() -> std::io::Result<()>{
    ratatui::run(|terminal| App::default().run(terminal))
}
