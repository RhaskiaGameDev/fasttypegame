use std::fs;
use std::io::Write;
use std::fmt;
use serde::*;

//pub const world_cities: &str = include_str!("csv/world-cities.csv");
pub const countries: &str = include_str!("csv/countries.csv");

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub name: &'static str,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Answer
{
    Empty,
    Some(String),
    Longest(String)
}

impl fmt::Display for Answer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::Empty => fmt.write_str("0☆"),
            Answer::Some(an) => fmt.write_str(&format!("{}★", an.len())),
            Answer::Longest(an) => fmt.write_str(&format!("{}⭐", an.len())),
        };

        Ok(())
    }
}

pub fn check_answer(answer: &str, answers: &Vec<String>, longest: u8) -> Answer
{
    let answer = &clean_str(answer);

    for a in answers
    {
        if answer == a //TODO: make so it chooses largest substring
        {
            println!("{}", longest);
            return match a.len() == longest.into()
            {
                false => Answer::Some(a.clone()),
                true => Answer::Longest(a.clone()),
            }
        }
    }

    Answer::Empty
}

pub fn load() -> (Vec<String>, u8)
{
    let mut lines =
        countries
        .split("\n").map(|x| x[..x.len()].to_owned())
        .collect::<Vec<String>>();

    println!("loaded file");

    let longest = lines.pop()
        .expect("Empty file").chars().nth(0).unwrap();

    (lines, longest as u8 - 33)
}

pub fn clean_file(path: &str, row: usize) -> Result<(), std::io::Error>
{
    let lines = std::fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split("\n").map(|x| x.to_owned())
        .collect::<Vec<String>>();

    let mut longest = 0;

    let mut new_path = "csv/".to_owned() + path;
    let mut file = std::fs::File::create(new_path)?;

    for line in lines
    {
        let split: Vec<&str> = line.split(",").collect();
        let word = clean_str(split[row]) + "\n";

        if word.len() > 1
        { file.write(word.as_bytes())?; }

        if word.len() > longest
        {
            longest = word.len();
        }
        println!("{}", word);
    }

    file.write(&[longest as u8 + 32, 32])?;

    Ok(())
}

fn clean_str(s: &str) -> String
{
    diacritics::remove_diacritics(
        &s.to_string()
        .to_ascii_lowercase())
        .chars()
        .filter(|x| x.is_numeric() || x.is_alphabetic())
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use crate::*;
//
//     #[test]
//     fn load_file() {
//         unsafe {
//         current_selection = Some(load("./src/csv/worldcities.csv"));
//         println!("{:?}", current_selection); }
//     }
//
//     #[test]
//     fn correct_answer() {
//         unsafe {
//             current_selection = Some(load("./src/csv/worldcities.csv"));
//         }
//
//         let selection_cache = load_cached();
//         let Some((answers, longest)) = selection_cache else { panic!("broken"); };
//
//         assert_eq!(check_answer("kuwaitcity", &answers, longest),
//                    Answer::Some(&"kuwaitcity".to_string()));
//     }
//
//     #[test]
//     fn longest() {
//         unsafe {
//             current_selection = Some(load("./src/csv/worldcities.csv"));
//         }
//
//         let selection_cache = load_cached();
//         let Some((answers, longest)) = selection_cache else { panic!("broken"); };
//
//         assert_eq!(check_answer("newyorkcity", &answers, longest),
//                    Answer::Longest(&"newyorkcity".to_string()));
//     }
//
//     #[test]
//     fn incorrect_answer() {
//         unsafe {current_selection = Some(load("./src/csv/worldcities.csv"));}
//
//         let selection_cache = load_cached();
//         let Some((answers, longest)) = selection_cache else { panic!("broken"); };
//
//         assert_eq!(check_answer("bucket", &answers, longest), Answer::Empty);
//     }
//
//     #[test]
//     pub fn extra_chars() {
//         unsafe {current_selection = Some(load("./src/csv/worldcities.csv"));}
//
//         let selection_cache = load_cached();
//         let Some((answers, longest)) = selection_cache else { panic!("broken"); };
//
//         assert_eq!(check_answer("KU%w**ai--t'ci(ty", &answers, longest),
//                    Answer::Some(&"kuwaitcity".to_string()));
//     }
//
//     #[test]
//     pub fn sub_answer() {
//         unsafe {current_selection = Some(load("./src/csv/worldcities.csv"));}
//
//         let selection_cache = load_cached();
//         let Some((answers, longest)) = selection_cache else { panic!("broken"); };
//
//         assert_eq!(check_answer("0000kuwaitcity555", &answers, longest),
//                    Answer::Some(&"kuwaitcity".to_string()));
//     }
// }