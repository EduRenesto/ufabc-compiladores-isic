use std::io::Cursor;

use ariadne::{Source, Report, Label};
use isic_back::cemitter::CEmitter;
use isic_interpreter::interpreter::IsiInterpreter;
use isic_front::parser::isilang_parser;
use isic_middle::{typeck::TypeCk, usageck::UsageCk};
use serde::Serialize;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[derive(Serialize)]
pub struct CompileResult {
    output_code: Option<String>,
    errors: Vec<String>,
    warns: Vec<String>,
}

#[derive(Serialize)]
pub struct InterpretResult {
    output: String,
    errors: Vec<String>,
    warns: Vec<String>,
}

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn compile_to_c(input_text: &str) -> JsValue {
    let parse_result = isilang_parser::program(input_text);

    let mut result = CompileResult {
        output_code: None,
        errors: vec![],
        warns: vec![],
    };

    let mut reporter_src = Source::from(input_text);

    match parse_result {
        Ok(ast) => 'a: {
            let mut typeck = TypeCk::new(&ast);
            if let Err(errors) = typeck.check() {
                for desc in errors {
                    let offset = reporter_src.get_offset_line(desc.span.start).unwrap();

                    let mut report_bytes: Vec<u8> = vec![];

                    let _report = Report::build(ariadne::ReportKind::Error, (), offset.1)
                        .with_message("Type error")
                        .with_label(
                            Label::new(((), desc.span.start..desc.span.end))
                                .with_color(ariadne::Color::Red)
                                .with_message(desc.desc),
                        )
                        .finish()
                        .write(&mut reporter_src, &mut report_bytes)
                        .unwrap();

                    let report_str = String::from_utf8(report_bytes).unwrap();

                    result.errors.push(report_str);
                }

                break 'a;
            }

            let mut usageck = UsageCk::new(&ast);
            let warns = usageck.check();

            for desc in warns {
                let offset = reporter_src.get_offset_line(desc.span.start).unwrap();

                let mut report_bytes: Vec<u8> = vec![];

                let _report = Report::build(ariadne::ReportKind::Warning, (), offset.1)
                    .with_message("Usage pattern warning")
                    .with_label(
                        Label::new(((), desc.span.start..desc.span.end))
                            .with_color(ariadne::Color::Yellow)
                            .with_message(desc.desc),
                    )
                    .finish()
                    .write(&mut reporter_src, &mut report_bytes)
                    .unwrap();

                let report_str = String::from_utf8(report_bytes).unwrap();

                result.warns.push(report_str);
            }

            let mut output_bytes: Vec<u8> = vec![];

            let emitter = CEmitter::new(&ast, &typeck.sym_table, &mut output_bytes);
            emitter.emit().unwrap();

            let output_str = String::from_utf8(output_bytes).unwrap();

            result.output_code = Some(output_str);
        }
        Err(e) => {
            let offset = reporter_src.get_offset_line(e.location.offset).unwrap();

            let mut report_bytes: Vec<u8> = vec![];

            let _report = Report::build(ariadne::ReportKind::Error, (), offset.1)
                .with_message("Syntax error")
                .with_label(
                    Label::new(((), e.location.offset..e.location.offset+1))
                        .with_color(ariadne::Color::Red)
                        .with_message(format!("Expected {}", e.expected)),
                )
                .finish()
                .write(&mut reporter_src, &mut report_bytes)
                .unwrap();

            let report_str = String::from_utf8(report_bytes).unwrap();

            result.errors.push(report_str);
        }
    }

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn run_interpreter(code: &str, input_text: &str) -> JsValue {
    let parse_result = isilang_parser::program(code);

    let mut result = InterpretResult {
        output: String::new(),
        errors: vec![],
        warns: vec![],
    };

    let mut reporter_src = Source::from(code);

    match parse_result {
        Ok(ast) => 'a: {
            let mut typeck = TypeCk::new(&ast);
            if let Err(errors) = typeck.check() {
                for desc in errors {
                    let offset = reporter_src.get_offset_line(desc.span.start).unwrap();

                    let mut report_bytes: Vec<u8> = vec![];

                    let _report = Report::build(ariadne::ReportKind::Error, (), offset.1)
                        .with_message("Type error")
                        .with_label(
                            Label::new(((), desc.span.start..desc.span.end))
                                .with_color(ariadne::Color::Red)
                                .with_message(desc.desc),
                        )
                        .finish()
                        .write(&mut reporter_src, &mut report_bytes)
                        .unwrap();

                    let report_str = String::from_utf8(report_bytes).unwrap();

                    result.errors.push(report_str);
                }

                break 'a;
            }

            let mut usageck = UsageCk::new(&ast);
            let warns = usageck.check();

            for desc in warns {
                let offset = reporter_src.get_offset_line(desc.span.start).unwrap();

                let mut report_bytes: Vec<u8> = vec![];

                let _report = Report::build(ariadne::ReportKind::Warning, (), offset.1)
                    .with_message("Usage pattern warning")
                    .with_label(
                        Label::new(((), desc.span.start..desc.span.end))
                            .with_color(ariadne::Color::Yellow)
                            .with_message(desc.desc),
                    )
                    .finish()
                    .write(&mut reporter_src, &mut report_bytes)
                    .unwrap();

                let report_str = String::from_utf8(report_bytes).unwrap();

                result.warns.push(report_str);
            }

            let mut input_cursor = Cursor::new(input_text.to_string());
            let mut output_bytes: Vec<u8> = vec![];

            let mut interpreter = IsiInterpreter::new(
                &ast,
                &mut input_cursor,
                &mut output_bytes,
            );

            interpreter.exec();

            result.output = String::from_utf8(output_bytes).unwrap();
        }
        Err(e) => {
            let offset = reporter_src.get_offset_line(e.location.offset).unwrap();

            let mut report_bytes: Vec<u8> = vec![];

            let _report = Report::build(ariadne::ReportKind::Error, (), offset.1)
                .with_message("Syntax error")
                .with_label(
                    Label::new(((), e.location.offset..e.location.offset+1))
                        .with_color(ariadne::Color::Red)
                        .with_message(format!("Expected {}", e.expected)),
                )
                .finish()
                .write(&mut reporter_src, &mut report_bytes)
                .unwrap();

            let report_str = String::from_utf8(report_bytes).unwrap();

            result.errors.push(report_str);
        }
    }

    serde_wasm_bindgen::to_value(&result).unwrap()
}
