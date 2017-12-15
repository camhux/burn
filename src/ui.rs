use std::io::Write;

use termion::{clear, cursor};

type Field = Vec<Vec<u8>>;

const LINE_END: &[u8] = b"\r\n";

// TODO: move terminal size info into here?
pub struct Ui<W: Write> {
  writer: W,
}

impl<W: Write> Ui<W> {
    pub fn create(writer: W) -> Self {
        let mut ui = Ui {
            writer: writer,
        };

        ui.setup_window();

        return ui;
    }

    fn setup_window(&mut self) {
        write!(self.writer, "{}{}{}", clear::All, cursor::Hide, cursor::Goto(1,1)).unwrap();
    }

    fn reset_window(&mut self) {
        write!(self.writer, "{}", cursor::Show).unwrap();
    }

    pub fn draw(&mut self, field: &Field) {
        write!(self.writer, "{}", cursor::Goto(1, 1)).unwrap();

        // TODO: composite the features onto the field
        for line in field {
            self.writer.write(&line).expect("failed to write line");
            self.writer.write(LINE_END).unwrap();
        }

        self.writer.flush().unwrap();
    }
}

impl<W: Write> Drop for Ui<W> {
    fn drop(&mut self) {
        self.reset_window();
    }
}
