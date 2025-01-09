use std::collections::HashMap;

pub enum PadType {
    Dir,
    Num,
}

trait SignalReceiver {
    fn send_signal(&mut self, signal: Option<char>);
}

impl<'a> SignalReceiver for Keypad<'a> {
    fn send_signal(&mut self, signal: Option<char>) {
        if !self.has_error {
            self.just_moved_from = None;
            self.just_moved_to = false;
            self.just_pressed = false;
            if signal.is_some() {
                let last_pos = (self.x, self.y);
                let mut moved = false;
                match signal.unwrap() {
                    'L' => {
                        self.x -= 1;
                        if self.x < 0 {
                            self.has_error = true;
                            return;
                        }
                        moved = true;
                    }
                    'R' => {
                        self.x += 1;
                        if self.x > self.max_x {
                            self.has_error = true;
                            return;
                        }
                        moved = true;
                    }
                    'U' => {
                        self.y -= 1;
                        if self.y < 0 {
                            self.has_error = true;
                            return;
                        }
                        moved = true;
                    }
                    'D' => {
                        self.y += 1;
                        if self.y > self.max_y {
                            self.has_error = true;
                            return;
                        }
                        moved = true;
                    }
                    'A' => {
                        let signal = *self.button_signals.get(&(self.x, self.y)).unwrap();
                        self.send_to.as_mut().unwrap().send_signal(Some(signal));
                        self.just_pressed = true;
                    }

                    _ => {
                        self.has_error = true;
                        return;
                    }
                }
                if (self.x, self.y) == self.invalid {
                    self.has_error = true;
                    return;
                }
                if moved {
                    self.just_moved_to = true;
                    self.just_moved_from = Some(last_pos);
                }
            } else {
                self.send_to.as_mut().unwrap().send_signal(None);
            }
        }
    }
}

impl<'a> SignalReceiver for Display {
    fn send_signal(&mut self, signal: Option<char>) {
        if signal.is_some() {
            self.text.push(signal.unwrap());
        }
    }
}
pub enum Device<'a> {
    Keypad(Keypad<'a>),
    Display(Display),
}

impl<'a> SignalReceiver for Device<'a> {
    fn send_signal(&mut self, signal: Option<char>) {
        match self {
            Device::Keypad(keypad) => keypad.send_signal(signal),
            Device::Display(display) => display.send_signal(signal),
        }
    }
}
pub struct Display {
    pub name: String,
    pub text: String,
}

pub struct Keypad<'a> {
    pub pad_type: PadType,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub button_signals: HashMap<(i32, i32), char>,
    pub invalid: (i32, i32),
    pub has_error: bool,
    pub just_moved_to: bool,
    pub just_moved_from: Option<(i32, i32)>,
    pub just_pressed: bool,
    pub send_to: Option<&'a mut Device<'a>>,
}

impl<'a> Keypad<'a> {
    pub fn new(pad_type: PadType, name: String) -> Self {
        let max_x;
        let max_y;
        let x;
        let y;
        let invalid;
        let button_signals;
        match pad_type {
            PadType::Dir => {
                x = 2;
                y = 0;
                max_x = 2;
                max_y = 1;
                invalid = (0, 0);
                button_signals = HashMap::from([
                    ((1, 0), 'U'),
                    ((2, 0), 'A'),
                    ((0, 1), 'L'),
                    ((1, 1), 'D'),
                    ((2, 1), 'R'),
                ]);
            }
            PadType::Num => {
                x = 2;
                y = 3;
                max_x = 2;
                max_y = 3;
                invalid = (0, 3);
                button_signals = HashMap::from([
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
            }
        }
        Keypad {
            pad_type,
            name,
            x,
            y,
            max_x,
            max_y,
            button_signals,
            invalid,
            has_error: false,
            just_moved_to: false,
            just_moved_from: None,
            just_pressed: false,
            send_to: None,
        }
    }
}
