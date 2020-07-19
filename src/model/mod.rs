#![allow(dead_code)]

pub mod tests;

use common::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct DateData{
    year:u16,
    month:u8,
    day:u8,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TodoElement {
    Project(String),
    Context(String),
    Text(String),
    Due(DateData),
}

impl TodoElement {
    fn project(str: &str)-> TodoElement {
        TodoElement::Project(String::from(str))
    }

    fn context(str: &str)-> TodoElement {
        TodoElement::Context(String::from(str))
    }

    fn text(str: &str)-> TodoElement {
        TodoElement::Text(String::from(str))
    }

    fn create_prefix_parser(prefix:char, element_constructor: &'static dyn Fn(&str)->TodoElement)->Box<dyn Fn(&str)-> Result<TodoElement, ParsingError>> {
        Box::new(move |input: &str| {
            if let Some(data) = input.strip_prefix(prefix) {
                Result::Ok(element_constructor(data))
            } else {
                Result::Err(ParsingError{message:"error parsing entity"})
            }
        })
    }
    
    pub fn try_parse_project(input:&str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_prefix_parser('+', &TodoElement::project)(input)
    }
    
    fn try_parse_context(input:&str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_prefix_parser('@', &TodoElement::context)(input)
    }
    
    fn try_parse_text(input:&str) -> Result<TodoElement, ParsingError> {
        Result::Ok(TodoElement::text(input))
    }

    fn try_parse_due(input: &str) -> Result<TodoElement, ParsingError> {
        if let Some(str_date) = input.strip_prefix("due:") {
            let x:Vec<&str> = str_date.split('-').collect();
            match (x.get(0),x.get(1),x.get(2)) {
                (Some(year_str), Some(month_str), Some(day_str)) => {
                    Result::Ok(
                        TodoElement::Due(
                            DateData{
                                year: year_str.parse::<u16>().map_err(|_|{ParsingError{message:"error parsing year"}})?,
                                month: month_str.parse::<u8>().map_err(|_|{ParsingError{message:"error parsing month"}})?,
                                day: day_str.parse::<u8>().map_err(|_|{ParsingError{message:"error parsing day"}})?
                            }
                        )
                    )
                },
                _ => Result::Err(ParsingError{message:"error parsing date"})
            }
        } else {
            Result::Err(ParsingError{message:"error parsing entity"})
        }
    }
    
    pub fn parse(input: &str) -> Result<TodoElement, ParsingError> {
        let parsers = [
            TodoElement::try_parse_project, 
            TodoElement::try_parse_context, 
            TodoElement::try_parse_due,
            TodoElement::try_parse_text,
        ];
        let mut iterator = parsers.iter();
        let mut last_error: Option<Result<TodoElement, ParsingError>> = Option::None;
        while let Some(parser) = iterator.next() {
            match parser(input) {
                parse_result @ Ok(_) => return parse_result,
                error @ Err(_) => last_error = Option::Some(error),
            }
        }
        last_error.unwrap()
    }
}

struct TodoEntry {
    parts:Vec<TodoElement>
}

impl TodoEntry {
    pub fn parse(data:&str) -> Result<TodoEntry, ParsingError>{
        let mut result = TodoEntry { parts : Vec::new() };
        for split in data.split_whitespace() {
            result.parts.push(TodoElement::parse(split).unwrap());
        }
        Result::Ok(result)
    }
}

struct TodoData {
    entries:Vec<TodoEntry>
}

impl TodoData {
    pub fn parse(data:&str) -> Result<TodoData, ParsingError> {
        let mut result = TodoData{
            entries:Vec::new()
        };
        for line in data.lines() {
            result.entries.push(TodoEntry::parse(line).expect("error parsing"));
        }
        Result::Ok(
            result
        )
    }
}

pub mod common {
    #[derive(Debug)]
    pub struct ParsingError {
        pub message: &'static str
    }
}