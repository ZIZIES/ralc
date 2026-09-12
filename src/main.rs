use std::env;
use std::process;

fn usage(program_name: &str) -> ! {
    println!("usage: {} <number 1> <operation> <number 2>\noptional: --dev-null", program_name);
    process::exit(1);
}

fn main() {
    print!("\n");
    // get args
    let args: Vec<String> = env::args().collect();
    // constants
    let program_name = &args[0];

    // --dev-null flag + length checks (MUST happen before indexing args[1..3])
    let mut dev_null: bool = false;
    if args.len() == 5 && args[4] == "--dev-null" {
        dev_null = true;
    } else if args.len() != 4 {
        usage(&program_name);
    }

    let first_number: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("bad number: {}", args[1]);
            process::exit(1);
        }
    };
    let operation = args[2].clone();
    let second_number: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("bad number: {}", args[3]);
            process::exit(1);
        }
    };

    // divide by zero check
    if operation == "/" && second_number == 0.0 {
        println!("cant divide by zero!");
        process::exit(1);
    }

    // does the actual operation
    let result = match operation.as_str() {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" | "x" => first_number * second_number,
        "/" => first_number / second_number,
        _ => usage(&program_name),
    };

    // --dev-null flag
    if dev_null {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut devnull = OpenOptions::new().write(true).open("/dev/null").unwrap();
        writeln!(devnull, "{}", result).unwrap();
    } /* normally print */ else {
        print!("{}\n\n", result);
    }
    //end main
}