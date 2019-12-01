use std::io::{stdin, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = stdin();
    let lines = stdin.lock().lines();

    let fuel_sum: u64 = lines.fold(0, |sum, mass_str| {
        let mass: u64 = u64::from_str(&mass_str.unwrap()).unwrap();
        sum + calculate_fuel_from_mass(mass)
    });
    println!("Total mass: {}", fuel_sum);
}

fn calculate_fuel_from_mass(mass: u64) -> u64 {
    ((((mass as f64) / 3.0).floor()) - 2.0) as u64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calculate_fuel_1() {
        assert_eq!(calculate_fuel_from_mass(12), 2);
    }

    #[test]
    fn calculate_fuel_2() {
        assert_eq!(calculate_fuel_from_mass(14), 2);
    }

    #[test]
    fn calculate_fuel_3() {
        assert_eq!(calculate_fuel_from_mass(1969), 654);
    }

    #[test]
    fn calculate_fuel_4() {
        assert_eq!(calculate_fuel_from_mass(100756), 33583);
    }
}
