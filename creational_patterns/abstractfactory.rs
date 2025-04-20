// creational_patterns/abstract_factory.rs

pub trait UIFactory {
    fn create_tracker_display(&self) -> String;
    fn create_goal_panel(&self) -> String;
}

pub struct MobileUIFactory;
impl UIFactory for MobileUIFactory {
    fn create_tracker_display(&self) -> String {
        "Mobile Tracker Display".into()
    }

    fn create_goal_panel(&self) -> String {
        "Mobile Goal Panel".into()
    }
}

pub struct WebUIFactory;
impl UIFactory for WebUIFactory {
    fn create_tracker_display(&self) -> String {
        "Web Tracker Display".into()
    }

    fn create_goal_panel(&self) -> String {
        "Web Goal Panel".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_ui_factory() {
        let factory = MobileUIFactory;
        assert_eq!(factory.create_tracker_display(), "Mobile Tracker Display");
        assert_eq!(factory.create_goal_panel(), "Mobile Goal Panel");
    }

    #[test]
    fn test_web_ui_factory() {
        let factory = WebUIFactory;
        assert_eq!(factory.create_tracker_display(), "Web Tracker Display");
        assert_eq!(factory.create_goal_panel(), "Web Goal Panel");
    }
}

