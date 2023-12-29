use std::{
    io::{stdin, stdout, Write},
    usize,
};

fn get_height_in_metres(height: &f32) -> f32 {
    height / 39.3701
}

fn get_height_in_feet_inch(hieght: &i16) -> Vec<i16> {
    vec![hieght / 12, hieght % 12]
}

fn main() {
    let mut value = String::new();
    print!("Enter your height (inches): ");
    stdout().flush().unwrap();
    let input: Result<usize, std::io::Error> = stdin().read_line(&mut value);
    match input {
        Ok(_b) => {
            let hi16 = value.trim().parse::<i16>();
            let hf = value.trim().parse::<f32>();

            match hi16 {
                Ok(h) => {
                    let feets_inches = get_height_in_feet_inch(&h);
                    println!(
                        "Write either \"I'm {}\'{}\"!\" \\ \"I am {}\" tall!\"",
                        feets_inches[0], feets_inches[1], h
                    );
                }
                Err(e) => println!("Error parsing height: {}", e),
            }

            match hf {
                Ok(h) => println!("(Height in metric: {:.4}m)", get_height_in_metres(&h)),
                Err(e) => println!("Error parsing height: {}", e),
            }
        }
        Err(e) => println!("Error {}", e),
    }
}

#[cfg(test)]

mod tests {
    use super::{get_height_in_feet_inch, get_height_in_metres};

    #[test]
    fn test_case_1() {
        let input: i16 = 76;
        let output_i = get_height_in_feet_inch(&input);

        let result = vec![6, 4];
        let matching = output_i
            .iter()
            .zip(&result)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 2);
    }

    #[test]
    fn test_case_2() {
        let input: i16 = 122;
        let output_i = get_height_in_feet_inch(&input);

        let result = vec![10, 2];
        let matching = output_i
            .iter()
            .zip(&result)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 2);
    }

    #[test]
    fn test_case_3() {
        let input: i16 = 57;
        let output_i = get_height_in_feet_inch(&input);

        let result = vec![4, 9];
        let matching = output_i
            .iter()
            .zip(&result)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, 2);
    }

    #[test]
    fn test_case_4() {
        let input = 76.0;
        let output_i = get_height_in_metres(&input);
        let result = input / 39.3701;
        assert_eq!(output_i, result);
    }

    #[test]
    fn test_case_5() {
        let input = 122.0;
        let output_i = get_height_in_metres(&input);
        let result = input / 39.3701;
        assert_eq!(output_i, result);
    }

    #[test]
    fn test_case_6() {
        let input = 57.0;
        let output_i = get_height_in_metres(&input);
        let result = input / 39.3701;
        assert_eq!(output_i, result);
    }
}
