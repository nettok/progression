extern crate termion;

use termion::{clear, color, cursor, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::iter::FromIterator;
use std::collections::LinkedList;
use std::io::{Write, stdout, stdin};
use std::time::Duration;
use std::thread;

struct Message {
    sender: String,
    data: String
}

struct Progression {
    messages: LinkedList<Message>,
    input_buffer: Vec<char>
}

impl Progression {
    fn append_msg(&mut self, msg: Message) {
        self.messages.push_back(msg)
    }

    fn input_char(&mut self, c: char) {
        match c {
            '\n' => {
                let data = String::from_iter(self.input_buffer.clone());
                if !data.is_empty() {
                    self.append_msg(Message { sender: "me".to_string(), data: data });
                }
                self.input_buffer.clear();
            },
            c => self.input_buffer.push(c)
        }
    }

    fn backspace(&mut self) {
        self.input_buffer.pop();
    }
}

fn main() {
    let mut progression = Progression { messages: LinkedList::new(), input_buffer: Vec::new() };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    draw_progression(&mut stdout, &progression);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc       => break,
            Key::Char(c)   => progression.input_char(c),
            Key::Backspace => progression.backspace(),
            _              => continue
        };

        draw_progression(&mut stdout, &progression);
    }

    writeln!(stdout, "Fin!\r");
}

fn draw_progression(w: &mut Write, p: &Progression) {
    clear(w);
    draw_messages(w, p.messages.iter());
    draw_input_buffer(w, p.input_buffer.as_slice());
}

fn clear(w: &mut Write) {
    write!(w, "{}\r", termion::clear::All);
    w.flush().unwrap();
}

fn draw_messages<'a, I>(w: &mut Write, messages: I)
    where I: Iterator<Item = &'a Message>
{
    write!(w, "{}", cursor::Goto(1, 1));

    for msg in messages {
        writeln!(w, "{bold}{}:{reset} {}\r",
                 msg.sender,
                 msg.data,
                 bold = style::Bold,
                 reset = style::Reset,
        );
    }
}

fn draw_input_buffer(w: &mut Write, chars: &[char]) {
    writeln!(w, "{goto}{fg_black}{bg_white}Input buffer.  Press [ESC] to exit...{reset}\r",
             fg_black = color::Fg(color::Black),
             bg_white = color::Bg(color::White),
             goto = cursor::Goto(1, 30),
             reset = style::Reset,
    );

    for c in chars {
        write!(w, "{}", c);
    }
    w.flush().unwrap();
}
