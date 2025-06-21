use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};
use tui::widgets::{Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use sysinfo::{System, SystemExt, CpuExt};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut sys = System::new_all();

    loop {
        sys.refresh_cpu();
        let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect::<Vec<f32>>();

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let text = cpu_usage.iter()
                .enumerate()
                .map(|(i, usage)| format!("CPU {}: {:.2}%", i, usage))
                .collect::<Vec<String>>()
                .join("\n");

            let paragraph = Paragraph::new(text)
                .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                .style(Style::default().fg(Color::Green));
            f.render_widget(paragraph, chunks[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

   
    

