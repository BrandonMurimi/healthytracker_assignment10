// creational_patterns/singleton_pattern.rs

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub struct AppSettings {
    pub dark_mode: bool,
    pub notifications_enabled: bool,
    pub language: String,
}

static SETTINGS: Lazy<Arc<Mutex<AppSettings>>> = Lazy::new(|| {
    Arc::new(Mutex::new(AppSettings {
        dark_mode: false,
        notifications_enabled: true,
        language: "en".to_string(),
    }))
});

pub fn get_settings_instance() -> Arc<Mutex<AppSettings>> {
    SETTINGS.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_access_and_modify() {
        let instance = get_settings_instance();
        {
            let mut settings = instance.lock().unwrap();
            settings.dark_mode = true;
        }

        let check_instance = get_settings_instance();
        let settings = check_instance.lock().unwrap();
        assert!(settings.dark_mode);
        assert_eq!(settings.language, "en");
    }

    #[test]
    fn test_singleton_identity() {
        let s1 = get_settings_instance();
        let s2 = get_settings_instance();
        assert!(Arc::ptr_eq(&s1, &s2));
    }
}

