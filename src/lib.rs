pub mod json_data;
pub mod object;
pub mod value;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use json_data::Json;

    use super::*;

    #[test]
    fn simple() {
        let obj = Json::create(
        r#"
        {
            a: 1,
            b: 2,
            c: 3
        }
        "#, true);
        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn simple_from_file() {
        let file = fs::read_to_string(
            PathBuf::from_str("Cases/Simple.json").expect("Invalid Path.")
        );
        assert_eq!(true, matches!(Json::create(&file.unwrap(), true), Ok(_)));
    }

    #[test]
    fn complex_from_file() {
        let file = fs::read_to_string(
            PathBuf::from_str("Cases/Complex.json").expect("Invalid Path.")
        );
        assert_eq!(true, matches!(Json::create(&file.unwrap(), true), Ok(_)));
    }

    #[test]
    fn functions() {
        let obj = Json::create(
        r#"
        {
            aa: "Hello,how,are,you",
            a: string(map(chars($aa), x => $x == "," ? " " : $x))
        }
        "#, true);

        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn macros() {
        let obj = Json::create(
        r#"
        {
            @person(n, a, s) n: { age: a, salary: s }
            people: {
                @person(John, 25, 90),
                @person(Suzie, 22, 100),
                @person(Max, 35, 150),
                @person(Mary, 27, 125),
                @person(Lisa, 32, 120)
            },
            average_salary: fold($people, |x, acc| => $acc + $x.val.salary) / #$people,
            people_above_average: sort(keys(filter($people, x => $x.val.salary > $average_salary)), |a, b| => $a > $b)
        }
        "#, true);

        assert_eq!(true, matches!(obj, Ok(_)));
    }
}

