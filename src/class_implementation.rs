// Assignment 10: Health Fitness Tracker – Class Implementations

// --- UserProfile ---
pub struct UserProfile {
    pub user_id: String,
    pub name: String,
    pub age: u32,
    pub weight: f32,
    pub height: f32,
}

impl UserProfile {
    pub fn update_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }

    pub fn bmi(&self) -> f32 {
        self.weight / ((self.height / 100.0).powf(2.0))
    }
}

// --- WorkoutSession ---
pub struct WorkoutSession {
    pub session_id: String,
    pub user_id: String,
    pub activity_type: String,
    pub duration_minutes: u32,
    pub calories_burned: f32,
}

impl WorkoutSession {
    pub fn calculate_intensity(&self) -> &str {
        if self.calories_burned / self.duration_minutes as f32 > 8.0 {
            "High"
        } else {
            "Moderate"
        }
    }
}

// --- FitnessGoal ---
pub struct FitnessGoal {
    pub goal_id: String,
    pub user_id: String,
    pub goal_type: String, // e.g., "Calories", "Steps"
    pub target_value: f32,
    pub current_value: f32,
}

impl FitnessGoal {
    pub fn is_achieved(&self) -> bool {
        self.current_value >= self.target_value
    }

    pub fn progress(&self) -> f32 {
        (self.current_value / self.target_value) * 100.0
    }
}

// --- NutritionLog ---
pub struct NutritionLog {
    pub log_id: String,
    pub user_id: String,
    pub meal_type: String, // e.g., Breakfast, Lunch
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fats: f32,
}

impl NutritionLog {
    pub fn total_macros(&self) -> f32 {
        self.protein + self.carbs + self.fats
    }
}

// --- Activity ---
pub struct Activity {
    pub activity_id: String,
    pub name: String,
    pub met_value: f32, // Metabolic Equivalent of Task
}

impl Activity {
    pub fn estimate_calories(&self, weight: f32, minutes: u32) -> f32 {
        (self.met_value * 3.5 * weight / 200.0) * minutes as f32
    }
}

// --- Device ---
pub struct Device {
    pub device_id: String,
    pub user_id: String,
    pub device_type: String, // e.g., Smartwatch
    pub sync_enabled: bool,
}

impl Device {
    pub fn toggle_sync(&mut self) {
        self.sync_enabled = !self.sync_enabled;
    }
}

// --- HealthStats ---
pub struct HealthStats {
    pub user_id: String,
    pub heart_rate: u32,
    pub steps: u32,
    pub sleep_hours: f32,
}

impl HealthStats {
    pub fn sleep_quality(&self) -> &str {
        if self.sleep_hours >= 7.0 {
            "Good"
        } else {
            "Poor"
        }
    }
}

// --- SyncManager ---
pub struct SyncManager;

impl SyncManager {
    pub fn sync_device(device: &Device) {
        if device.sync_enabled {
            println!("Device {} synced successfully.", device.device_id);
        } else {
            println!("Device {} is not enabled for sync.", device.device_id);
        }
    }
}

