use std::io::Read;
use std::{path::PathBuf, error::Error, fs::File};

use clap::Parser;
use isic_back::cemitter::CEmitter;
use isic_middle::typeck::TypeCk;

#[derive(Parser)]
#[command()]
struct CliArgs {
    #[arg(short='i', long="input")]
    /// O arquivo de entrada.
    pub input_file: PathBuf,

    #[arg(short='o', long="output")]
    /// O arquivo de saída. Padrão: <arquivo de entrada>.c
    output_file: Option<PathBuf>,
}

impl CliArgs {
    pub fn get_output_file(&self) -> PathBuf {
        match &self.output_file {
            Some(f) => f.to_owned(),
            None => {
                let mut f = self.input_file.clone();
                f.set_extension("c");

                f
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let mut input  = File::open(&args.input_file)?;
    let mut output = File::create(&args.get_output_file())?;

    let mut input_text = String::new();
    input.read_to_string(&mut input_text)?;

    let parse_result = isic_front::parser::isilang_parser::program(&input_text);

    match parse_result {
        Ok(ast) => {
            //let emitter = CEmitter::new(&ast, &mut output);

            //emitter.emit().unwrap();

            let mut checker = TypeCk::new(&ast);

            checker.check().unwrap();
        },
        Err(e) => {
            // TODO: add expected tokens
            let err = chic::Error::new("parse error")
                .error(
                    1,
                    e.location.offset + e.location.line - 1,
                    e.location.offset + e.location.line,
                    &input_text,
                    format!("expected: {}", e.expected)
                )
                .to_string();

            eprintln!("{}", err);
            eprintln!("{:?}", e);
        },
    }

    Ok(())
}
