use std::io::Read;
use std::{error::Error, fs::File, path::PathBuf};

use ariadne::{Label, Report, Source};
use clap::Parser;
use isic_back::cemitter::CEmitter;
use isic_middle::typeck::TypeCk;
use isic_middle::usageck::UsageCk;

#[derive(Parser)]
#[command()]
struct CliArgs {
    #[arg(short = 'i', long = "input")]
    /// O arquivo de entrada.
    pub input_file: PathBuf,

    #[arg(short = 'o', long = "output")]
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

    let mut input = File::open(&args.input_file)?;
    let mut output = File::create(&args.get_output_file())?;

    let mut input_text = String::new();
    input.read_to_string(&mut input_text)?;

    let parse_result = isic_front::parser::isilang_parser::program(&input_text);

    let mut reporter_src = Source::from(&input_text);

    match parse_result {
        Ok(ast) => 'a: {
            let mut typeck = TypeCk::new(&ast);
            if let Err(errors) = typeck.check() {
                //let report = Report::build(ariadne::ReportKind::Error, args.input_file, offset);
                for desc in errors {
                    let offset = reporter_src.get_offset_line(desc.span.start).unwrap();

                    let _report = Report::build(ariadne::ReportKind::Error, (), offset.1)
                        .with_message("Type error")
                        .with_label(
                            Label::new(((), desc.span.start..desc.span.end))
                                .with_color(ariadne::Color::Red)
                                .with_message(desc.desc),
                        )
                        .finish()
                        .print(&mut reporter_src)
                        .unwrap();
                }

                break 'a;
            }

            let mut usageck = UsageCk::new(&ast);
            let warns = usageck.check();

            for desc in warns {
                let offset = reporter_src.get_offset_line(desc.span.start).unwrap();

                let _report = Report::build(ariadne::ReportKind::Warning, (), offset.1)
                    .with_message("Usage pattern warning")
                    .with_label(
                        Label::new(((), desc.span.start..desc.span.end))
                            .with_color(ariadne::Color::Yellow)
                            .with_message(desc.desc),
                    )
                    .finish()
                    .print(&mut reporter_src)
                    .unwrap();
            }

            let emitter = CEmitter::new(&ast, &typeck.sym_table, &mut output);
            emitter.emit().unwrap();
        }
        Err(e) => {
            // XXX
            //let err = chic::Error::new("parse error")
            //    .error(
            //        1,
            //        e.location.offset + e.location.line - 1,
            //        e.location.offset + e.location.line,
            //        &input_text,
            //        format!("expected: {}", e.expected)
            //    )
            //    .to_string();
            let offset = reporter_src.get_offset_line(e.location.offset).unwrap();

            let _report = Report::build(ariadne::ReportKind::Error, (), offset.1)
                .with_message("Syntax error")
                .with_label(
                    Label::new(((), e.location.offset..e.location.offset+1))
                        .with_color(ariadne::Color::Red)
                        .with_message(format!("Expected {}", e.expected)),
                )
                .finish()
                .print(&mut reporter_src)
                .unwrap();
        }
    }

    Ok(())
}
