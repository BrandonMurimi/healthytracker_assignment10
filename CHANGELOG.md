# 📓 CHANGELOG.md – Health Fitness Tracker (Assignment 10)

## 🔧 Class Implementations
- Created foundational classes: `UserProfile`, `WorkoutSession`, `FitnessGoal`, `Activity`, `Device`, `NutritionLog`, `HealthStats`, `SyncManager`
- Implemented core logic methods like BMI calculation, calorie estimation, goal progress

🔗 Related: `fitness_tracker_classes.rs`

---

## 🏗️ Creational Patterns

### 1. 🏭 Simple Factory
- **Added:** `ActivityFactory` with enum support for Walk, Run, Cycle
- **Tests:** Verified MET value correctness, calorie estimation

🔗 `simple_factory.rs`

### 2. 🏭 Factory Method
- **Added:** `GoalEvaluator` trait with `CalorieGoal`, `StepsGoal` implementations
- **Tests:** Goal logic and type discrimination

🔗 `factory_method.rs`

### 3. 🏭 Abstract Factory
- **Added:** `UIFactory` for Web and Mobile UI panels
- **Tests:** Asserts display panel content

🔗 `abstract_factory.rs`

### 4. 🔨 Builder
- **Added:** `WorkoutPlanBuilder` for structured workout creation
- **Tests:** Validates chaining methods and object output

🔗 `builder_pattern.rs`

### 5. 🧬 Prototype
- **Added:** `WorkoutTemplateCache` for cloning base workouts
- **Tests:** Ensures deep clone, template equality

🔗 `prototype_pattern.rs`

### 6. 🔐 Singleton
- **Added:** Global `AppSettings` instance with shared Arc<Mutex<>>
- **Tests:** Verifies shared instance identity and modifiability

🔗 `singleton_pattern.rs`

---

## 🧪 Testing
- Consolidated all unit tests into `tests_health_tracker_patterns.rs`
- Each pattern validated with success criteria and coverage

📅 Last Updated: April 2025

