/**
 * Calculated the Desired Cycle Time
 * Test project to check if the math is correct. 
 * Dec 7, 2022
 * Edward. N
 */

use std::io;
use round::round_up;

fn main() {
    // Production Time
    let mut temp = String::new();
    println!("\nWhat is the Production Time for this process?");
    
    io::stdin()
        .read_line( &mut temp)
        .expect("Failed to Readline...");

    // Time given to production in minutes - Variable
    let time_available = temp
        .trim()
        .parse::<f64>()
        .expect( "error in time aval var...");

    // Desired Unit of Output
    let mut temp = String::new(); // Cleared data in stream 
    println!("\nWhat is the Desried Unit of Output for this process?");

    io::stdin()
        .read_line( &mut temp)
        .expect("Failed to Readline...");

    // Desired unit of Output - Variable
    let d_units = temp
        .trim()
        .parse::<f64>()
        .expect("Error in d_unit variable..");

    // Calc
    let desired_ctime = round_up(time_available / d_units, 2);

    // Print w/ formatting
    println!("\nThe Production Time Availible is: {} mins", time_available);
    println!("The Desired Unit of Output is: {} units", d_units);
    println!("The Desired Cycle Time is: {} mins", desired_ctime);
}