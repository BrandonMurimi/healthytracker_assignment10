// creational_patterns/prototype_pattern.rs

#[derive(Clone, Debug, PartialEq)]
pub struct WorkoutTemplate {
    pub name: String,
    pub duration: u32,
    pub focus_area: String,
    pub calories: f32,
}

pub struct WorkoutTemplateCache {
    base_template: WorkoutTemplate,
}

impl WorkoutTemplateCache {
    pub fn new(template: WorkoutTemplate) -> Self {
        Self { base_template: template }
    }

    pub fn clone_template(&self) -> WorkoutTemplate {
        self.base_template.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_workout_template() {
        let original = WorkoutTemplate {
            name: "Full Body Burn".into(),
            duration: 45,
            focus_area: "Total Body".into(),
            calories: 400.0,
        };

        let cache = WorkoutTemplateCache::new(original.clone());
        let clone = cache.clone_template();

        assert_eq!(original, clone);
        assert_ne!(&original as *const _, &clone as *const _); // Ensure different instances
    }
}

