pub mod econ;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod value;

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use econ::Econ;
    use serde::{Deserialize, Serialize};

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
        "#,
            true,
        );
        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn simple_from_file() {
        assert_eq!(
            true,
            matches!(Econ::create("test/Simple.econ", true), Ok(_))
        );
    }

    #[test]
    fn logic_arithmetic() {
        let obj = Econ::create(
            r#"
        {
            //a comment
            a: [1, "string", true, false, nil, 1 > 2, 1 < 2, 1 >= 1, 1 <= 1, 1 == 1, 1 ~= 2, true && true, true || false, true and true, true or true, not true, ~true],
            b: ((1+3)/2) * (5/3),
            c: #[1,2,3,4],
            d: "hello" == "hello" ? true : false,
            e: [1,2,3] + [4,5,6],
            f: {a: 1} + {b: 2},
            g: $e[2],
            h: $f.b,
            i: "I'm a multiline" \
               "string with \"* + - / filter\"" \
               "keywords and operators",
            j: [
                "multi_line" \
                "string" \
                "in" \
                "array",
                "single line",
                "another multi_line" \
                "string" \
                "in" \
                "array"
            ],
            k: "another multi_line" \
               "string" \
               "in" \
               "object",
            l: null,
            m: nil
        }
        "#,
            true,
        );

        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn complex_from_file() {
        //let a: serde_json::Value = serde_json::from_str(&fs::read_to_string("test/Complex.econ").unwrap()).expect("JSON was not well-formatted");
        //println!("{}", a);
        assert_eq!(
            true,
            matches!(Econ::create("test/Complex.econ", true), Ok(_))
        );
    }

    #[test]
    fn large_from_file() {
        //let a: serde_json::Value = serde_json::from_str(&fs::read_to_string("test/large-file.json").unwrap()).expect("JSON was not well-formatted");
        //println!("{}", a);
        assert_eq!(
            true,
            matches!(Econ::create("test/large-file.json", false), Ok(_))
        );
    }

    #[test]
    fn functions() {
        let obj = Econ::create(
            r#"
        {
            aa: "Hello,how,are,you",
            a: to_string(map(chars($aa), x => $x == "," ? " " : $x))
        }
        "#,
            true,
        );

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
        "#,
            true,
        );

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
        "#,
        );
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
        "#,
            true,
        );

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
        "#,
            true,
        );

        assert_eq!(true, matches!(obj, Ok(_)));
    }

    #[test]
    fn econ_deserialize() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Point {
            x: f64,
            y: f64,
        }

        let p;
        let obj = Econ::from(
            r#"
        {
            x: 1+1,
            y: 2+5
        }
        "#,
        );
        p = Econ::to_struct::<Point>(&obj).unwrap();
        assert_eq!(Point { x: 2.0, y: 7.0 }, p);
    }
}
