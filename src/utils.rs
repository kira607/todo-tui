use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
};
use uuid::Uuid;

/// Генерирует уникальный ID с необязательным префиксом.
/// Пример: "action-550e8400e29b41d4a716446655440000"
pub fn generate_id(prefix: Option<&str>) -> String {
    let id = Uuid::new_v4().to_string().replace("-", "");
    match prefix {
        Some(p) => format!("{}-{}", p, id),
        None => id,
    }
}

/// Centers a [`Rect`] within another [`Rect`] using the provided [`Constraint`]s.
///
/// # Examples
///
/// ```rust
/// use ratatui::layout::{Constraint, Rect};
///
/// let area = Rect::new(0, 0, 100, 100);
/// let horizontal = Constraint::Percentage(20);
/// let vertical = Constraint::Percentage(30);
///
/// let centered = center(area, horizontal, vertical);
/// ```
pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
