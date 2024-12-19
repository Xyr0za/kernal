use crate::ast::Environment;
// use crate::ast::Node;

pub fn parse_line(environment: Environment,tokens: Vec<(String, String)>) {
    if tokens.is_empty() {
        return;
    }

    for token in tokens {
        println!("token: {:?}", token);
    }
}
