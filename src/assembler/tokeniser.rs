#[derive(Default,Debug)]
pub struct TokenisedLine {
    pub label: Option<String>,

    pub opcode: Option<String>,
    pub operand1: Option<String>,
    pub operand2: Option<String>,
    pub operand3: Option<String>,
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
                _ => (),
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
                _ => (), 
            }
        }
        return Some(tok_line);
    }
}
