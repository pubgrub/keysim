//! # [Ratatui] Block example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::{owo_colors::OwoColorize, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
    DefaultTerminal, Frame,
};
use std::collections::HashMap;
use std::fs::read_to_string;

struct App {
    should_exit: bool,
    key_pressed: Option<KeyCode>,
    pad_x: Vec<i32>,
    pad_y: Vec<i32>,
    pad_max_x: Vec<i32>,
    pad_max_y: Vec<i32>,
    pad_invalid: Vec<(i32, i32)>,
    pad_has_error: Vec<bool>,
    pad_just_moved_to: Vec<bool>,
    pad_just_moved_from: Vec<Option<(i32, i32)>>,
    pad_just_pressed: Vec<bool>,
    input_pointer: i32,
    input_lines: Vec<String>,
    input_count: usize,
    akt_line: usize,

    input: String,
}
impl App {
    fn new() -> Self {
        let pad_x = vec![2, 2, 2, 2];
        let pad_y = vec![0, 0, 0, 3];
        let pad_max_x = vec![2, 2, 2, 2];
        let pad_max_y = vec![1, 1, 1, 3];
        let pad_invalid = vec![(0, 0), (0, 0), (0, 0), (0, 3)];
        let pad_has_error = vec![false, false, false, false];
        let pad_just_moved_to = vec![false, false, false, false];
        let pad_just_moved_from: Vec<Option<(i32, i32)>> = vec![None, None, None, None];
        let pad_just_pressed = vec![false, false, false, false];

        Self {
            should_exit: false,
            key_pressed: None,
            pad_x,
            pad_y,
            pad_max_x,
            pad_max_y,
            pad_invalid,
            pad_has_error,
            pad_just_moved_to,
            pad_just_moved_from,
            pad_just_pressed,
            input: String::new(),
            input_pointer: 0,
            input_lines: vec![],
            input_count: 0,
            akt_line: 0,
        }
    }
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.input_lines = load_file();
        self.input_count = self.input_lines.len();
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let KeyCode::Char(c) = key.code {
                    if c == 'q' {
                        self.should_exit = true;
                    } else {
                        let c_i = c.to_digit(10);
                        if c_i.is_some() && c_i.unwrap() > 0 {
                            let n = c_i.unwrap() - 1;
                            if n < self.input_count as u32 {
                                self.akt_line = (n) as usize;
                                self.input_pointer = self
                                    .input_pointer
                                    .min(self.input_lines[n as usize].len() as i32);
                            }
                        }
                    }
                } else {                  self.key_pressed = Some(key.code);
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        //        let command_string = "DLLARRUA";
        let command_string = self.input_lines[self.akt_line].clone();
        let commands: Vec<char> = command_string.chars().map(|c| c).collect();

        let dir_button_signals = HashMap::from([
            ((1, 0), 'U'),
            ((2, 0), 'A'),
            ((0, 1), 'L'),
            ((1, 1), 'D'),
            ((2, 1), 'R'),
        ]);
        let mut dir_signal_button = HashMap::new();
        for (k, v) in dir_button_signals.iter() {
            dir_signal_button.insert(*v, *k);
        }
        let dir_valid_signals = vec!['U', 'A', 'L', 'D', 'R'];
        let num_button_signals = HashMap::from([
            ((0, 0), '7'),
            ((1, 0), '8'),
            ((2, 0), '9'),
            ((0, 1), '4'),
            ((1, 1), '5'),
            ((2, 1), '6'),
            ((0, 2), '1'),
            ((1, 2), '2'),
            ((2, 2), '3'),
            ((1, 3), '0'),
            ((2, 3), 'A'),
        ]);
        let num_valid_signals = vec!['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        if self.key_pressed.is_some() {
            self.input_pointer = match self.key_pressed.unwrap() {
                KeyCode::Char(' ') | KeyCode::Right => {
                    (self.input_pointer + 1).min(command_string.len() as i32)
                }
                KeyCode::Left => (self.input_pointer - 1).max(0),

                _ => self.input_pointer,
            };
            self.key_pressed = None;
        }

        self.pad_x = vec![2, 2, 2, 2];
        self.pad_y = vec![0, 0, 0, 3];
        self.pad_has_error = vec![false, false, false, false];
        self.pad_just_moved_to = vec![false, false, false, false];
        self.pad_just_moved_from = vec![None, None, None, None];
        self.pad_just_pressed = vec![false, false, false, false];

        let mut pad_output: String = String::new();

        let mut signal: Option<&char>;
        'a: for c in commands[0..self.input_pointer as usize].iter() {
            if self.pad_has_error[0] {
                break;
            }

            self.pad_just_moved_from[0] = None;
            self.pad_just_moved_to[0] = false;
            self.pad_just_pressed[0] = false;
            if dir_valid_signals.contains(c) {
                (self.pad_x[0], self.pad_y[0]) = *dir_signal_button.get(&c).unwrap();
                self.pad_just_pressed[0] = true;
                signal = Some(c);
            } else {
                self.pad_has_error[0] = true;
                break 'a;
            }
            for pad in 1..4 {
                let last_pos = (self.pad_x[pad], self.pad_y[pad]);
                self.pad_just_moved_to[pad] = false;
                self.pad_just_moved_from[pad] = None;
                self.pad_just_pressed[pad] = false;
                if signal.is_some() {
                    //                    println!("signal:{}", signal.unwrap());
                    if !dir_valid_signals.contains(signal.unwrap()) {
                        self.pad_has_error[pad] = true;
                        break 'a;
                    }
                    let mut moved = false;
                    match signal.unwrap() {
                        'L' => {
                            self.pad_x[pad] -= 1;
                            if self.pad_x[pad] < 0
                                || self.pad_invalid[pad] == (self.pad_x[pad], self.pad_y[pad])
                            {
                                self.pad_has_error[pad] = true;
                                break 'a;
                            }
                            moved = true;
                            signal = None;
                        }
                        'R' => {
                            self.pad_x[pad] += 1;
                            if self.pad_x[pad] > self.pad_max_x[pad]
                                || self.pad_invalid[pad] == (self.pad_x[pad], self.pad_y[pad])
                            {
                                self.pad_has_error[pad] = true;
                                break 'a;
                            }
                            moved = true;
                            signal = None;
                        }
                        'U' => {
                            self.pad_y[pad] -= 1;
                            if self.pad_x[pad] < 0
                                || self.pad_invalid[pad] == (self.pad_x[pad], self.pad_y[pad])
                            {
                                self.pad_has_error[pad] = true;
                                break 'a;
                            }
                            moved = true;
                            signal = None;
                        }
                        'D' => {
                            self.pad_y[pad] += 1;
                            if self.pad_y[pad] > self.pad_max_y[pad]
                                || self.pad_invalid[pad] == (self.pad_x[pad], self.pad_y[pad])
                            {
                                self.pad_has_error[pad] = true;
                                break 'a;
                            }
                            moved = true;
                            signal = None;
                        }
                        'A' => {
                            if pad == 3 {
                                pad_output = pad_output
                                    + num_button_signals
                                        .get(&(self.pad_x[pad], self.pad_y[pad]))
                                        .unwrap()
                                        .to_string()
                                        .as_str();
                            }

                            self.pad_just_pressed[pad] = true;
                            signal = match pad {
                                0..=2 => Some(
                                    dir_button_signals
                                        .get(&(self.pad_x[pad], self.pad_y[pad]))
                                        .unwrap(),
                                ),
                                3 => Some(
                                    num_button_signals
                                        .get(&(self.pad_x[pad], self.pad_y[pad]))
                                        .unwrap(),
                                ),
                                _ => None,
                            };
                            moved = true;
                        }
                        _ => {}
                    }
                    if moved {
                        self.pad_just_moved_to[pad] = true;
                        self.pad_just_moved_from[pad] = Some(last_pos);
                    }
                }
            }
        }

        /////////////////////////////////////////////////
        //  Layout and Rendering  ///////////////////////
        /////////////////////////////////////////////////

        let empty_paragraph = Paragraph::new("");

        // layout main areas

        let [input_area, pad_area, nav_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(14),
            Constraint::Length(3),
        ])
        .spacing(1)
        .areas(inner_rect(&frame.area()));

        // layout input buttons

        let input_buttons: Vec<Rect> = Layout::horizontal([Constraint::Length(7); 10])
            .split(input_area)
            .to_vec();

        // layout pads

        let pads_rect = Layout::horizontal([Constraint::Length(25); 5])
            .spacing(3)
            .split(pad_area);

        // layout output and steps

        let [output_rect, steps_rect] =
            Layout::vertical([Constraint::Length(3); 2]).areas(pads_rect[4]);

        // layout buttons

        let two_rows = Layout::vertical([Constraint::Length(3); 2]).margin(1);
        let four_rows = Layout::vertical([Constraint::Length(3); 4]).margin(1);
        let three_cols = Layout::horizontal([Constraint::Length(7); 3]).horizontal_margin(1);

        let mut buttons: Vec<Rect> = vec![];
        for i in 0..4 {
            let rows;
            if i < 3 {
                rows = two_rows.clone();
            } else {
                rows = four_rows.clone();
            }
            for row in rows.split(pads_rect[i]).iter() {
                let button_row = three_cols.split(*row);
                for b in button_row.iter() {
                    buttons.push(b.clone());
                }
            }
        }

        // render outer frame

        let block = Block::bordered()
            .title(Line::from("  Key Sim 2024  ").centered())
            .title(Line::from(" help: ? ").right_aligned());
        frame.render_widget(empty_paragraph.clone().block(block), frame.area());

        // render input buttons

        let inactive_line_style = Style::new().white().on_black().bold();
        let active_line_style = Style::new().black().on_green().bold();
        let mut style;
        for i in 0..9 {
            if i < self.input_count {
                let text = (i + 1).to_string();
                if i == self.akt_line {
                    style = active_line_style;
                } else {
                    style = inactive_line_style;
                }
                render_button(
                    "".to_string(),
                    &Paragraph::new(text).centered(),
                    Borders::ALL,
                    frame,
                    input_buttons[i],
                    style,
                );
            }
        }

        // render pads

        let box_normal_style = Style::new().white().on_black();
        let box_error_style = Style::new().black().on_red();
        let pad_boxes_names = vec!["manual pad", "second pad", "third pad", "num pad"];
        let mut style;
        for i in 0..4 {
            if self.pad_has_error[i] {
                style = box_error_style;
            } else {
                style = box_normal_style;
            };
            render_box(
                pad_boxes_names[i].to_string(),
                Borders::ALL,
                style,
                frame,
                pads_rect[i],
            );
        }

        // render output

        render_borders(
            "output".to_string(),
            &Paragraph::new(pad_output),
            Borders::ALL,
            frame,
            output_rect,
        );

        // render steps

        render_borders(
            "steps".to_string(),
            &Paragraph::new(self.input_pointer.to_string()),
            Borders::ALL,
            frame,
            steps_rect,
        );

        // render buttons

        let dir_labels = vec!["", "^", "A", "<", "v", ">"];
        let num_labels = vec!["7", "8", "9", "4", "5", "6", "1", "2", "3", "", "0", "A"];

        let normal_style = Style::new().white().on_black().bold();
        let pos_style = Style::new().black().on_green().bold();
        let pressed = Style::new().black().on_red().bold();
        let moved_from = Style::new().green().on_black().bold();

        let mut style;
        for (i, b) in buttons.iter().enumerate() {
            let text;
            let pad: usize = (i / 6).min(3);
            if i < 18 {
                text = dir_labels[i % 6];
            } else {
                text = num_labels[i - 18];
            }

            if i as i32 == pad as i32 * 6 + self.pad_x[pad] + self.pad_y[pad] * 3 {
                if self.pad_just_pressed[pad] {
                    style = pressed;
                } else {
                    style = pos_style;
                }
            } else {
                if self.pad_just_moved_from[pad].is_some()
                    && i as i32 - pad as i32 * 6
                        == self.pad_just_moved_from[pad].unwrap().0
                            + self.pad_just_moved_from[pad].unwrap().1 * 3
                {
                    style = moved_from;
                } else {
                    style = normal_style;
                }
            }
            render_button(
                "".to_string(),
                &Paragraph::new(text).centered(),
                Borders::ALL,
                frame,
                *b,
                style,
            );
        }

        // render nav

        let nav_string = " ".to_string() + command_string.as_str();

        let mut pre_cursor = "";
        if self.input_pointer > 0 {
            pre_cursor = &nav_string[0..self.input_pointer as usize];
        }
        let cursor = &nav_string[self.input_pointer as usize..self.input_pointer as usize + 1];
        let post_cursor = &nav_string[self.input_pointer as usize + 1..];

        // define areas

        let pre_rect: Rect = Rect {
            x: nav_area.x + 1,
            y: nav_area.y + 1,
            width: pre_cursor.len() as u16,
            height: 1,
        };
        let cursor_rect: Rect = Rect {
            x: pre_rect.x + pre_rect.width,
            y: nav_area.y + 1,
            width: 1,
            height: 1,
        };
        let post_rect: Rect = Rect {
            x: cursor_rect.x + 1,
            y: cursor_rect.y,
            width: nav_area.width - pre_rect.width - 3,
            height: 1,
        };

        let pre_para = Paragraph::new(pre_cursor).style(Style::new().white().on_blue());
        let cursor_para = Paragraph::new(cursor).style(Style::new().black().on_green());
        let post_para = Paragraph::new(post_cursor).style(Style::new().white().on_blue());
        frame.render_widget(pre_para, pre_rect);
        frame.render_widget(cursor_para, cursor_rect);
        frame.render_widget(post_para, post_rect);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

/// Calculate the layout of the UI elements.
///
/// Returns a tuple of the title area and the main areas.

fn inner_rect(r: &Rect) -> Rect {
    Rect {
        x: r.x + 1,
        y: r.y + 1,
        width: r.width - 2,
        height: r.height - 2,
    }
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Block example. Press q to quit")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn placeholder_paragraph() -> Paragraph<'static> {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
    Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true })
}

fn render_borders(
    title: String,
    paragraph: &Paragraph,
    border: Borders,
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::new()
        .borders(border)
        .title(title)
        .padding(Padding::new(1, 0, 0, 0));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn render_box(title: String, border: Borders, style: Style, frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(border)
        .title(title)
        .style(style)
        .padding(Padding::new(1, 1, 1, 1));
    frame.render_widget(Paragraph::new("").block(block), area);
}

fn render_button(
    title: String,
    paragraph: &Paragraph,
    border: Borders,
    frame: &mut Frame,
    area: Rect,
    style: Style,
) {
    let block = Block::new()
        .borders(border)
        .title(title)
        .style(style)
        .padding(Padding::new(0, 0, 0, 0));
    frame.render_widget(paragraph.clone().block(block), area);
}

fn load_file() -> Vec<String> {
    let filename = "keypad2024input.txt";
    let mut lines = Vec::new();
    //println!("{:?}", path);
    for line in read_to_string(filename).unwrap().lines() {
        let mut l = line.to_string();
        if l.chars().all(|c| "<>^vA".contains(c)) {
            l = l.replacen("v", "D", 99999);
            l = l.replacen("^", "U", 99999);
            l = l.replacen("<", "L", 99999);
            l = l.replacen(">", "R", 99999);
        }
        if l.chars().all(|c| "LRUDA".contains(c)) {
            if lines.len() < 9 {
                lines.push(l);
            }
        }
    }
    lines
}
