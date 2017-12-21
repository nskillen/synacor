use std::fs::File;
use std::io::prelude::*;

mod assembly_steps;
mod disassembly_steps;
mod types;

pub fn assemble(source: String, dest_filename: &str) {
    let source_len = source.len();
    let source_without_comments = assembly_steps::remove_comments(source);
    let source_lines            = assembly_steps::split_to_lines(&source_without_comments);
    let tokens                  = assembly_steps::tokenize(source_lines);
    let tokens                  = assembly_steps::resolve_labels(tokens);
    let bytes                   = assembly_steps::convert_to_bytes(tokens);

    for c in bytes.chunks(20) {
        c.chunks(2).for_each(|cc| print!("{:#04X}{:02X} ", cc[0], cc[1]));
        println!("");
    }

    let mut f = File::create(dest_filename).unwrap();
    f.write_all(&bytes[..]);
    println!("Assembled {} bytes of source to {} bytes of binary", source_len, bytes.len());
}

pub fn disassemble(bin: Vec<u8>, dest_filename: &str) {
    let words        = disassembly_steps::convert_to_words(bin);
    let tokens       = disassembly_steps::convert_to_tokens(words);
    let source_lines = disassembly_steps::convert_to_instructions(tokens);
    let source_str   = source_lines.into_iter().collect::<String>();

    let mut f = File::create(dest_filename).unwrap();
    write!(f, "{}", source_str);
}

/*
; this is a sample program that can be used to test the assembler
; it does nothing more than printing a "Hello, World" string, followed
; by a newline

jmp start                   ; create a region for defining data

text dw "Hello, World",10,0 ; the 10 is newline

print_word:
    push r2                 ; will use r2, so preserve value
    rmem r2 [r1]            ; address of next char of string
    add  r1 r1 1            ; increment r1 to point to next char
    jf   r2 print_word_done ; null-terminator found, we're done
    out  r2                 ; print char in r2 to screen
    jmp  print_word         ; loop back to print next char
print_word_done:
    pop r2                  ; restore r2
    ret                     ; return

start:
    set r1, text            ; set r1 to the beginning of the string
    call print_word         ; print the string
    halt                    ; halt the CPU
*/