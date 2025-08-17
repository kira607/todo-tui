use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    prelude::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::{
    core::task::{Task, Tasks},
    utils::center,
    tui::widgets::{
        component::Component,
        input_field::{InputField, InputFieldMsg},
        task_form::{TaskForm, TaskFormMsg},
        task_info::TaskInfo,
        tasks_list::{TasksList, TasksListMsg},
    },
};

#[derive(Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Edit,
    Sandbox,
}

pub enum AppMsg {
    Input(InputFieldMsg),
    TasksList(TasksListMsg),
    TaskForm(TaskFormMsg),
    Quit,
    Sandbox,
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
    input: InputField,
    task_form: TaskForm,
}

// widgets!(
//     (tasks_list, TasksList, TasksList(TasksListMsg)),
//     (task_info, TaskInfo),
//     (input, InputField, Input(InputFieldMsg)),
//     (task_form, TaskForm, TaskForm(TaskFormMsg)),
// );

// Main App methods
impl App {
    pub fn new(tasks: Tasks) -> Self {
        let items = tasks.all().to_vec();
        Self {
            tasks: tasks,
            running: true,
            mode: Mode::Normal,
            tasks_list: TasksList::new(items),
            task_info: TaskInfo::default(),
            input: InputField::default(),
            task_form: TaskForm::default(),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame, frame.area()))?;
            let event = event::read()?;
            let msg = self.handle_event(event);
            if let Some(msg) = msg {
                self.handle_message(msg);
            }
        }
        Ok(())
    }

    fn add_task(&mut self, title: &str) {
        if !title.is_empty() {
            let task = self.tasks.add_task(title);
            self.tasks_list.add_task(task);
            self.update_task_info();
        }
        self.mode = Mode::Normal;
    }

    fn update_task_title(&mut self, title: &str) {
        if !title.is_empty() {
            let task = self.tasks_list.selected_item();
            if let Some(task) = task {
                self.tasks.update_title(&task.id, title);
                self.tasks_list.update_selected(title);
                self.update_task_info();
            }
        }
        self.mode = Mode::Normal;
    }

    /// Update a [`TaskInfo`] widget based on
    /// currently selected item in [`TasksList`] widget.
    fn update_task_info(&mut self) {
        if self.tasks_list.is_empty() {
            self.task_info.clear();
        }
        if let Some(task) = self.tasks_list.selected_item() {
            self.task_info.set_task(task);
        }
    }
}

// Component trait implementation
impl Component for App {
    type Msg = AppMsg;

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let [header_area, main_area, task_info_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(8),
            Constraint::Fill(2),
            Constraint::Length(1),
        ])
        .areas(area);
        let input_area = center(area, Constraint::Percentage(50), Constraint::Length(5));
        let sandbox_area = center(area, Constraint::Percentage(70), Constraint::Percentage(70));

        Paragraph::new("Tasks app")
            .bold()
            .centered()
            .render(header_area, frame.buffer_mut());
        self.tasks_list.draw(frame, main_area);
        self.task_info.draw(frame, task_info_area);
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(footer_area, frame.buffer_mut());

        match self.mode {
            Mode::Normal => {}
            Mode::Insert | Mode::Edit => self.input.draw(frame, input_area),
            Mode::Sandbox => self.task_form.draw(frame, sandbox_area),
        }
    }

    fn handle_event(&mut self, event: Event) -> Option<Self::Msg> {
        let Event::Key(key) = event else {
            return None;
        };
        if key.kind != KeyEventKind::Press {
            return None;
        }
        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit_app(),
                KeyCode::Char('x') => self.toggle_sandbox(),
                _ => self.tasks_list.handle_event(event).map(AppMsg::TasksList),
            },
            Mode::Insert | Mode::Edit => self.input.handle_event(event).map(AppMsg::Input),
            Mode::Sandbox => self.task_form.handle_event(event).map(AppMsg::TaskForm),
        }
    }

    fn handle_message(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::Input(input_field_msg) => match input_field_msg {
                InputFieldMsg::Submited(text) => self.on_input_submited(text),
                InputFieldMsg::Canceled => self.on_input_canceled(),
            },
            AppMsg::TasksList(tasks_list_msg) => match tasks_list_msg {
                TasksListMsg::AddTask => self.on_add_task(),
                TasksListMsg::EditTask => self.on_edit_task_title(),
                TasksListMsg::ToggleStatus => self.on_toggle_status(),
                TasksListMsg::DeleteTask => self.on_delete_task(),
                TasksListMsg::SelectedNone => self.on_selected_none(),
                TasksListMsg::SelectedNext => self.on_selected_next(),
                TasksListMsg::SelectedPrevious => self.on_selected_previous(),
                TasksListMsg::SelectedFirst => self.on_selected_first(),
                TasksListMsg::SelectedLast => self.on_selected_last(),
            },
            AppMsg::TaskForm(msg) => match msg {
                TaskFormMsg::Submited(task) => self.on_task_form_submited(task),
                TaskFormMsg::Canceled => self.on_task_form_canceled(),
            },
            AppMsg::Quit => self.on_quit(),
            AppMsg::Sandbox => self.on_sandbox(),
        }
    }
}

// Eent handlers
impl App {
    fn quit_app(&mut self) -> Option<AppMsg> {
        Some(AppMsg::Quit)
    }

    fn toggle_sandbox(&mut self) -> Option<AppMsg> {
        Some(AppMsg::Sandbox)
    }
}

// Handlers of child widgets messages.
impl App {
    fn on_input_submited(&mut self, input: String) {
        match self.mode {
            Mode::Normal => panic!("Unexpected message"),
            Mode::Insert => self.add_task(&input),
            Mode::Edit => self.update_task_title(&input),
            Mode::Sandbox => panic!("Unexpected message"),
        }
    }

    fn on_input_canceled(&mut self) {
        self.mode = Mode::Normal;
    }

    fn on_add_task(&mut self) {
        self.mode = Mode::Insert;
        self.input.clear().set_title("Add a new task");
    }

    fn on_edit_task_title(&mut self) {
        let Some(task) = self.tasks_list.selected_item() else {
            return;
        };
        self.mode = Mode::Edit;
        self.input.clear().set_title("Edit a task title");
        self.input.set_input(&task.title);
    }

    fn on_toggle_status(&mut self) {
        let toggled = self.tasks_list.toggle_status();
        if let Some(task) = toggled {
            self.tasks.toggle(&task.id);
            self.update_task_info();
        }
    }

    fn on_delete_task(&mut self) {
        if let Some(task) = self.tasks_list.remove_selected() {
            self.tasks.delete(&task.id);
        }
        self.update_task_info();
    }

    fn on_selected_none(&mut self) {
        self.tasks_list.select_none();
        self.task_info.clear();
    }

    fn on_selected_next(&mut self) {
        self.tasks_list.select_next();
        self.update_task_info(); // needs another update
    }

    fn on_selected_previous(&mut self) {
        self.tasks_list.select_previous();
        self.update_task_info();
    }

    fn on_selected_first(&mut self) {
        self.tasks_list.select_first();
        self.update_task_info();
    }

    fn on_selected_last(&mut self) {
        self.tasks_list.select_last();
        self.update_task_info(); // needs another update
    }

    fn on_task_form_submited(&mut self, _task: Task) {
        self.mode = Mode::Normal;
    }

    fn on_task_form_canceled(&mut self) {
        self.mode = Mode::Normal;
    }

    fn on_quit(&mut self) {
        self.running = false;
        self.tasks.save().unwrap();
    }

    fn on_sandbox(&mut self) {
        self.mode = Mode::Sandbox;
    }
}
