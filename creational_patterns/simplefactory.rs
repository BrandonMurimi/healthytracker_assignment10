// creational_patterns/simple_factory.rs

pub enum ActivityType {
    Walk,
    Run,
    Cycle,
}

pub struct Activity {
    pub activity_id: String,
    pub name: String,
    pub met_value: f32,
}

impl Activity {
    pub fn estimate_calories(&self, weight: f32, duration_minutes: u32) -> f32 {
        (self.met_value * 3.5 * weight / 200.0) * duration_minutes as f32
    }
}

pub struct ActivityFactory;

impl ActivityFactory {
    pub fn create_activity(activity_type: ActivityType) -> Activity {
        match activity_type {
            ActivityType::Walk => Activity {
                activity_id: "ACT001".into(),
                name: "Walking".into(),
                met_value: 3.5,
            },
            ActivityType::Run => Activity {
                activity_id: "ACT002".into(),
                name: "Running".into(),
                met_value: 7.5,
            },
            ActivityType::Cycle => Activity {
                activity_id: "ACT003".into(),
                name: "Cycling".into(),
                met_value: 6.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_walk_activity() {
        let activity = ActivityFactory::create_activity(ActivityType::Walk);
        assert_eq!(activity.name, "Walking");
        assert_eq!(activity.met_value, 3.5);
    }

    #[test]
    fn test_calorie_estimation() {
        let activity = ActivityFactory::create_activity(ActivityType::Run);
        let calories = activity.estimate_calories(70.0, 30);
        assert!(calories > 200.0);
    }
}

