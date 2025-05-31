//! # Paint Calculator
//!
//! This module implements an interactive paint calculator application that
//! determines the amount of paint needed for rooms of various shapes.
//!
//! ## Features
//!
//! - **Multiple Room Types**: Supports rectangular, circular, and L-shaped rooms
//! - **Area Calculation**: Accurately calculates square footage based on room dimensions
//! - **Paint Estimation**: Determines required gallons based on standard coverage rates
//! - **User Interaction**: Provides clear prompts and guides users through input process
//! - **Rounding Logic**: Ensures users purchase sufficient paint by rounding up to whole gallons
use std::io::Write;

trait Area {
    fn area(&self) -> f64;
}

enum RoomType {
    Rectangular {
        length: f64,
        width: f64,
    },
    Circular {
        diameter: f64,
    },
    LShaped {
        length: f64,
        width: f64,
        alcove_length: f64,
        alcove_width: f64,
    },
}

impl Area for RoomType {
    fn area(&self) -> f64 {
        match self {
            RoomType::Rectangular { length, width } => length * width,
            RoomType::Circular { diameter } => {
                let radius = diameter / 2.0;
                std::f64::consts::PI * radius * radius
            }
            RoomType::LShaped {
                length,
                width,
                alcove_length,
                alcove_width,
            } => {
                let main_area = length * width;
                let alcove_area = alcove_length * alcove_width;
                main_area + alcove_area
            }
        }
    }
}

fn prompt_for_float(prompt: &str) -> f64 {
    loop {
        print!("{prompt} ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Ok(value) = input.trim().parse::<f64>() {
            return value;
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}

fn prompt_for_room_type() -> RoomType {
    loop {
        println!("Choose the type of room:");
        println!("1. Rectangular");
        println!("2. Circular");
        println!("3. L-Shaped");

        let mut choice = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut choice) {
            eprintln!("Error: {}", e);
            continue;
        }

        match choice.trim() {
            "1" => {
                let length = prompt_for_float("Enter the length of the room in feet:");
                let width = prompt_for_float("Enter the width of the room in feet:");
                return RoomType::Rectangular { length, width };
            }
            "2" => {
                let diameter = prompt_for_float("Enter the diameter of the room in feet:");
                return RoomType::Circular { diameter };
            }
            "3" => {
                let length = prompt_for_float("Enter the length of the main area in feet:");
                let width = prompt_for_float("Enter the width of the main area in feet:");
                let alcove_length = prompt_for_float("Enter the length of the alcove in feet:");
                let alcove_width = prompt_for_float("Enter the width of the alcove in feet:");
                return RoomType::LShaped {
                    length,
                    width,
                    alcove_length,
                    alcove_width,
                };
            }
            _ => println!("Invalid choice. Please select 1, 2, or 3."),
        }
    }
}

fn calculate_gallons_needed(room_type: &RoomType) -> u32 {
    const SQUARE_FT_PER_GALLON: f64 = 350.0; // Average coverage of paint in square feet per gallon
    let area = room_type.area();
    let gallons_needed = area / SQUARE_FT_PER_GALLON;
    if gallons_needed < 1.0 {
        1 // At least one gallon is needed
    } else {
        gallons_needed.ceil() as u32 // Round up to the nearest whole gallon
    }
}

fn main() {
    let room_type = prompt_for_room_type();
    let area = room_type.area();
    println!(
        "You will need {} gallons of paints to cover an area of {:.2} square feet.",
        calculate_gallons_needed(&room_type),
        area
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_gallons_needed_handles_exact_division() {
        // Test cases where area is exactly divisible by SQUARE_FT_PER_GALLON
        let room = RoomType::Rectangular {
            length: 35.0,
            width: 10.0,
        }; // 350 sq ft
        assert_eq!(calculate_gallons_needed(&room), 1); // Exactly 1 gallon

        let room = RoomType::Rectangular {
            length: 70.0,
            width: 10.0,
        }; // 700 sq ft
        assert_eq!(calculate_gallons_needed(&room), 2); // Exactly 2 gallons
    }

    #[test]
    fn calculate_gallons_needed_rounds_up_correctly() {
        // Test cases where we need to round up
        let room = RoomType::Rectangular {
            length: 20.0,
            width: 20.0,
        }; // 400 sq ft
        assert_eq!(calculate_gallons_needed(&room), 2); // Slightly more than 1 gallon (1.14)

        let room = RoomType::Circular { diameter: 10.0 }; // ~78.54 sq ft
        assert_eq!(calculate_gallons_needed(&room), 1); // Less than 1 gallon but rounds up

        let room = RoomType::LShaped {
            length: 30.0,
            width: 10.0,
            alcove_length: 10.0,
            alcove_width: 6.0,
        }; // 360 sq ft
        assert_eq!(calculate_gallons_needed(&room), 2); // Slightly more than 1 gallon (1.03)
    }

    #[test]
    fn calculate_gallons_needed_handles_small_areas() {
        // Test with small areas (less than one gallon)
        let room = RoomType::Rectangular {
            length: 10.0,
            width: 10.0,
        }; // 100 sq ft
        assert_eq!(calculate_gallons_needed(&room), 1); // Less than 1 gallon but minimum is 1

        let room = RoomType::Circular { diameter: 5.0 }; // ~19.63 sq ft
        assert_eq!(calculate_gallons_needed(&room), 1); // Much less than 1 gallon but minimum is 1
    }

    #[test]
    fn calculate_gallons_needed_handles_large_areas() {
        // Test with large areas
        let room = RoomType::Rectangular {
            length: 100.0,
            width: 100.0,
        }; // 10,000 sq ft
        assert_eq!(calculate_gallons_needed(&room), 29); // 28.57 gallons rounded up

        let room = RoomType::LShaped {
            length: 50.0,
            width: 30.0,
            alcove_length: 20.0,
            alcove_width: 15.0,
        }; // 1800 sq ft
        assert_eq!(calculate_gallons_needed(&room), 6); // 5.14 gallons rounded up
    }

    #[test]
    fn calculate_gallons_needed_handles_different_room_types() {
        // Test with different room types
        let rectangular = RoomType::Rectangular {
            length: 35.0,
            width: 10.0,
        }; // 350 sq ft
        assert_eq!(calculate_gallons_needed(&rectangular), 1);

        let circular = RoomType::Circular { diameter: 21.0 }; // ~346.36 sq ft
        assert_eq!(calculate_gallons_needed(&circular), 1);

        let l_shaped = RoomType::LShaped {
            length: 20.0,
            width: 15.0,
            alcove_length: 10.0,
            alcove_width: 5.0,
        }; // 350 sq ft
        assert_eq!(calculate_gallons_needed(&l_shaped), 1);
    }
}
