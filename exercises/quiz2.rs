// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    use std::borrow::Cow;

    // TODO: Complete the function signature!
    // pub fn transformer(input: Vec<(Cow<Str>, Command)>) -> Vec<Cow<str>> {
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<Cow<str>> {
        // TODO: Complete the output declaration!
        let mut output: Vec<Cow<str>> = vec![];

        // for (string, command) in input.iter() {
        //     // TODO: Complete the function body. You can do it!
        //     match command {
        //         Command::Uppercase => {
        //             output.push(Cow::Owned(string.to_uppercase()));
        //         },
        //         Command::Trim => {
        //             output.push(Cow::Borrowed(string.trim()));
        //         },
        //         Command::Append(size) => {
        //             let mut s = string.to_string();
        //             for _ in 0..*size {
        //                 s.push_str("bar");
        //             }
        //             output.push(s.into());
        //         }
        //     }
        // }

        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match &&&&&&command {
                Command::Uppercase => {
                    output.push(Cow::Owned(string.to_uppercase()));
                },
                Command::Trim => {
                    output.push(Cow::Borrowed(string.trim()));
                },
                Command::Append(size) => {
                    let mut s = string.to_string();
                    for _ in 0..*size {
                        s.push_str("bar");
                    }
                    output.push(s.into());
                }
            }
        }

        // for (string, command) in input.iter() {
        //     // TODO: Complete the function body. You can do it!
        //     match *command {
        //         Command::Uppercase => {
        //             output.push(Cow::Owned(string.to_uppercase()));
        //         },
        //         Command::Trim => {
        //             output.push(Cow::Borrowed(string.trim()));
        //         },
        //         Command::Append(size) => {
        //             let mut s = string.to_string();
        //             for _ in 0..size {
        //                 s.push_str("bar");
        //             }
        //             output.push(s.into());
        //         }
        //     }
        // }

        // for (string, ref command) in input.iter() {
        //     // TODO: Complete the function body. You can do it!
        //     match command {
        //         Command::Uppercase => {
        //             output.push(Cow::Owned(string.to_uppercase()));
        //         },
        //         Command::Trim => {
        //             output.push(Cow::Borrowed(string.trim()));
        //         },
        //         Command::Append(size) => {
        //             let mut s = string.to_string();
        //             for _ in 0..*size {
        //                 s.push_str("bar");
        //             }
        //             output.push(s.into());
        //         }
        //     }
        // }


        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
