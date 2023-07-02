static mut current_selection: Option<(Vec<String>, u8)> = None;

use std::fs;
use std::io::Write;

#[derive(Debug, PartialEq)]
pub enum Answer<'a>
{
    Empty,
    Some(&'a String),
    Longest(&'a String)
}

pub fn check_answers<'a>(answer: &str, answers: &'a Vec<String>, longest: u8) -> Answer<'a>
{
    let answer = &clean_str(answer);

    for a in answers
    {
        if answer.contains(a)
        {
            println!("{}", longest);
            return match a.len() == longest.into()
            {
                false => Answer::Some(a),
                true => Answer::Longest(a),
            }
        }
    }

    Answer::Empty
}

pub fn set_cache(path: &str) -> (Vec<String>, u8)
{
    let mut lines = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split("\n").map(|x| x[..x.len()].to_owned())
        .collect::<Vec<String>>();

    println!("loaded file");

    let longest = lines.pop()
        .expect("Empty file").chars().nth(0).unwrap();

    (lines, longest as u8 - 32)
}

pub fn load_cached() -> Option<(Vec<String>, u8)>
{
   unsafe { current_selection.clone() } 
}

pub fn clean_file(path: &str, row: usize) -> Result<(), std::io::Error>
{
    let lines = std::fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .split("\n").map(|x| x.to_owned())
        .collect::<Vec<String>>();

    let mut longest = 0;

    let mut new_path = "data/".to_owned() + path;
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

use std::env;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn load_file() {
        unsafe {
        current_selection = Some(set_cache("./src/csv/worldcities.csv")); 
        println!("{:?}", current_selection); }
    }

    #[test]
    fn correct_answer() {
        unsafe {
            current_selection = Some(set_cache("./src/csv/worldcities.csv"));
        }

        let selection_cache = load_cached();
        let Some((answers, longest)) = selection_cache else { panic!("broken"); };

        assert_eq!(check_answers("kuwaitcity", &answers, longest), 
                    Answer::Some(&"kuwaitcity".to_string()));
    }

    #[test]
    fn longest() {
        unsafe {
            current_selection = Some(set_cache("./src/csv/worldcities.csv"));
        }

        let selection_cache = load_cached();
        let Some((answers, longest)) = selection_cache else { panic!("broken"); };

        assert_eq!(check_answers("newyorkcity", &answers, longest), 
                    Answer::Longest(&"newyorkcity".to_string()));
    }

    #[test]
    fn incorrect_answer() {
        unsafe {current_selection = Some(set_cache("./src/csv/worldcities.csv"));}

        let selection_cache = load_cached();
        let Some((answers, longest)) = selection_cache else { panic!("broken"); };

        assert_eq!(check_answers("bucket", &answers, longest), Answer::Empty);
    }

    #[test]
    pub fn extra_chars() {
        unsafe {current_selection = Some(set_cache("./src/csv/worldcities.csv"));}

        let selection_cache = load_cached();
        let Some((answers, longest)) = selection_cache else { panic!("broken"); };

        assert_eq!(check_answers("KU%w**ai--t'ci(ty", &answers, longest), 
                    Answer::Some(&"kuwaitcity".to_string()));
    }

    #[test]
    pub fn sub_answer() {
        unsafe {current_selection = Some(set_cache("./src/csv/worldcities.csv"));}

        let selection_cache = load_cached();
        let Some((answers, longest)) = selection_cache else { panic!("broken"); };

        assert_eq!(check_answers("0000kuwaitcity555", &answers, longest), 
                    Answer::Some(&"kuwaitcity".to_string()));
    }
}