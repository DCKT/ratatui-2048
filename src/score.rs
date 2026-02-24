use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

#[derive(Default, Clone)]
pub struct Score {
    pub value: i32,
}

impl Widget for &Score {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(format!("{}", self.value))
            .block(
                Block::default()
                    .title("Score")
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded),
            )
            .render(area, buf);
    }
}
