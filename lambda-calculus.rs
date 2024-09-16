fn main() {
    use lambda_parser::*;
    use lambda_eval::*;

    let table = parse(r#"
succ = n -> s -> z -> s (n s z);
add = n -> n succ;
mul = m -> n -> m (add n) 0;
main = mul 100 100;
    "#).unwrap();
    eprintln!("{:?}", table);

    let value = new_apply(new_apply(table["main"].clone(), new_nat_succ()), new_nat(0));
    eprintln!("{:?}", value.eval())
}

pub mod lambda_parser {
    use super::lambda_eval::*;
    use std::rc::Rc;
    use std::collections::HashMap;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub fn parse(source: &str) -> Result<HashMap<String, Rc<Term>>> {
        let tokens = tokenize(source)?;
        let (table, len) = parse_program(&tokens)?;
        if len < tokens.len() {
            return Err(format!("unexpected token {:?}", tokens[len]).into());
        }
        Ok(table)
    }

    #[derive(Clone, Debug)]
    enum Token {
        Name(String),
        Number(usize),
        Symbol(char),
        SymbolArrow,
    }
    fn tokenize(source: &str) -> Result<Vec<Token>> {
        let mut tokens = vec![];
        let mut temp_name = String::new();
        let mut temp_number = 0;
        enum Mode { Begin, Number, Ident, Arrow }
        let mut mode = Mode::Begin;
        let mut line = 1;
        let mut column = 0;
        for c in source.chars().chain(['$'].into_iter()) {
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
            match mode {
                Mode::Begin => {
                    match c {
                        '_' | 'A' ..= 'Z' | 'a' ..= 'z' => {
                            mode = Mode::Ident;
                            temp_name.clear();
                            temp_name.push(c);
                        }
                        '0' ..= '9' => {
                            mode = Mode::Number;
                            temp_number = c.to_digit(10).unwrap() as usize;
                        }
                        '=' | ';' | '(' | ')' => {
                            tokens.push(Token::Symbol(c));
                        }
                        '-' => {
                            mode = Mode::Arrow;
                        }
                        _ => {}
                    }
                }
                Mode::Number => {
                    match c {
                        '0' ..= '9' => {
                            temp_number = temp_number * 10 + c.to_digit(10).unwrap() as usize;
                        }
                        '_' | 'A' ..= 'Z' | 'a' ..= 'z' => {
                            tokens.push(Token::Number(temp_number));
                            mode = Mode::Ident;
                            temp_name = String::new();
                            temp_name.push(c);
                        }
                        '=' | ';' | '(' | ')' => {
                            tokens.push(Token::Number(temp_number));
                            mode = Mode::Begin;
                            tokens.push(Token::Symbol(c));
                        }
                        '-' => {
                            tokens.push(Token::Number(temp_number));
                            mode = Mode::Arrow;
                        }
                        _ => {
                            tokens.push(Token::Number(temp_number));
                            mode = Mode::Begin;
                        }
                    }
                }
                Mode::Ident => {
                    match c {
                        '_' | '0' ..= '9' | 'A' ..= 'Z' | 'a' ..= 'z' => {
                            temp_name.push(c);
                        }
                        '=' | ';' | '(' | ')' => {
                            tokens.push(Token::Name(temp_name.clone()));
                            mode = Mode::Begin;
                            tokens.push(Token::Symbol(c));
                        }
                        '-' => {
                            tokens.push(Token::Name(temp_name.clone()));
                            mode = Mode::Arrow;
                        }
                        _ => {
                            tokens.push(Token::Name(temp_name.clone()));
                            mode = Mode::Begin;
                        }
                    }
                }
                Mode::Arrow => {
                    match c {
                        '>' => {
                            tokens.push(Token::SymbolArrow);
                            mode = Mode::Begin;
                        }
                        '$' => {
                            return Err(format!("unexpected end of file").into());
                        }
                        _ => {
                            return Err(format!("unexpected char {c} at {line}:{column}").into());
                        }
                    }
                }
            }
        }
        Ok(tokens)
    }

    // program = entry*
    fn parse_program(tokens: &[Token]) -> Result<(HashMap<String, Rc<Term>>, usize)> {
        let mut table = HashMap::new();
        let mut offset = 0;
        while let Some(((name, value), skip)) = parse_entry(&tokens[offset ..], &table) {
            offset += skip;
            table.insert(name, value);
        }
        Ok((table, offset))
    }

    // entry = name "=" expr ";"
    fn parse_entry(tokens: &[Token], globals: &HashMap<String, Rc<Term>>) -> Option<((String, Rc<Term>), usize)> {
        let mut offset = 0;
        let Some(&Token::Name(ref name)) = tokens.get(offset) else { return None };
        offset += 1;
        let Token::Symbol('=') = tokens[offset] else { return None };
        offset += 1;
        let Some((value, skip)) = parse_expr(&tokens[offset ..], globals, &HashMap::new(), 0) else { return None };
        offset += skip;
        let Token::Symbol(';') = tokens[offset] else { return None };
        offset += 1;
        Some(((name.to_string(), value), offset))
    }

    // expr = (ident "->")* primary+
    fn parse_expr(tokens: &[Token], globals: &HashMap<String, Rc<Term>>, locals: &HashMap<String, usize>, level: usize) -> Option<(Rc<Term>, usize)> {
        // parse args
        let mut offset = 0;
        let mut args = vec![];
        loop {
            let Token::Name(arg) = &tokens[offset] else { break };
            let Token::SymbolArrow = &tokens[offset + 1] else { break };
            offset += 2;
            args.push(arg.to_string());
        }

        // set args to locals_next
        let level_next = level + args.len();
        let mut locals_temp;
        let locals_next = {
            if level_next == level {
                locals
            } else {
                locals_temp = locals.clone();
                for (i, arg) in args.into_iter().enumerate() {
                    locals_temp.insert(arg, level + i);
                }
                &locals_temp
            }
        };

        // parse applies
        let mut primaries = vec![];
        while let Some((primary, skip)) = parse_primary(&tokens[offset ..], globals, &locals_next, level_next) {
            primaries.push(primary);
            offset += skip;
        }
        let mut value = primaries.into_iter().reduce(new_apply)?;

        for _ in level .. level_next {
            value = new_lambda(value);
        }

        Some((value, offset))
    }

    // primary = "(" expr ")" / name / nat
    fn parse_primary(tokens: &[Token], globals: &HashMap<String, Rc<Term>>, locals: &HashMap<String, usize>, level: usize) -> Option<(Rc<Term>, usize)> {
        if let &Token::Symbol('(') = &tokens[0] {
            let mut offset = 1;
            let Some((value, skip)) = parse_expr(&tokens[offset ..], globals, locals, level) else { return None };
            offset += skip;
            let Token::Symbol(')') = tokens[offset] else { return None };
            offset += 1;
            Some((value, offset))
        } else if let &Token::Name(ref name) = &tokens[0] {
            if let Some(&ref_level) = locals.get(name) {
                Some((new_ref(level - ref_level - 1), 1))
            } else if let Some(value) = globals.get(name) {
                Some((value.clone(), 1))
            } else {
                panic!("'{name}' is not defined")
            }
        } else if let &Token::Number(n) = &tokens[0] {
            Some((new_nat(n), 1))
        } else {
            None
        }
    }
}

pub mod lambda_eval {
    use std::rc::Rc;
    
    pub fn new_ref(index: usize) -> Rc<Term> { Rc::new(Term::Ref(index)) }
    pub fn new_lambda(body: Rc<Term>) -> Rc<Term> { Rc::new(Term::Lambda(body)) }
    pub fn new_apply(f: Rc<Term>, x: Rc<Term>) -> Rc<Term> { Rc::new(Term::Apply(f, x)) }
    pub fn new_repeat(n: usize, f: Rc<Term>) -> Rc<Term> { Rc::new(Term::Repeat(n, f)) }
    pub fn new_nat(n: usize) -> Rc<Term> { Rc::new(Term::Nat(n)) }
    pub fn new_nat_succ() -> Rc<Term> { Rc::new(Term::NatSucc) }

    #[derive(Clone, Debug)]
    pub enum Term {
        Ref(usize),
        Lambda(Rc<Term>),
        Apply(Rc<Term>, Rc<Term>),
        Repeat(usize, Rc<Term>),
        Nat(usize),
        NatSucc,
    }
    impl Term {
        // 最左最外簡約を行う
        pub fn eval(self: &Rc<Term>) -> Rc<Term> {
            let mut value = self.clone();
            while let Some(next_value) = value.eval_step() {
                value = next_value;
            }
            value
        }

        fn eval_step(self: &Rc<Self>) -> Option<Rc<Self>> {
            use Term::*;
            use std::ops::Deref;
            match self.deref() {
                Apply(f, x) => f.apply(x),
                _ => None,
            }
        }

        fn apply(self: &Rc<Self>, param: &Rc<Self>) -> Option<Rc<Self>> {
            use Term::*;
            use std::ops::Deref;
            match self.deref() {
                &Ref(_) => panic!("unexpected ref"),
                &Lambda(ref body) => Some(body.subst(0, param).unwrap_or_else( || body.clone() )),
                &Apply(ref f, ref x) => {
                    let f = f.eval();
                    let g = f.apply(x)?;
                    g.apply(param)
                }
                &Repeat(n, ref f) => {
                    let mut x = param.clone();
                    for _ in 0 .. n {
                        x = f.apply(&x)?;
                    }
                    Some(x)
                }
                &NatSucc => {
                    if let &Nat(n) = param.eval().deref() {
                        Some(new_nat(n + 1))
                    } else {
                        panic!("tried to apply NatSucc to non-Nat term")
                    }
                }
                &Nat(n) => Some(new_repeat(n, param.clone())),
            }
        }

        fn subst(self: &Rc<Self>, key_index: usize, replacement: &Rc<Self>) -> Option<Rc<Self>> {
            use Term::*;
            use std::ops::Deref;
            match self.deref() {
                &Ref(index) => {
                    if index == key_index {
                        Some(replacement.clone())
                    } else {
                        None
                    }
                }
                &Lambda(ref body) => Some(new_lambda(body.subst(key_index + 1, replacement)?)),
                &Apply(ref f, ref x) => {
                    let f_ = f.subst(key_index, replacement);
                    let x_ = x.subst(key_index, replacement);
                    if f_.is_some() || x_.is_some() {
                        Some(new_apply(f_.unwrap_or_else(|| f.clone() ), x_.unwrap_or_else(|| x.clone() )))
                    } else {
                        None
                    }
                }
                &Repeat(n, ref f) => Some(new_repeat(n, f.subst(key_index, replacement)?)),
                _ => None
            }
        }
    }
}
