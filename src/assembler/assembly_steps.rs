use std::collections::HashMap;
use std::iter::Iterator;
use regex::Regex;
use synacor::WORD;
use synacor::opcode::Opcode;
use super::types::{Argument,Token,TokenType};

pub fn remove_comments(source: String) -> String {
    lazy_static! {
        static ref comment_rx: Regex = Regex::new(r"^\s*;.*$").unwrap();
    }
    source.lines()
    .filter(|l| !comment_rx.is_match(l)) // remove all comment-only lines
    .map(|l| l[0..l.find(';').unwrap_or(l.len())].trim())
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn split_to_lines<'l>(source: &'l String) -> Vec<&'l str> {
    source.lines().collect::<Vec<&'l str>>()
}

pub fn tokenize<'t>(source_lines: Vec<&'t str>) -> Vec<Token<'t>> {
    lazy_static! {
        static ref instruction_rx: Regex = Regex::new(r"(?x)
            ^\s*
            (?:(?P<label>[\w_]+):\s)?\s*
            (?P<opcode>halt|set|push|pop|eq|gt|jmp|jt|jf|add|mult|mod|and|or|not|rmem|wmem|call|ret|out|in|noop)
            (?:\s+(?P<a>\#\d+|r\d|\[r\d\]|[\w_]+))?
            (?:\s+(?P<b>\#\d+|r\d|\[r\d\]|[\w_]+))?
            (?:\s+(?P<c>\#\d+|r\d|\[r\d\]|[\w_]+))?
            $").unwrap();
        static ref declaration_rx: Regex = Regex::new(r#"(?x)
            ^\s*
            (?P<label>[a-z][\w_]+)
            \s+dw\s+
            "(?P<data>.*[^\\])"
            (?:,(?P<end_chars>\d+(?:,\d+)*))?
            $"#).unwrap();
        static ref label_only_rx: Regex = Regex::new(r"^\s*([a-z][\w_]+):\s*$").unwrap();
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut last_label: Option<&str> = None;

    for idx in 0..source_lines.len() {
        let l = source_lines[idx];

        if l == "" { continue; }

        if label_only_rx.is_match(l) {
            last_label = Some(label_only_rx.captures(l).unwrap().get(1).unwrap().as_str());
            continue;
        }
        
        if instruction_rx.is_match(l) {
            let caps = instruction_rx.captures(l).unwrap();

            let label = last_label.clone().or_else(|| caps.name("label").map(|l| l.as_str()));
            let opcode = Opcode::try_from(caps.name("opcode").expect("failed to parse opcode").as_str());
            let arg_a = caps.name("a").map(|a| Argument::from(a.as_str()));
            let arg_b = caps.name("b").map(|b| Argument::from(b.as_str()));
            let arg_c = caps.name("c").map(|c| Argument::from(c.as_str()));

            if last_label.is_some() { last_label = None; }

            let off = tokens.last().map(|t| t.offset + t.size()).unwrap_or(0);

            tokens.push(Token {
                tok_type: TokenType::Instruction,
                label: label,
                offset: off,
                opcode: opcode,
                args: [arg_a, arg_b, arg_c],
                data: vec![]
            });
        } else if declaration_rx.is_match(l) {
            if last_label.is_some() {
                // oh shit son
                println!("standalone label before data declaration is not supported");
                last_label = None;
            }

            let caps = declaration_rx.captures(l).unwrap();

            let label = caps.name("label").unwrap().as_str();
            let mut data = caps.name("data").unwrap().as_str().chars().map(|c| c as WORD).collect::<Vec<WORD>>();
            let end_chars = caps.name("end_chars").unwrap().as_str()
                .split(",")
                .map(|n| n.parse::<WORD>().unwrap())
                .collect::<Vec<WORD>>();

            data.extend(end_chars);

            let off = tokens.last().map(|t| t.offset + t.size()).unwrap_or(0);
            tokens.push(Token {
                tok_type: TokenType::DataDeclaration,
                label: Some(label),
                offset: off,
                opcode: None,
                args: [None; 3],
                data: data
            });
        } else {
            println!("Unable to parse line #{}: {}", idx+1, l);
            continue;
        }
    }

    tokens
}

pub fn resolve_labels(mut tokens: Vec<Token>) -> Vec<Token> {
    let label_map: HashMap<String,usize> = tokens.iter().filter(|t| t.label.is_some()).map(|t| (t.label.unwrap().to_string(), t.offset)).collect();
    tokens.iter_mut().for_each(|tok| {
        tok.args = [
            match tok.args[0] {
                Some(Argument::Label(l)) => Some(Argument::Number(*label_map.get(&l[..]).unwrap() as u16)),
                a => a,
            },
            match tok.args[1] {
                Some(Argument::Label(l)) => Some(Argument::Number(*label_map.get(&l[..]).unwrap() as u16)),
                a => a,
            },
            match tok.args[2] {
                Some(Argument::Label(l)) => Some(Argument::Number(*label_map.get(&l[..]).unwrap() as u16)),
                a => a,
            }
        ];
        tok.label = None;
    });
    tokens
}

pub fn convert_to_bytes(tokens: Vec<Token>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for tok in tokens {
        let words = tok.as_words();
        let tok_bytes = words.iter().flat_map(|w| vec![(w & 0xFF) as u8, (w >> 8) as u8]).collect::<Vec<u8>>();
        println!("{:?} => {:?}", tok, tok_bytes);
        bytes.extend(tok_bytes);
    }
    bytes
}