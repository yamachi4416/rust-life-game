use portable_pty::{Child, CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

pub struct Command {
    child: Box<dyn Child + Send + Sync>,
    writer: Box<dyn Write + Send>,
    parser: Arc<Mutex<vt100::Parser>>,
}

impl Command {
    pub fn start() -> Self {
        let cmd = CommandBuilder::new(env!("CARGO_BIN_EXE_rust-life-game"));

        let pty = NativePtySystem::default()
            .openpty(PtySize {
                rows: 25,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();

        let app = Self {
            child: pty.slave.spawn_command(cmd).unwrap(),
            writer: pty.master.take_writer().unwrap(),
            parser: Arc::new(Mutex::new(vt100::Parser::new(25, 80, 0))),
        };

        let processor = Arc::clone(&app.parser);

        thread::spawn(move || {
            let mut buf = [0u8; 4096];
            let mut reader = pty.master.try_clone_reader().unwrap();
            while let Ok(n @ 1..) = reader.read(&mut buf) {
                processor.lock().unwrap().process(&buf[..n]);
            }
        });

        app.wait_for("OCTAGON");
        app
    }

    pub fn frame(&self) -> String {
        let parser = self.parser.lock().unwrap();
        let screen = parser.screen();
        let (rows, cols) = screen.size();

        let mut out = String::new();
        for r in 0..rows {
            for c in 0..cols {
                let cell = screen.cell(r, c).unwrap();
                out.push_str(&match cell.contents().as_str() {
                    "" | " " => match cell.bgcolor() {
                        vt100::Color::Idx(n) if n != 15 => format!("{n:x}"),
                        _ => " ".into(),
                    },
                    text => text.into(),
                });
            }
            out.push('\n');
        }
        out
    }

    pub fn press(&mut self, c: char) -> String {
        let mut buf = [0u8; 4];
        self.writer
            .write_all(c.encode_utf8(&mut buf).as_bytes())
            .unwrap();
        self.writer.flush().unwrap();
        sleep(Duration::from_millis(50));
        self.frame()
    }

    pub fn quit(mut self) -> bool {
        self.press('q');
        self.child.wait().unwrap().success()
    }

    fn wait_for(&self, text: &str) {
        while !self.frame().contains(text) {
            sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        let _ = self.child.kill();
        self.child.wait().unwrap();
    }
}
