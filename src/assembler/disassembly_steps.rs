use synacor::WORD;
use synacor::opcode::Opcode;
use super::types::{Argument,Token,TokenType};

#[derive(Debug,PartialEq)]
enum DisassemblyMode {
    Data,
    Instruction
}

pub fn convert_to_words(bytes: Vec<u8>) -> Vec<WORD> {
    let mut words: Vec<WORD> = Vec::new();
    for c in bytes.chunks(2) {
        words.push(((c[1] as WORD) << 8) + c[0] as WORD);
    }
    words
}

pub fn convert_to_tokens<'t>(words: Vec<WORD>) -> Vec<Token<'t>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut mode = DisassemblyMode::Instruction;
    let mut idx = 0;
    let mut tok: Option<Token> = None;

    loop {
        let word = words[idx];
        if mode == DisassemblyMode::Data {
            let mut t = tok.unwrap();
            if word == 0 {
                    t.data.push(word);
                    tokens.push(t);
                    tok = None;
                    mode = DisassemblyMode::Instruction;
            } else {
                t.data.push(word);
                tok = Some(t);
            }
        } else {
            // max opcode is 21, for Noop
            if word > 21 {
                if tok.is_some() {
                    panic!("Attempted to begin data declaration while parsing token, or failed parsing token");
                }
                let mut t = Token::new_data();
                t.data.push(word);
                tok = Some(t);
            } else {
                let mut t = Token::new_instr();
                let opcode: Opcode = word.into();
                for off in 0..opcode.argc() {
                    t.args[off] = match words[idx + off] {
                        n if n <  0x8000               => Some(Argument::Number(n)),
                        n if n >= 0x8000 && n < 0x8008 => Some(Argument::Register((n - 0x8000) as usize)),
                        n                              => panic!("Invalid number: {}", n),
                    }
                }
                idx += opcode.argc();
                t.opcode = Some(opcode);
                tokens.push(t);
            }
        }
        idx += 1;
    }

    panic!("Decoded to tokens:\n{:?}", tokens);

    tokens
}

pub fn convert_to_instructions<'t>(tokens: Vec<Token<'t>>) -> Vec<String> {
    let lines: Vec<String> = Vec::new();

    for _t in tokens {}

    // for t in tokens {
    //     if t.tok_type == TokenType::DataDeclaration {
    //         let mut in_str = true;
    //         let mut data_literal: String = "\"";

    //         for ascii_char in t.data.iter().map(|c) {
    //             match ascii_char {
    //                 c if c < 0x20 => {
    //                     if in_str { data_literal += "\","; in_str = false; }
    //                     data_literal += c.to_string() + ",";
    //                 },
    //                 c if c >= 0x20 && c < 0x80 {
    //                     if !in_str { data_literal += "\""; in_str = true; }

    //                 }
    //                 0 => { // null terminator
                        
    //                 },
    //                 9 => { //tab character
    //                     if in_str { data_literal += '"'; in_str = false; }
    //                     data_literal += '9';
    //                 },
    //                 10 => { // newline
    //                     if in_str { data_literal += '"'; in_str = false; }
    //                     data_literal += "10";
    //                 },
    //                 13 => { // carraige return, windows-only
    //                     if in_str { data_literal += '"'; in_str = false; }
    //                     data_literal += "13";
    //                 },
    //                 c => {
    //                     if !in_str { data_literal += ",\""; in_str = true; }
    //                     data_literal += c;
    //                 }
    //             }
    //         }
    //         lines.push(format!("{} dw {}", t.label, data_literal));
    //     } else {

    //     }
    // }
    
    lines
}