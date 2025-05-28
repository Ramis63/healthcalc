use std::io;

fn main() {
    println!("Welcome to HealthCalc - Your BMI & Health Assistant!");

    // Ask for height in cm
    println!("Enter your height in cm:");
    let height_cm = read_input().trim().parse::<f32>().unwrap_or(0.0);
    let height_m = height_cm / 100.0;

    // Ask for weight in kg
    println!("Enter your weight in kg:");
    let weight = read_input().trim().parse::<f32>().unwrap_or(0.0);

    if height_m <= 0.0 || weight <= 0.0 {
        println!("Invalid input. Please enter positive numbers for height and weight.");
        return;
    }

    // Calculate BMI
    let bmi = weight / (height_m * height_m);
    println!("\nYour BMI is: {:.2}", bmi);

    // Show category and tips
    match bmi {
        bmi if bmi < 18.5 => {
            println!("Category: Underweight");
            println!("Tips: Eat nutrient-rich foods, increase calorie intake, and consult a nutritionist.");
        }
        bmi if bmi < 24.9 => {
            println!("Category: Normal weight");
            println!("Tips: Maintain your routine, stay active, and eat a balanced diet.");
        }
        bmi if bmi < 29.9 => {
            println!("Category: Overweight");
            println!("Tips: Exercise regularly, reduce sugar and fried food intake, and stay hydrated.");
        }
        _ => {
            println!("Category: Obese");
            println!("Tips: Consider a personalized health plan, exercise, and speak to a healthcare provider.");
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
