use std::io::{self, Write};

fn main() {
    println!("=== Температурный конвертер ===\n");

    loop {
        println!("Выберите направление перевода:");
        println!("1. °C -> °F");
        println!("2. °F -> °C");
        println!("3. °C -> K");
        println!("4. K -> °C");
        println!("5. °F -> K");
        println!("6. K -> °F");
        println!("0. Выход");

        print!("Ваш выбор: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "0" => {
                println!("До свидания!");
                break;
            }
            "1" => convert_c_to_f(),
            "2" => convert_f_to_c(),
            "3" => convert_c_to_k(),
            "4" => convert_k_to_c(),
            "5" => convert_f_to_k(),
            "6" => convert_k_to_f(),
            _ => println!("Неверный выбор, попробуйте снова.\n"),
        }

        println!("Продолжить? (Enter - да, 0 - выход)");
        let mut cont = String::new();
        io::stdin().read_line(&mut cont).unwrap();
        if cont.trim() == "0" {
            println!("До свидания!");
            break;
        }
    }
}

fn read_temperature(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Ошибка: введите число."),
        }
    }
}

// °C -> °F : (C * 9/5) + 32
fn convert_c_to_f() {
    let c = read_temperature("Введите температуру в °C: ");
    let f = celsius_to_fahrenheit(c);
    println!("{:.2} °C = {:.2} °F", c, f);
}

// °F -> °C : (F - 32) * 5/9
fn convert_f_to_c() {
    let f = read_temperature("Введите температуру в °F: ");
    let c = fahrenheit_to_celsius(f);
    println!("{:.2} °F = {:.2} °C", f, c);
}

// °C -> K : C + 273.15
fn convert_c_to_k() {
    let c = read_temperature("Введите температуру в °C: ");
    let k = celsius_to_kelvin(c);
    println!("{:.2} °C = {:.2} K", c, k);
}

// K -> °C : K - 273.15
fn convert_k_to_c() {
    let k = read_temperature("Введите температуру в K: ");
    let c = kelvin_to_celsius(k);
    println!("{:.2} K = {:.2} °C", k, c);
}

// °F -> K : (F - 32) * 5/9 + 273.15
fn convert_f_to_k() {
    let f = read_temperature("Введите температуру в °F: ");
    let k = fahrenheit_to_kelvin(f);
    println!("{:.2} °F = {:.2} K", f, k);
}

// K -> °F : (K - 273.15) * 9/5 + 32
fn convert_k_to_f() {
    let k = read_temperature("Введите температуру в K: ");
    let f = kelvin_to_fahrenheit(k);
    println!("{:.2} K = {:.2} °F", k, f);
}

// Чистые функции конвертации (без ввода/вывода, для тестирования)
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn kelvin_to_celsius(k: f64) -> f64 {
    k - 273.15
}

fn fahrenheit_to_kelvin(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0 + 273.15
}

fn kelvin_to_fahrenheit(k: f64) -> f64 {
    (k - 273.15) * 9.0 / 5.0 + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(20.0), 68.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
        assert_eq!(celsius_to_fahrenheit(-273.15), -459.66999999999996);
        assert!((celsius_to_fahrenheit(36.6) - 97.88).abs() < 0.001);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(68.0), 20.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
        assert_eq!(fahrenheit_to_celsius(-459.67), -273.15);
        assert!((fahrenheit_to_celsius(98.6) - 37.0).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        assert_eq!(celsius_to_kelvin(-273.15), 0.0);
        assert_eq!(celsius_to_kelvin(0.0), 273.15);
        assert_eq!(celsius_to_kelvin(100.0), 373.15);
        assert_eq!(celsius_to_kelvin(20.0), 293.15);
        assert_eq!(celsius_to_kelvin(-50.0), 223.14999999999998);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        assert_eq!(kelvin_to_celsius(0.0), -273.15);
        assert_eq!(kelvin_to_celsius(273.15), 0.0);
        assert_eq!(kelvin_to_celsius(373.15), 100.0);
        assert_eq!(kelvin_to_celsius(293.15), 20.0);
    }

    #[test]
    fn test_fahrenheit_to_kelvin() {
        assert_eq!(fahrenheit_to_kelvin(-459.67), 0.0);
        assert_eq!(fahrenheit_to_kelvin(32.0), 273.15);
        assert_eq!(fahrenheit_to_kelvin(212.0), 373.15);
        assert!((fahrenheit_to_kelvin(68.0) - 293.15).abs() < 0.001);
        assert!((fahrenheit_to_kelvin(-40.0) - 233.15).abs() < 0.001);
    }

    #[test]
    fn test_kelvin_to_fahrenheit() {
        assert_eq!(kelvin_to_fahrenheit(0.0), -459.66999999999996);
        assert_eq!(kelvin_to_fahrenheit(273.15), 32.0);
        assert_eq!(kelvin_to_fahrenheit(373.15), 212.0);
        assert!((kelvin_to_fahrenheit(293.15) - 68.0).abs() < 0.001);
        assert!((kelvin_to_fahrenheit(233.15) - (-40.0)).abs() < 0.001);
    }

    #[test]
    fn test_roundtrip_celsius_fahrenheit() {
        for c in [-273.15, -40.0, 0.0, 20.0, 36.6, 100.0] {
            let f = celsius_to_fahrenheit(c);
            let c_back = fahrenheit_to_celsius(f);
            assert!((c - c_back).abs() < 0.0001);
        }
    }

    #[test]
    fn test_roundtrip_celsius_kelvin() {
        for c in [-273.15, -50.0, 0.0, 20.0, 100.0] {
            let k = celsius_to_kelvin(c);
            let c_back = kelvin_to_celsius(k);
            assert!((c - c_back).abs() < 0.0001);
        }
    }

    #[test]
    fn test_roundtrip_fahrenheit_kelvin() {
        for f in [-459.67, -40.0, 0.0, 32.0, 68.0, 212.0] {
            let k = fahrenheit_to_kelvin(f);
            let f_back = kelvin_to_fahrenheit(k);
            assert!((f - f_back).abs() < 0.0001);
        }
    }

    #[test]
    fn test_chained_conversions() {
        let original_c = 25.0;
        let f = celsius_to_fahrenheit(original_c);
        let k = fahrenheit_to_kelvin(f);
        let final_c = kelvin_to_celsius(k);
        assert!((original_c - final_c).abs() < 0.0001);

        let original_f = 75.0;
        let c = fahrenheit_to_celsius(original_f);
        let k = celsius_to_kelvin(c);
        let final_f = kelvin_to_fahrenheit(k);
        assert!((original_f - final_f).abs() < 0.0001);
    }

    #[test]
    fn test_extreme_values() {
        let high_c = 1_000_000.0;
        let high_f = celsius_to_fahrenheit(high_c);
        assert!(high_f > high_c);

        let low_c = -273.149;
        let low_k = celsius_to_kelvin(low_c);
        assert!(low_k > 0.0);

        let absolute_zero_f = -459.67;
        let absolute_zero_k = fahrenheit_to_kelvin(absolute_zero_f);
        assert!((absolute_zero_k - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_identity_conversions() {
        let c = 23.5;
        let k1 = celsius_to_kelvin(c);
        let c_back = kelvin_to_celsius(k1);
        let k2 = celsius_to_kelvin(c_back);
        assert!((k1 - k2).abs() < 0.0001);

        let f = 99.9;
        let c = fahrenheit_to_celsius(f);
        let f_back = celsius_to_fahrenheit(c);
        assert!((f - f_back).abs() < 0.0001);
    }
}
