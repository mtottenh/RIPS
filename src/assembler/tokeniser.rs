#[derive(Default,Debug)]
pub struct TokenisedLine {
    label: Option<String>,

    opcode: Option<String>,
    operand1: Option<String>,
    operand2: Option<String>,
    operand3: Option<String>,
}

pub fn tokenise_line (line: String) -> Option<TokenisedLine> {

    let mut tokens : Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 1 {
        return None;
    }
    if tokens[0].contains(':') {
        let mut tok_line = TokenisedLine { label : Some( tokens[0].to_string() ),
        ..Default::default() };
        for t in 1..tokens.len() {
            match t {
                1 => tok_line.opcode = Some(tokens[t].to_string()),
                2 => tok_line.operand1 = Some(tokens[t].to_string()),
                3 => tok_line.operand2 = Some(tokens[t].to_string()),
                4 => tok_line.operand3 = Some(tokens[t].to_string()),
                _ => println!("Garbage input detected {} ", tokens[t]),
            }
        }
        return Some(tok_line);
    } else {
        let mut tok_line = TokenisedLine { opcode : Some(tokens[0].to_string()),
        ..Default::default()};
        for t in 1..tokens.len() {
            match t {
                1 => tok_line.operand1 = Some(tokens[t].to_string()),
                2 => tok_line.operand2 = Some(tokens[t].to_string()),
                3 => tok_line.operand3 = Some(tokens[t].to_string()),
                _ => println!("Garbage input detected {} ", tokens[t]),
            }
        }
        return Some(tok_line);
    }
}
