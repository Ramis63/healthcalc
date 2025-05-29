use std::io::{self, Write};

fn main() {
    println!("Welcome to HealthCalc (BMI Calculator)!");
    println!("----------------------------------------");

    // Input: Weight
    print!("Enter your weight in kg: ");
    io::stdout().flush().unwrap();
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).unwrap();
    let weight: f32 = match weight_input.trim().parse() {
        Ok(w) => w,
        Err(_) => {
            println!("Invalid weight input.");
            wait_to_exit();
            return;
        }
    };

    // Input: Height
    print!("Enter your height in meters (e.g., 1.75): ");
    io::stdout().flush().unwrap();
    let mut height_input = String::new();
    io::stdin().read_line(&mut height_input).unwrap();
    let height: f32 = match height_input.trim().parse() {
        Ok(h) => h,
        Err(_) => {
            println!("Invalid height input.");
            wait_to_exit();
            return;
        }
    };

    // Calculate BMI
    let bmi = weight / (height * height);
    println!("\nYour BMI is: {:.2}", bmi);

    // Show category and tips
    match bmi {
        bmi if bmi < 18.5 => {
            println!("Category: Underweight");
            println!("Tips: Eat nutrient-rich foods, increase calorie intake, and consult a healthcare provider if needed.");
        }
        bmi if bmi < 24.9 => {
            println!("Category: Normal weight");
            println!("Tips: Maintain your routine, stay active, and eat a balanced diet.");
        }
        bmi if bmi < 29.9 => {
            println!("Category: Overweight");
            println!("Tips: Exercise regularly, reduce sugar and fried food intake, and monitor your portion sizes.");
        }
        _ => {
            println!("Category: Obese");
            println!("Tips: Consider a personalized health plan, exercise, and speak to a healthcare professional for advice.");
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
