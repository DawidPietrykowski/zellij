use criterion::{criterion_group, criterion_main, Criterion};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use zellij_server::panes::sixel::SixelImageStore;
use zellij_server::panes::LinkHandler;
use zellij_server::panes::TerminalPane;
use zellij_server::tab::Pane;
use zellij_utils::data::{Palette, Style};
use zellij_utils::pane_size::PaneGeom;

fn generate_big_file() -> Vec<u8> {
    let mut content = Vec::new();

    content.extend_from_slice("abcdef\n".as_bytes());

    for i in 0..50_000 {
        content
            .extend_from_slice(format!("Line {:06}: Some repeating content here\n", i).as_bytes());
    }

    content.extend_from_slice("fedcba\n".as_bytes());

    content
}

fn create_pane() -> TerminalPane {
    let mut fake_win_size = PaneGeom::default();
    fake_win_size.cols.set_inner(121);
    fake_win_size.rows.set_inner(20);

    let pid = 1;
    let style = Style::default();
    let sixel_image_store = Rc::new(RefCell::new(SixelImageStore::default()));
    let terminal_emulator_color_codes = Rc::new(RefCell::new(HashMap::new()));
    let debug = false;
    let arrow_fonts = true;
    let styled_underlines = true;
    let explicitly_disable_kitty_keyboard_protocol = false;
    TerminalPane::new(
        pid,
        fake_win_size,
        style,
        0,
        String::new(),
        Rc::new(RefCell::new(LinkHandler::new())),
        Rc::new(RefCell::new(None)),
        sixel_image_store,
        Rc::new(RefCell::new(Palette::default())),
        terminal_emulator_color_codes,
        None,
        None,
        debug,
        arrow_fonts,
        styled_underlines,
        explicitly_disable_kitty_keyboard_protocol,
    )
}

pub fn searching_scroll_viewport(terminal_pane: &mut TerminalPane) {
    terminal_pane.update_search_term("abcdef");
    terminal_pane.clear_search();
    terminal_pane.update_search_term("fedcba");
    terminal_pane.clear_search();
}

pub fn search_nonexistent(terminal_pane: &mut TerminalPane) {
    terminal_pane.update_search_term("aaaaaa");
    terminal_pane.clear_search();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = generate_big_file();
    let mut terminal_pane = create_pane();
    terminal_pane.handle_pty_bytes(content.clone());
    c.bench_function("up_down", |b| {
        b.iter(|| searching_scroll_viewport(&mut terminal_pane))
    });
    c.bench_function("nonexistent", |b| {
        b.iter(|| search_nonexistent(&mut terminal_pane))
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
