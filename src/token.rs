
pub const RUNE_LITERAL: &str = "runeLiteral";
pub const STRING_LITERAL: &str = "stringLiteral";
pub const INTEGER_LITERAL: &str = "integerLiteral";
pub const BOOLEAN_LITERAL: &str = "booleanLiteral";
pub const IDENTIFIER: &str = "identifier";
pub const EOF: &str = "eof";
pub const UNKNOWN: &str = "unknown";

pub const PIPE: &str = "|";
pub const AMP: &str = "&";
pub const NOT: &str = "!";
pub const AND: &str = "&&";
pub const OR: &str = "||";
pub const EQ: &str = "==";
pub const NEQ: &str = "!=";
pub const LT: &str = "<";
pub const RT: &str = ">";
pub const LT_EQ: &str = "<=";
pub const RT_EQ: &str = ">=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const TIMES: &str = "*";
pub const DIV: &str = "/";
pub const POST_INCREMENT: &str = "++";
pub const POST_DECREMENT: &str = "--";

 // Reserved
 pub const INTEGER: &str = "integer";
 pub const RUNE: &str = "rune";
 pub const STRING: &str = "string";
 pub const BOOLEAN: &str = "boolean";
 pub const USER: &str = "user";
 pub const REPO: &str = "repo";
 pub const CI_CONFIG: &str = "ciConfig";
 pub const DEPLOYMENT: &str = "deployment";
 pub const MANIFEST: &str = "manifest";
 pub const IF: &str = "if";
 pub const ELSE: &str = "else";
 pub const WHILE: &str = "while";
 pub const VOID: &str = "void";
 pub const VAR: &str = "var";
 pub const COMMAND: &str = "command";
 pub const FUNC: &str = "func";
 pub const COMPLEX: &str = "complex";
 pub const RETURN: &str = "return";

 pub const L_PARENT: &str = "(";
 pub const R_PARENT: &str = ")";
 pub const L_BRACE: &str = "{";
 pub const R_BRACE: &str = "}";
 pub const SEMI: &str = ";";
 pub const COMMA: &str = ",";
 pub const ASSIGN: &str = "=";
 pub const DOT: &str = ".";

 #[derive(Debug)]
 pub struct Token {
     pub token_type: String,
     pub value: String,
     pub line: usize,
     pub column: usize,
 }
