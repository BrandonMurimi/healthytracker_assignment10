// creational_patterns/builder_pattern.rs

#[derive(Debug, PartialEq)]
pub struct WorkoutPlan {
    pub warmup_minutes: u32,
    pub workout_minutes: u32,
    pub cooldown_minutes: u32,
    pub target_calories: f32,
    pub intensity_level: String,
}

pub struct WorkoutPlanBuilder {
    warmup_minutes: u32,
    workout_minutes: u32,
    cooldown_minutes: u32,
    target_calories: f32,
    intensity_level: String,
}

impl WorkoutPlanBuilder {
    pub fn new() -> Self {
        Self {
            warmup_minutes: 0,
            workout_minutes: 0,
            cooldown_minutes: 0,
            target_calories: 0.0,
            intensity_level: "Moderate".into(),
        }
    }

    pub fn warmup(mut self, minutes: u32) -> Self {
        self.warmup_minutes = minutes;
        self
    }

    pub fn workout(mut self, minutes: u32) -> Self {
        self.workout_minutes = minutes;
        self
    }

    pub fn cooldown(mut self, minutes: u32) -> Self {
        self.cooldown_minutes = minutes;
        self
    }

    pub fn target_calories(mut self, calories: f32) -> Self {
        self.target_calories = calories;
        self
    }

    pub fn intensity(mut self, level: &str) -> Self {
        self.intensity_level = level.into();
        self
    }

    pub fn build(self) -> WorkoutPlan {
        WorkoutPlan {
            warmup_minutes: self.warmup_minutes,
            workout_minutes: self.workout_minutes,
            cooldown_minutes: self.cooldown_minutes,
            target_calories: self.target_calories,
            intensity_level: self.intensity_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workout_plan_builder() {
        let plan = WorkoutPlanBuilder::new()
            .warmup(10)
            .workout(30)
            .cooldown(5)
            .target_calories(300.0)
            .intensity("High")
            .build();

        assert_eq!(plan.warmup_minutes, 10);
        assert_eq!(plan.workout_minutes, 30);
        assert_eq!(plan.cooldown_minutes, 5);
        assert_eq!(plan.target_calories, 300.0);
        assert_eq!(plan.intensity_level, "High");
    }
}

