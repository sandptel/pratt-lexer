use std::fmt;

#[derive(Debug,Clone, Copy)]
enum Token
{
    Atom(char),
    Op(char),
    Eof
}

#[derive(Debug)]
struct Lexer
{
    tokens: Vec<Token>
}

//tokenizer 
impl Lexer
{
    pub fn new(input:&str) -> Lexer
    {
        let tokens = input.chars().filter(|it| !it.is_ascii_whitespace()).map(|x|{
            match x{
                '0'..='9'| 'a'..='z'| 'A'..='Z' => Token::Atom(x),
                _ => Token::Op(x)
            }
        });
        let tokens= tokens.collect();

        Lexer{tokens}
    }

    pub fn next(&mut self)  -> Token 
    {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&mut self)-> Token
    {
        *self.tokens.last().unwrap_or(&Token::Eof)
    }
}


fn expr(input:&str) -> S
{
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer)
}

fn expr_bp(lexer:&mut Lexer) -> S
{
let lhs = match lexer.next(){
    Token::Atom(s)=> S::Atom(s),
    t=> panic!("wrong input {:?}",t)
};

loop
{
    let op = match lexer.next()
    {
        Token::Atom(s)=> todo!(),
        Token::Op(op)=> op,
        Token::Eof => break ,
    };
    let (l_bp,r_bp) = infix_binding_power(op);

}

lhs

}

fn infix_binding_power(operator: char)->(u8,u8)
{
    match operator
    {
        '+' => (1,2),
        '*' => (3,4),
        '-' => todo!(),
        '/' => todo!(),
        _ => panic!("unidentified operator {}",operator)
    }

}

enum S{
Atom(char),
Cons(char,Vec<S>)
}

impl fmt::Display for S
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            S::Atom(ch)=>write!(f,"{}", ch) ,
            S::Cons(head,rest)=>{
                write!(f,"{}", head)?;
                for s in rest
                {
                    write!(f,"{}",s);
                }
                write!(f,")")
        }, 
        }
    }
}

fn main() {
    println!("{:?}",Lexer::new("3+4+4+4+5+6").peek());
}

#[test]
fn test1() {
    let s = expr("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))")
}
#[test]
fn test2() {
    let s = expr("1"); 
    assert_eq!(s.to_string(), "1");
}

