pub mod econ;
pub mod object;
pub mod value;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, str::FromStr};

    use econ::Econ;
    use object::Access;

    use super::*;

    #[test]
    fn simple() {
        let obj = Econ::create(
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
        assert_eq!(true, matches!(Econ::create("test/Simple.econ", true), Ok(_)));
    }

    #[test]
    fn complex_from_file() {
        assert_eq!(true, matches!(Econ::create("test/Complex.econ", true), Ok(_)));
    }

    #[test]
    fn functions() {
        let obj = Econ::create(
        r#"
        {
            aa: "Hello,how,are,you",
            a: to_string(map(chars($aa), x => $x == "," ? " " : $x))
        }
        "#, true);

        println!("{:?}", obj);

        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn macros() {
        let obj = Econ::create(
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
            people_above_average: sort(keys(filter($people, x => $x.val.salary > $average_salary)), |a, b| => !a > !b)
        }
        "#, true);

        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn api_access_obj() {
        let obj = Econ::from(
        r#"
        {
            a: {
                b: {
                    c: [
                        1,
                        2,
                        3,
                        4
                    ]
                }
            }
        }
        "#);
        assert_eq!(3f64, obj["a"]["b"]["c"][2].value::<f64>());
    }

    #[test]
    fn constraints() {
        let obj = Econ::create(
        r#"
        {
            @!{string, x => $x == "Hello World", "No Hello Worlds!"}
            a: "Hello World"
        }
        "#, true);
        
        assert_eq!(true, matches!(obj, Err(_)));
    }

    #[test]
    fn ref_depth() {
        let obj = Econ::create(
        r#"
        {
            a: {
                aa: 1,
                ab: $aa
            },
            b: {
                bb: $aa,
                ba: !a
            }
        }
        "#, true);
        
        assert_eq!(true, matches!(obj, Ok(_)));
    }
}

