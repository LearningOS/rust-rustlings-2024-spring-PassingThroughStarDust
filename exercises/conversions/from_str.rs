// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

//

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

//  Method 1
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let pieces: Vec<&str> = s.split(',').collect();
        match pieces.len() {
            1 if pieces[0] == "" => Err(Self::Err::Empty),
            2 => {
                if pieces[0] == "" {
                    Err(Self::Err::NoName)
                } else {
                    let result = pieces[1].parse::<usize>().map_err(|err| {Self::Err::ParseInt(err)})?;
                    //the signature of map_err is just "pub fn map_err<F, O>(self, op: O) -> Result<T, F>"
                    /* ? was previously explained as either unwrap or return Err(err). 
                    This is only mostly true. It actually means unwrap or return Err(From::from(err)). 
                    Since From::from is a conversion utility between different types, 
                    this means that if you ? where the error is convertible to the return type, 
                    it will convert automatically.  */
                    Ok(Self {
                        name: pieces[0].to_string(),
                        age: result,
                        })
                }
            }
            _ => Err(Self::Err::BadLen)
        }

    }
}

/*  Method 2
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let pieces: Vec<&str> = s.split(',').collect();
        match pieces.len() {
            1 if pieces[0] == "" => Err(Self::Err::Empty),
            2 => {
                if pieces[0] == "" {
                    Err(Self::Err::NoName)
                } else {
                    match pieces[1].parse::<usize>() {
                        Ok(result) => Ok(Self {
                            name: pieces[0].to_string(),
                            age: result,
                            }),
                        Err(result) => Err(Self::Err::ParseInt(result))
                    }
                }
            }
            _ => Err(Self::Err::BadLen)
        }

    }
} */

/*  Method 3
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        match s.is_empty() {
            true => Err(ParsePersonError::Empty),
            false => {
                let parts: Vec<&str> = s.split(',').collect();
                match parts.len() != 2 {
                    true => return Err(ParsePersonError::BadLen),
                    false => {
                        let name = parts[0].trim();
                        if name.is_empty() {
                            return Err(ParsePersonError::NoName)
                        } 
                        let number = parts[1].parse(); 
                        if let Err(x) = number{
                            return Err(ParsePersonError::ParseInt(x))
                        }
                        Ok(Person{name:String::from(name), age:number.unwrap()})
                    }
                }
            }
        }
    }
}
 */

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
