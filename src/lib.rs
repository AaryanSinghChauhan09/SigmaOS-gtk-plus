#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetType { Window, Button, Label, BoxContainer }

pub struct GtkApplicationContext {
    pub app_id: String,
    pub is_running: bool,
}

impl GtkApplicationContext {
    pub fn new(app_id: &str) -> Self {
        Self { app_id: app_id.into(), is_running: true }
    }
    pub fn quit(&mut self) { self.is_running = false; }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gtk_application_context_aaa() {
        // Arrange
        let mut app = GtkApplicationContext::new("org.sigmaos.MyApp");
        assert!(app.is_running);
        // Act
        app.quit();
        // Assert
        assert!(!app.is_running);
    }
}
