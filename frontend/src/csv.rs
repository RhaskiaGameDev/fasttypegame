pub(crate) mod loader;

use std::fs;
use std::io::Write;

pub fn return_answers() -> (Vec<String>, u8)
{
    let mut lines =
    // fs::read_to_string(new_path)
    //    .expect("Should have been able to read the file")
    loader::WORLDCITIES
        .split("\n").map(|x| x.to_owned())
        .collect::<Vec<String>>();

    println!("loaded file");

    let longest = lines.pop()
        .expect("Empty file").chars().last().unwrap();

    (lines, longest as u8)
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

    file.write(&longest.to_be_bytes())?;

    Ok(())
}

fn clean_str(s: &str) -> String
{
    diacritics::remove_diacritics(
        &s.to_string()
            .replace(&[' ', '\'', ',', '‘', '’', '-', '(', ')'][..], "")
            .to_ascii_lowercase())
}

use std::env;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

pub fn in_answers<'a>(answer: &str, answers: &'a Vec<String>) -> Option<&'a String>
{
    let answer = &clean_str(answer);

    for a in answers
    {
        if answer == a
        { return Some(a); }
    }

    None
}