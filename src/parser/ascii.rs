const TAB_WIDTH: usize = 4;

enum TokenType {
    OpenBracket,
    CloseBracket,
    Data,
    Comma,
    Key,
}

pub struct Token {
    token_type: TokenType,
    contents: String,
    line: usize,
    column: usize,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect::<Vec<char>>();
    let mut result: Vec<Token> = vec![];
    let mut line: usize = 1;
    let mut column: usize = 1;
    let mut token_begin: Option<usize> = None;
    let mut token_end: Option<usize> = None;
    let mut comment = false;
    let mut in_double_quotes = false;
    let mut pending_data_token = false;
    let mut process_data_token = |token_type: TokenType, must_have_token: bool| {
        if (token_begin != None) && (token_end != None) {
            let mut in_double_quotes = false;
            cfor!{
                let mut cur = token_begin.unwrap();
                cur != token_end.unwrap() + 1;
                cur += 1;
                {
                    let c = chars[cur];
                    if c == '\"' {
                        in_double_quotes = !in_double_quotes;
                    }
                    if !in_double_quotes && is_space_or_new_line(c) {
                        panic!("unexpected whitespace in token");
                    }
                    if in_double_quotes {
                        panic!("non-terminated double quotes");
                    }
                    result.push(Token {
                        token_type: token_type,
                        contents: input[token_begin.unwrap()..token_end.unwrap()].to_string(),
                        line: line,
                        column: column,
                    });
                }
            }
        } else if (must_have_token) {
            panic!("unexpected character, expected data token");
        }
        token_begin = None;
        token_end = None;
    };
    cfor!{
        let mut cur: usize = 0;
        cur < chars.len();
        {column += if chars[cur] == '\t' {TAB_WIDTH} else {1}; cur += 1};
        {
            let c = chars[cur];
            if is_line_end(c) {
                comment = false;
                column = 0;
                line += 1;
            }
            if comment {
                continue;
            }
            if in_double_quotes {
                if c == '\"' {
                    in_double_quotes = false;
                    pending_data_token = false;
                    token_end = Some(cur);
                    process_data_token(TokenType::Data, false);
                }
                continue;
            }
            match c {
                '\"' => {
                    if token_begin == None {
                        panic!("unexpected double-quote");
                    }
                    token_begin = Some(cur);
                    in_double_quotes = true;
                    continue;
                },
                ';' => {
                    process_data_token(TokenType::Data, false);
                    comment = true;
                    continue;
                },
                '{' => {
                    process_data_token(TokenType::Data, false);
                    result.push(Token {
                        token_type: TokenType::OpenBracket,
                        contents: input[cur..(cur + 1)].to_string(),
                        line: line,
                        column: column,
                    });
                    continue;
                },
                '}' => {
                    process_data_token(TokenType::Data, false);
                    result.push(Token {
                        token_type: TokenType::CloseBracket,
                        contents: input[cur..(cur + 1)].to_string(),
                        line: line,
                        column: column,
                    });
                    continue;
                },
                ',' => {
                    if pending_data_token {
                        process_data_token(TokenType::Data, true);
                    }
                    result.push(Token {
                        token_type: TokenType::Comma,
                        contents: input[cur..(cur + 1)].to_string(),
                        line: line,
                        column: column,
                    });
                    continue;
                },
                ':' => {
                    if pending_data_token {
                        process_data_token(TokenType::Data, true);
                    } else {
                        panic!("unexpected colon");
                    }
                    continue;
                },
                _ => {
                    if is_space_or_new_line(c) {
                        if token_begin != None {
                            let mut token_type = TokenType::Data;
                            cfor!{
                                let mut peek = cur;
                                is_space_or_new_line(chars[peek]);
                                peek += 1;
                                {
                                    if chars[peek] == ':' {
                                        token_type = TokenType::Key;
                                        cur = peek;
                                        break;
                                    }
                                }
                            }
                            process_data_token(token_type, false);
                        }
                        pending_data_token = false;
                    } else {
                        token_end = Some(cur);
                        if token_begin == None {
                            token_begin = Some(cur);
                        }
                        pending_data_token = true;
                    }
                },
            }
            println!("{}, {}", cur, c);
        }
    }
    return result;
}

fn is_space(c: char) -> bool {
    match c {
        ' ' | '\t' => true,
        _ => false,
    }
}

fn is_line_end(c: char) -> bool {
    match c {
        '\r' | '\n' | '\0' | '\x0c' => true,
        _ => false,
    }
}

fn is_space_or_new_line(c: char) -> bool {
    is_space(c) || is_line_end(c)
}
