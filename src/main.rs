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

mod tests
{
    use std::fmt;

    // fn test_Lexer()
    // {
    //     assert_eq!()
    // }
}
