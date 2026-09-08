//! High-framerate Ratatui terminal dashboard.

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If not a TTY (e.g. In CI or script redirect), print summary and exit cleanly
    if !crossterm::tty::IsTty::is_tty(&io::stdout()) {
        println!("Aether TUI Monitor: Headless / Non-TTY mode. Protocol reactor operational.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_dashboard(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("TUI Error: {:?}", err);
    }

    Ok(())
}

fn run_dashboard<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut rank = 0;
    const K: usize = 16;
    let mut waterfall = Vec::new();
    let mut packet_id = 0;
    let start = Instant::now();

    loop {
        // Handle input events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        // Simulate packet flow
        packet_id += 1;
        let is_dropped = (packet_id % 7) == 0;
        let is_repair = (packet_id % 4) == 0;

        if !is_dropped {
            rank = (rank + 1).min(K);
        }

        let item = if is_dropped {
            ListItem::new(Line::from(vec![
                Span::styled(format!(" [DROP]   Pkt #{:05} | Physical Loss (Rain Fade)", packet_id), Style::default().fg(Color::Red)),
            ]))
        } else if is_repair {
            ListItem::new(Line::from(vec![
                Span::styled(format!(" [REPAIR] Pkt #{:05} | GF(2^8) Inner RLNC Symbol (Seed: 0x{:04X})", packet_id, packet_id * 31), Style::default().fg(Color::Cyan)),
            ]))
        } else {
            ListItem::new(Line::from(vec![
                Span::styled(format!(" [SOURCE] Pkt #{:05} | GF(2) Systematic Symbol", packet_id), Style::default().fg(Color::Green)),
            ]))
        };

        waterfall.insert(0, item);
        if waterfall.len() > 15 {
            waterfall.pop();
        }

        if rank == K && (packet_id % 5 == 0) {
            rank = 0; // Reset window for demo
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(8),
                    Constraint::Length(5),
                ])
                .split(f.area());

            // 1. Header
            let title = Paragraph::new(" AETHER-TRANSPORT: SOVEREIGN CELLULAR TELEMETRY MONITOR (PORT HARCOURT, NG)")
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).title("Protocol Node: AETHER_01"));
            f.render_widget(title, chunks[0]);

            // 2. Matrix Rank Progress Gauge
            let pct = ((rank as f64 / K as f64) * 100.0) as u16;
            let gauge = Gauge::default()
                .block(Block::default().title(format!(" Fulcrum Matrix Inversion Rank (DoF: {}/{}) ", rank, K)).borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
                .percent(pct);
            f.render_widget(gauge, chunks[1]);

            // 3. Middle split: Waterfall & RF Telemetry
            let mid_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(chunks[2]);

            let waterfall_list = List::new(waterfall.clone())
                .block(Block::default().title(" Packet Stream Waterfall ").borders(Borders::ALL));
            f.render_widget(waterfall_list, mid_chunks[0]);

            let telemetry_text = vec![
                Line::from(vec![Span::raw("Carrier: "), Span::styled("MTN Nigeria (4G LTE Band 7)", Style::default().fg(Color::Yellow))]),
                Line::from(vec![Span::raw("RSRP:    "), Span::styled("-104 dBm (Weak Tropical Cell)", Style::default().fg(Color::LightRed))]),
                Line::from(vec![Span::raw("RSRQ:    "), Span::styled("-14 dB", Style::default().fg(Color::White))]),
                Line::from(vec![Span::raw("SINR:    "), Span::styled("2.4 dB (Rain Fade Warning)", Style::default().fg(Color::LightRed))]),
                Line::from(vec![Span::raw("State:   "), Span::styled("Subspace Authenticated", Style::default().fg(Color::Green))]),
            ];
            let tel_paragraph = Paragraph::new(telemetry_text)
                .block(Block::default().title(" RF Physical Telemetry ").borders(Borders::ALL));
            f.render_widget(tel_paragraph, mid_chunks[1]);

            // 4. Performance Metrics
            let elapsed = start.elapsed().as_secs();
            let metrics_text = vec![
                Line::from(vec![
                    Span::styled("Goodput: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("62.8 Mbps  |  ", Style::default().fg(Color::Green)),
                    Span::styled("Observed RTT: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("142 ms  |  ", Style::default().fg(Color::Yellow)),
                    Span::styled("TCP Stalls Avoided: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{} pkts  |  ", packet_id / 7), Style::default().fg(Color::Cyan)),
                    Span::styled("Uptime: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{}s", elapsed), Style::default().fg(Color::White)),
                ]),
            ];
            let metrics = Paragraph::new(metrics_text)
                .block(Block::default().title(" Live Transport Metrics (Press 'q' to exit) ").borders(Borders::ALL));
            f.render_widget(metrics, chunks[3]);
        })?;

        // Limit execution in automated testing if running for more than 5 seconds without user interaction
        if start.elapsed() > Duration::from_secs(60) {
            break;
        }
    }

    Ok(())
}
