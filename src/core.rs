enum LispError {
    DivByZero,
    LispErr,
}

pub enum ParseError {
    SemanticError,
    BracketsError,
    UnexpectedEOF,
}


pub struct SemanticExpr {
    args: Vec<SemanticArg>,
}

pub enum SemanticArg {
    SemanticExpr,
    SemanticAtom(String),
}

impl SemanticExpr {
    fn new() -> SemanticExpr {
        SemanticExpr { args: Vec::new() }
    }

    fn add_arg(&mut self, arg: SemanticArg) -> () {
        self.args.push(arg);
    }

    fn parse(mut tokens: Vec<String>) -> Result<SemanticExpr, ParseError> {
        let mut semantic_expr: SemanticExpr = SemanticExpr::new();
        let token = tokens.remove(0);
        match token.as_str(){
            "(" => {
                semantic_expr.add_arg(SemanticArg::SemanticExpr);
                Ok(semantic_expr)
            },
            _ => semantic_expr.add_arg(SemanticArg::SemanticAtom(token))
            
            
        }
    }

    pub fn tokenize(code: &String) -> Vec<String> {
        code.replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|x| x.to_string())
            .collect()
    }
}

//impl SemanticExpr{
//    fn new(tokenized_code: Vec<String>) -> SemanticExpr{
//    }
//}

pub enum LispExpr {
    Atom(i32),
    Expression(fn(LispList) -> LispExpr, LispList),
}

struct LispList {
    list: Vec<LispExpr>,
}

pub trait LispCore {
    fn plus(&self, args: LispList) -> Result<LispExpr, LispError>;
    //fn minus(&self, val1: LispExpr, val2: LispExpr) -> Result<LispExpr, LispError>;
    //fn div(&self, val1: LispExpr, val2: LispExpr) -> Result<LispExpr, LispError>;
}

pub struct LispVm {
    res: LispExpr,
}

pub struct LispParser {}

//    fn parse_single_code(code: String) -> Result<SemanticExpr, ParseError>{
//        match LispParser::tokenize(code){
//            Ok(args) => Ok(SemanticExpr{func: args[0], args: Vec::new().extend_from_slice(args[1..args.len()])}),
//            Err(err) => Err(err)
//        }
//    }

//fn parse(code: String) -> Result<LispExpr, ParseError>{
//
//}

//impl LispCore for LispVm{
//    fn plus(&self, args: LispList) -> Result<LispExpr, LispError>{
//        let mut res: i32 = 0;
//        for arg in args.list.iter(){
//            match arg{
//                LispExpr::Atom(num) => res += num,
//                LispExpr::Expression(func, args_list) => res += match LispExpr::Expression(func, *args_list).result(){
//                    LispExpr::Atom(value) => value,
//                    LispExpr::Expression(func, args_list) =>
//                },
//            }
//        }
//        Ok(LispExpr::Atom(res))
//    }
//}
