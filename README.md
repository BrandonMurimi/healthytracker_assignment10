# healthytracker_assignment10

# 🧠 Health Fitness Tracker – Assignment 10

This project implements a Health Fitness Tracker in Rust, using object-oriented design and six creational design patterns. All core classes are derived from a class diagram and include complete unit testing and pattern-specific behavior.

---

## 📚 Contents

### ✅ Class Implementations
- `UserProfile`, `WorkoutSession`, `FitnessGoal`, `NutritionLog`, `Activity`, `Device`, `HealthStats`, `SyncManager`
- Defined in: `src/fitness_tracker_classes.rs`

### 🔧 Creational Design Patterns
Located in `/creational_patterns/`

| Pattern           | File                          | Purpose                                                      |
|------------------|-------------------------------|--------------------------------------------------------------|
| Simple Factory    | `simple_factory.rs`           | Creates Walk, Run, Cycle activities                          |
| Factory Method    | `factory_method.rs`           | Dynamically evaluates Steps or Calorie goals                |
| Abstract Factory  | `abstract_factory.rs`         | UI factory for Web and Mobile interfaces                    |
| Builder           | `builder_pattern.rs`          | Builds `WorkoutPlan` with warmup, intensity, cooldown       |
| Prototype         | `prototype_pattern.rs`        | Clones workout session templates                            |
| Singleton         | `singleton_pattern.rs`        | Global access to shared app settings                        |

### 🧪 Unit Testing
- Centralized in: `tests_health_tracker_patterns.rs`
- Covers pattern usage, object instantiation, edge cases

### 📘 Documentation
- `README_health_tracker.md` – This file
- `CHANGELOG.md` – Full commit and implementation log

---

## 🧠 Learning Outcomes
- Designed scalable object models in Rust
- Applied creational patterns to real-world use cases
- Developed and tested reusable modules

---

## 📁 File Structure
```
/assignment10/
├── src/
│   └── fitness_tracker_classes.rs
├── creational_patterns/
│   ├── simple_factory.rs
│   ├── factory_method.rs
│   ├── abstract_factory.rs
│   ├── builder_pattern.rs
│   ├── prototype_pattern.rs
│   ├── singleton_pattern.rs
├── tests/
│   └── tests_health_tracker_patterns.rs
├── README_health_tracker.md
├── CHANGELOG.md
```

📅 **Submitted:** April 2025
