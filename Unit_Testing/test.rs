// tests/tests_health_tracker_patterns.rs

#[cfg(test)]
mod tests {
    use crate::builder_pattern::*;
    use crate::factory_method::*;
    use crate::simple_factory::*;
    use crate::abstract_factory::*;
    use crate::prototype_pattern::*;
    use crate::singleton_pattern::*;

    #[test]
    fn test_simple_factory_activity() {
        let activity = ActivityFactory::create_activity(ActivityType::Walk);
        assert_eq!(activity.name, "Walking");
        assert_eq!(activity.met_value, 3.5);
    }

    #[test]
    fn test_factory_method_steps_goal() {
        let evaluator = create_goal_evaluator("steps");
        assert!(evaluator.evaluate(10000.0, 8000.0));
        assert_eq!(evaluator.goal_type(), "Steps");
    }

    #[test]
    fn test_abstract_factory_web_ui() {
        let factory = WebUIFactory;
        assert_eq!(factory.create_tracker_display(), "Web Tracker Display");
        assert_eq!(factory.create_goal_panel(), "Web Goal Panel");
    }

    #[test]
    fn test_builder_workout_plan() {
        let plan = WorkoutPlanBuilder::new()
            .warmup(10)
            .workout(40)
            .cooldown(5)
            .target_calories(350.0)
            .intensity("Moderate")
            .build();
        assert_eq!(plan.cooldown_minutes, 5);
        assert_eq!(plan.target_calories, 350.0);
    }

    #[test]
    fn test_prototype_template_clone() {
        let original = WorkoutTemplate {
            name: "Power Circuit".into(),
            duration: 30,
            focus_area: "Strength".into(),
            calories: 450.0,
        };
        let cache = WorkoutTemplateCache::new(original.clone());
        let clone = cache.clone_template();
        assert_eq!(original, clone);
    }

    #[test]
    fn test_singleton_app_settings_mutability() {
        let settings = get_settings_instance();
        {
            let mut config = settings.lock().unwrap();
            config.language = "fr".into();
        }
        let settings = get_settings_instance();
        let config = settings.lock().unwrap();
        assert_eq!(config.language, "fr");
    }
}

