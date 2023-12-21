mod parser;
mod propositional;
mod verify;
//mod test_helpers;

use annotate_snippets::{Annotation, AnnotationType, Renderer, Slice, Snippet, SourceAnnotation};
use chumsky::error::SimpleReason;
use chumsky::Parser;

use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file_path = "stdin";
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        match crate::parser::parser().parse(line.clone()) {
            Ok(claim) => {
                println!("{:#?}", claim);
            }
            Err(parse_errs) => parse_errs.into_iter().for_each(|e| {
                let (start, end) = (e.span().start, e.span().end);
                let line_start = line.chars().take(start).filter(|c| c == &'\n').count() + 1;
                let (title, label) = {
                    match e.reason() {
                        SimpleReason::Unexpected => match e.expected().len() {
                            0 => ("Unexpected".to_string(), "Unexpected".to_string()),
                            _ => {
                                let expected = format!(
                                    "expected one of {}",
                                    e.expected()
                                        .map_while(|exp| {
                                            match exp {
                                                Some(c) => Some(format!("'{}'", c.to_string())),
                                                None => None,
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                );
                                let found = format!("");
                                (format!("{}, {}", expected, found), expected)
                            }
                        },
                        SimpleReason::Custom(reason) => (reason.to_string(), reason.to_string()),
                        _ => {
                            unreachable!("current implmentation doesn't use 'unclosed_delimiter()'")
                        }
                    }
                };
                let snippet = Snippet {
                    title: Some(Annotation {
                        label: Some(&title),
                        id: None,
                        annotation_type: AnnotationType::Error,
                    }),
                    footer: vec![Annotation {
                        label: Some("hey gavin tell me what you think about the parser!"),
                        id: None,
                        annotation_type: AnnotationType::Note,
                    }],
                    slices: vec![Slice {
                        source: &line,
                        line_start,
                        origin: Some(file_path),
                        fold: false,
                        annotations: vec![SourceAnnotation {
                            label: &label,
                            range: (start, end),
                            annotation_type: AnnotationType::Error,
                        }],
                    }],
                };
                let renderer = Renderer::styled();
                println!("{}", renderer.render(snippet));
            }),
        }
    }
    Ok(())
}
