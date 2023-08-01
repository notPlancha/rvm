use crate::parsing::version_parser::Version;

peg::parser!( pub grammar the_parser() for str {
  pub rule parse_version() -> Version
    = ['v' | 'V']? " "? m:main() {Default::default()} //TODO

  rule num() -> u32
    = n:$(['0'..='9']+) {? n.parse().or(Err("number")) }

  rule main() -> (u32, Option<u32>, Option<u32>)
    = M:num() "."? m:num()? "."? p:num()? { (M, m, p) }


});