use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    prelude::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::{
    core::Tasks,
    widgets::{InputPopup, TaskInfo, TasksList},
};

pub enum Mode {
    Normal,
    Insert,
    Edit,
}

pub struct App {
    // storage
    tasks: Tasks,
    // state
    running: bool,
    mode: Mode,
    // widgets
    tasks_list: TasksList,
    task_info: TaskInfo,
    popup: InputPopup,
}

impl App {
    pub fn new(tasks: Tasks) -> Self {
        let items = tasks.all().to_vec();
        Self {
            tasks: tasks,
            running: true,
            mode: Mode::Normal,
            tasks_list: TasksList::new(items),
            task_info: TaskInfo::default(),
            popup: InputPopup::default(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('a') => {
                    self.mode = Mode::Insert;
                    self.popup.clear().set_title("Add a new task");
                }
                KeyCode::Char('d') => {
                    if let Some(task) = self.tasks_list.remove_selected() {
                        self.tasks.delete(&task.id);
                    }
                }
                KeyCode::Char('e') => {
                    self.mode = Mode::Edit;
                    self.popup.clear().set_title("Edit a task title");
                    self.popup
                        .set_input(&self.tasks_list.selected_item().unwrap().title);
                }
                KeyCode::Char('q') | KeyCode::Esc => self.prepare_shutdown(),
                KeyCode::Char('h') | KeyCode::Left => self.select_none(),
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.select_last(),
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter | KeyCode::Char(' ') => {
                    self.toggle_status();
                }
                _ => {}
            },
            Mode::Insert => match key.code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Enter => {
                    let input = self.popup.get_input().trim();
                    if !input.is_empty() {
                        let task = self.tasks.add_task(input);
                        self.tasks_list.add_task(task);
                    }
                    self.mode = Mode::Normal;
                }
                KeyCode::Char(c) => self.popup.push(c),
                KeyCode::Backspace => {
                    self.popup.pop();
                }
                _ => {}
            },
            Mode::Edit => match key.code {
                KeyCode::Esc => self.mode = Mode::Normal,
                KeyCode::Enter => {
                    let input = self.popup.get_input().trim();
                    if !input.is_empty() {
                        let task = self.tasks_list.selected_item();
                        if let Some(task) = task {
                            self.tasks.update_title(&task.id, input);
                            self.tasks_list.update_selected(input)
                        }
                    }
                    self.mode = Mode::Normal;
                }
                KeyCode::Char(c) => self.popup.push(c),
                KeyCode::Backspace => {
                    self.popup.pop();
                }
                _ => {}
            },
        }
    }

    fn select_none(&mut self) {
        self.tasks_list.select_none();
        self.task_info.clear();
    }

    fn select_next(&mut self) {
        self.tasks_list.select_next();
        self.update_task_info(); // needs another update
    }

    fn select_previous(&mut self) {
        self.tasks_list.select_previous();
        self.update_task_info();
    }

    fn select_first(&mut self) {
        self.tasks_list.select_first();
        self.update_task_info();
    }

    fn select_last(&mut self) {
        self.tasks_list.select_last();
        self.update_task_info(); // needs another update
    }

    fn toggle_status(&mut self) {
        let toggled = self.tasks_list.toggle_status();
        if let Some(task) = toggled {
            self.tasks.toggle(&task.id);
            self.update_task_info();
        }
    }

    fn prepare_shutdown(&mut self) {
        self.running = false;
        self.tasks.save().unwrap();
    }

    /// Update a [`TaskInfo`] widget based on
    /// currently selected item in [`TasksList`] widget.
    fn update_task_info(&mut self) {
        if let Some(task) = self.tasks_list.selected_item() {
            self.task_info.set_task(task);
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, task_info_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(8),
            Constraint::Fill(2),
            Constraint::Length(1),
        ])
        .areas(area);

        App::render_header(header_area, buf);
        self.tasks_list.render(main_area, buf);
        self.task_info.render(task_info_area, buf);
        App::render_footer(footer_area, buf);

        match self.mode {
            Mode::Normal => {}
            Mode::Insert => self.popup.render(area, buf),
            Mode::Edit => self.popup.render(area, buf),
        }
        // self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("My app")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }
}
