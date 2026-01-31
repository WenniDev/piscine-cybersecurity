use std::{
    io,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
};

use crate::totp;

const TOTP_PERIOD: u64 = 30;

pub fn run_tui(key: &[u8], qr_string: &str) -> io::Result<()> {
    let mut app = App::new(key, qr_string);
    ratatui::run(|terminal| app.run(terminal))
}

#[derive(Debug, Default)]
struct App {
    key: Vec<u8>,
    qr_string: String,
    time_remaining: u64,
    progress: u16,
    otp_code: u32,
    exit: bool,
}

impl App {
    fn new(key: &[u8], qr_string: &str) -> App {
        App {
            key: key.into(),
            qr_string: qr_string.into(),
            time_remaining: 0,
            progress: 0,
            otp_code: 0,
            exit: false,
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            self.time_remaining = TOTP_PERIOD - (now % TOTP_PERIOD);
            self.progress =
                ((TOTP_PERIOD - self.time_remaining) as f64 / TOTP_PERIOD as f64 * 100.0) as u16;

            self.otp_code = totp::totp(&self.key);

            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key_event(key)
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Min(10),   // QR Code
            Constraint::Length(3), // OTP code
            Constraint::Length(3), // Progress bar
            Constraint::Length(2), // Instructions
        ])
        .split(area);

        let qr_code = Block::default()
            .borders(Borders::ALL)
            .title(" ft_otp ".bold())
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan));

        let _qr_paragraph = Paragraph::new(self.qr_string.as_str())
            .block(qr_code)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White))
            .render(chunks[0], buf);

        let code_color = if self.time_remaining <= 5 {
            Color::Red
        } else if self.time_remaining <= 10 {
            Color::Yellow
        } else {
            Color::Green
        };

        let _otp_paragraph = Paragraph::new(format!("{:06}", self.otp_code))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Current Code ".bold())
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(code_color).bold())
            .render(chunks[1], buf);

        let _gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Next code in {}s ", self.time_remaining))
                    .title_alignment(Alignment::Center),
            )
            .gauge_style(Style::default().fg(code_color).bg(Color::DarkGray))
            .percent(self.progress)
            .render(chunks[2], buf);

        let _instruction = Paragraph::new("Press 'q' or 'Esc' to quit")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .render(chunks[3], buf);
    }
}
