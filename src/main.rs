use std::io::{self, Write};

fn main() {
    println!("Welcome to HealthCalc - Your BMI & Health Assistant!");

    // Input: Height in centimeters
    print!("Enter your height in centimeters: ");
    io::stdout().flush().unwrap();
    let mut height_cm = String::new();
    io::stdin().read_line(&mut height_cm).unwrap();
    let height_cm: f32 = match height_cm.trim().parse() {
        Ok(h) => h,
        Err(_) => {
            println!("Invalid height input.");
            wait_to_exit();
            return;
        }
    };
    let height_m = height_cm / 100.0;

    // Input: Weight in kilograms
    print!("Enter your weight in kilograms: ");
    io::stdout().flush().unwrap();
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).unwrap();
    let weight: f32 = match weight.trim().parse() {
        Ok(w) => w,
        Err(_) => {
            println!("Invalid weight input.");
            wait_to_exit();
            return;
        }
    };

    if height_m <= 0.0 || weight <= 0.0 {
        println!("Invalid input. Please enter positive numbers for height and weight.");
        wait_to_exit();
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

    wait_to_exit();
}

// Function to pause before exit
fn wait_to_exit() {
    println!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let mut exit_input = String::new();
    io::stdin().read_line(&mut exit_input).unwrap();
}
