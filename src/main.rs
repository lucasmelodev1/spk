use std::ops;
use std::env;

/// Backslash command (like \n) ranges.
/// 
/// Each field represents the command name
struct BackslashCommandRanges {
    n: Vec<ops::Range<usize>>,
    r: Vec<ops::Range<usize>>,
    t: Vec<ops::Range<usize>>,
    back_slash: Vec<ops::Range<usize>>,
}

impl BackslashCommandRanges {
    fn new() -> BackslashCommandRanges {
        BackslashCommandRanges {
            n: Vec::new(),
            r: Vec::new(),
            t: Vec::new(),
            back_slash: Vec::new(),
        }
    }
}

struct Flags {
    enable_backslash_commands: bool,
}

impl Flags {
    fn new() -> Flags {
        return Flags { 
            enable_backslash_commands: false
        };
    }
}

fn backslash_ranges(str: &str) -> BackslashCommandRanges {
    let bytes = str.as_bytes();
    let mut commands = BackslashCommandRanges::new();
    for (index, char) in bytes.iter().enumerate() {
        if char == &b'\\' {
            match bytes[index+1] {
                b'n' => commands.n.push(index..index+1),
                b'r' => commands.r.push(index..index+1),
                b't' => commands.t.push(index..index+1),
                b'\\' => commands.back_slash.push(index..index+1),
                _ => (),
            };
        }
    };

    commands
}

/// Replace string backslash commands characters to
/// their specific commands.
/// 
/// Example: Replace the string "\\n" to the "\n"
/// command, which breaks the current line.
fn replace_commands(str: &mut String, ranges: BackslashCommandRanges) {
    for range in ranges.n {
        str.replace_range(range, "\n");
    }
    for range in ranges.r {
        str.replace_range(range, "\r");
    }
    for range in ranges.t {
        str.replace_range(range, "\t");
    }
    for range in ranges.back_slash {
        str.replace_range(range, "\\")
    }
}

/// Returns a `Flags` instance and the index which the argument list ends
/// 
/// If there are only flags in the arguments, it returns None as index
fn flags(args: &Vec<String>) -> (Flags, Option<usize>) {
    let mut arguments = Flags::new();
    for (index, str) in args.iter().enumerate() {
        let str = str.as_bytes();

        if !(str[0] == b'-') {
            return (arguments, Some(index))
        }

        match str[1..] {
            [b'e'] => arguments.enable_backslash_commands = true,
            _ => (),
        }
    };

    (arguments, None)
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    let (flags, args_start) = flags(&args);
    if flags.enable_backslash_commands {
        for arg in &mut args {
            replace_commands(arg, backslash_ranges(arg));
        }
    }
    
    match args_start {
        Some(start) => {
            for arg in &args[start..] {
                print!("{arg} ")
            }
        },
        None => (),
    }

    println!();
}
