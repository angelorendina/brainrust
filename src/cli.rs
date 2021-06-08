use crate::lang::VM;
use std::collections::VecDeque;
use std::env;
use std::io::Write;

/// Returned when CLI app fails.
pub enum CliError {
    Arguments(&'static str),
    File(std::io::Error),
    Syntax,
}

struct CliArgs {
    path: String,
    source_file: String,
    output_file: Option<String>,
    input_file: Option<String>,
    input_stream: Option<String>,
    print_screen: bool,
}

impl CliArgs {
    /// Parses given cli arguments into a static struct with convenience methods.
    fn parse() -> Result<CliArgs, CliError> {
        let mut cli_args = CliArgs {
            path: "".into(),
            source_file: "".into(),
            output_file: None,
            input_file: None,
            input_stream: None,
            print_screen: false,
        };
        let mut args: VecDeque<String> = env::args().collect();

        // Argument #0 is always provided by the OS, pointing to where the executable runs.
        // CLI params come from #1 etc.
        match args.pop_front() {
            Some(p) => {
                cli_args.path = p;
            }
            None => {
                return Err(CliError::Arguments("Not enough arguments."));
            }
        }

        // Returns with usage if no arguments are passed.
        if args.len() == 0 {
            let usage = include_str!("help");
            return Err(CliError::Arguments(usage));
        }

        // Returns with usage if '--help' or '-h' flag.
        for v in args.iter() {
            if v == "--help" || v == "-h" {
                let usage = include_str!("help");
                return Err(CliError::Arguments(usage));
            }
        }

        // Searches for output file flag, '--output' or '-o' then filename.
        for (i, v) in args.iter().enumerate() {
            if v == "--output" || v == "-o" {
                if i + 1 < args.len() {
                    cli_args.output_file = args.remove(i + 1);
                    args.remove(i);
                    break;
                } else {
                    return Err(CliError::Arguments("Missing output file name."));
                }
            }
        }

        // Searches for input file flag, '--input' or '-i' then filename.
        for (i, v) in args.iter().enumerate() {
            if v == "--input" || v == "-i" {
                if i + 1 < args.len() {
                    cli_args.input_file = args.remove(i + 1);
                    args.remove(i);
                    break;
                } else {
                    return Err(CliError::Arguments("Missing input file name."));
                }
            }
        }

        // Searches for input stream flag, '--stream' or '-s' then stream.
        for (i, v) in args.iter().enumerate() {
            if v == "--stream" || v == "-s" {
                if i + 1 < args.len() {
                    cli_args.input_stream = args.remove(i + 1);
                    args.remove(i);
                    break;
                } else {
                    return Err(CliError::Arguments("Missing stream."));
                }
            }
        }

        // Searches for print flag, '--print' or '-p'.
        for (i, v) in args.iter().enumerate() {
            if v == "--print" || v == "-p" {
                cli_args.print_screen = true;
                args.remove(i);
                break;
            }
        }

        // Reads last remaining argument as source file.
        if args.len() == 1 {
            cli_args.source_file = args.remove(0).unwrap();
        } else {
            return Err(CliError::Arguments("Missing source file name."));
        }

        // Returns error if output is not being printed or saved to file.
        if !cli_args.print_screen && cli_args.output_file == None {
            return Err(CliError::Arguments("No output file or print flag."));
        }

        // All good!
        return Ok(cli_args);
    }

    /// Reads source code from target file.
    fn load_source(&self) -> Result<String, std::io::Error> {
        match std::fs::read_to_string(&self.source_file) {
            Ok(src) => {
                return Ok(src);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    /// Returns input buffer. It consists of the stream then bytecode from input file.
    fn fetch_input(&self) -> Result<VecDeque<u8>, std::io::Error> {
        let mut input: VecDeque<u8> = VecDeque::new();
        if let Some(stream) = &self.input_stream {
            for c in stream.chars() {
                input.push_back(c as u8);
            }
        }
        if let Some(file) = &self.input_file {
            match std::fs::read(file) {
                Ok(bytes) => {
                    for b in bytes {
                        input.push_back(b);
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        return Ok(input);
    }

    /// Writes data to output file, if provided.
    fn flush_output(&self, data: &[u8]) -> Result<(), std::io::Error> {
        match &self.output_file {
            Some(file) => match std::fs::File::create(file) {
                Ok(mut stream) => match stream.write_all(data) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(err);
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            },
            None => {
                return Ok(());
            }
        }
    }
}

/// Parses CLI arguments and runs the required functionality.
/// Returns the output on success, or CliError otherwise.
pub fn execute() -> Result<Vec<u8>, CliError> {
    // Parses cli params.
    let args: CliArgs;
    match CliArgs::parse() {
        Ok(a) => {
            args = a;
        }
        Err(err) => {
            return Err(err);
        }
    }

    // Gets source code.
    let source: String;
    match args.load_source() {
        Ok(src) => {
            source = src;
        }
        Err(err) => {
            return Err(CliError::File(err));
        }
    }

    // Constructs a new VM from source.
    let mut vm: VM;
    match VM::construct(&source) {
        Ok(v) => {
            vm = v;
        }
        Err(_) => {
            return Err(CliError::Syntax);
        }
    }

    // Gets the input buffer.
    let mut input: VecDeque<u8>;
    match args.fetch_input() {
        Ok(b) => {
            input = b;
        }
        Err(err) => {
            return Err(CliError::File(err));
        }
    }

    // Runs the VM.
    let mut output: Vec<u8> = Vec::new();
    vm.run(&mut input, &mut output);

    // Prints output to screen, if requested.
    if args.print_screen {
        println!("{}", output.iter().map(|b| *b as char).collect::<String>());
    }

    // Writes output to file, if requested.
    match args.flush_output(&output) {
        Ok(_) => {}
        Err(err) => {
            return Err(CliError::File(err));
        }
    }

    // Done!
    return Ok(output);
}
