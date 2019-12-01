use std::io::{stdin, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = stdin();
    let lines = stdin.lock().lines();

    let mass: u64 = lines.fold(0, |sum, mass_str| {
        let mass: u64 = u64::from_str(&mass_str.unwrap()).unwrap();
        sum + calculate_recursive_fuel_from_mass(mass)
    });
    println!("Total mass: {}", mass);
}

fn calculate_fuel_from_mass(mass: u64) -> i64 {
    ((((mass as f64) / 3.0).floor()) - 2.0) as i64
}

fn calculate_recursive_fuel_from_mass(mass: u64) -> u64 {
    let mut sum = 0;
    let mut mass = mass as i64;
    loop {
        mass = calculate_fuel_from_mass(mass as u64);
        if mass <= 0 {
            break;
        } else {
            sum += mass;
        }
    }
    sum as u64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calculate_fuel_1() {
        assert_eq!(calculate_recursive_fuel_from_mass(14), 2);
    }

    #[test]
    fn calculate_fuel_2() {
        assert_eq!(calculate_recursive_fuel_from_mass(1969), 966);
    }

    #[test]
    fn calculate_fuel_3() {
        assert_eq!(calculate_recursive_fuel_from_mass(100756), 50346);
    }
}
