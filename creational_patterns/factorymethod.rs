// creational_patterns/factory_method.rs

pub trait GoalEvaluator {
    fn evaluate(&self, current: f32, target: f32) -> bool;
    fn goal_type(&self) -> &str;
}

pub struct CalorieGoal;
impl GoalEvaluator for CalorieGoal {
    fn evaluate(&self, current: f32, target: f32) -> bool {
        current >= target
    }
    fn goal_type(&self) -> &str {
        "Calories"
    }
}

pub struct StepsGoal;
impl GoalEvaluator for StepsGoal {
    fn evaluate(&self, current: f32, target: f32) -> bool {
        current >= target
    }
    fn goal_type(&self) -> &str {
        "Steps"
    }
}

pub fn create_goal_evaluator(goal_type: &str) -> Box<dyn GoalEvaluator> {
    match goal_type.to_lowercase().as_str() {
        "calories" => Box::new(CalorieGoal),
        "steps" => Box::new(StepsGoal),
        _ => panic!("Unsupported goal type: {}", goal_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calorie_goal_evaluation() {
        let evaluator = create_goal_evaluator("calories");
        assert!(evaluator.evaluate(500.0, 400.0));
        assert_eq!(evaluator.goal_type(), "Calories");
    }

    #[test]
    fn test_steps_goal_evaluation() {
        let evaluator = create_goal_evaluator("steps");
        assert!(evaluator.evaluate(10000.0, 10000.0));
        assert_eq!(evaluator.goal_type(), "Steps");
    }

    #[test]
    #[should_panic(expected = "Unsupported goal type")]
    fn test_invalid_goal_type() {
        create_goal_evaluator("distance");
    }
}

