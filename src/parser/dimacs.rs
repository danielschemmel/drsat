//use ::std::vec::Vec;
use ::std::option::Option;
use ::std::str;
use ::regex::bytes::Regex;

use gp::ast;

fn skip_comments(bytes: &[u8]) -> &[u8] {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"^(?:[ \t]*c[^\n]*(?:\n|$))*").unwrap();
	}
	let m = RE.find(bytes).unwrap();
	assert_eq!(m.start(), 0);
	&bytes[m.end()..]
}

#[test]
fn skip_comments_test() {
	assert_eq!(skip_comments(b""), &b""[..]);
	assert_eq!(skip_comments(b"c"), &b""[..]);
	assert_eq!(skip_comments(b"c\n"), &b""[..]);
	assert_eq!(skip_comments(b"c a b c\n"), &b""[..]);
	assert_eq!(skip_comments(b"c a\nc b\n"), &b""[..]);
	assert_eq!(skip_comments(b"p cnf 1 2"), &b"p cnf 1 2"[..]);
	assert_eq!(skip_comments(b"c a\nc b\np cnf 1 2"), &b"p cnf 1 2"[..]);
}

fn parse_header(bytes: &[u8]) -> Option<(&[u8], usize, usize)> {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"^[ \t]*p[ \t]+cnf[ \t]+([0-9]+)[ \t]+([0-9]+)[ \t]*(?:\r?\n|$)").unwrap();
	}
	if let Option::Some(m) = RE.captures(bytes) {
		assert_eq!(m.len(), 3);
		assert_eq!(m.get(0).unwrap().start(), 0);
		let variables = str::from_utf8(m.get(1).unwrap().as_bytes()).unwrap().parse::<usize>().unwrap();
		let clauses = str::from_utf8(m.get(2).unwrap().as_bytes()).unwrap().parse::<usize>().unwrap();
		Option::Some((&bytes[m.get(0).unwrap().end()..], variables, clauses))
	} else { Option::None }
}

fn parse_variable(bytes: &[u8]) -> Option<(&[u8], usize, bool)> {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"^[ \t\r\n]*(?P<neg>-[ \t\r\n]*)?(?P<id>[0-9]+)").unwrap();
	}
	if let Option::Some(m) = RE.captures(bytes) {
		assert!(m.len() == 2 || m.len() == 3);
		assert_eq!(m.get(0).unwrap().start(), 0);
		let id = str::from_utf8(m.name("id").unwrap().as_bytes()).unwrap().parse::<usize>().unwrap();
		Option::Some((&bytes[m.get(0).unwrap().end()..], id, m.name("neg").is_some()))
	} else { Option::None }
}

fn parse_clause(mut bytes: &[u8]) -> Option<(&[u8], ast::Or)> {
	let mut clause = ast::Or::new();
	loop {
		if let Option::Some((remaining, id, negated)) = parse_variable(bytes) {
			bytes = remaining;
			if id == 0 { break; }
			clause.variables.push(ast::Variable::new(id, negated));
		} else { return Option::None }
	}
	if clause.variables.len() != 0 {
		Option::Some((bytes, clause))
	} else { Option::None }
}

fn skip_end(bytes: &[u8]) -> &[u8] {
	lazy_static! {
		static ref RE: Regex = Regex::new(r"^[ \t\r\n]*(?:%[ \t\r\n]*0[ \t\r\n]*)?").unwrap();
	}
	let m = RE.find(bytes).unwrap();
	assert_eq!(m.start(), 0);
	&bytes[m.end()..]
}

pub fn parse(mut bytes: &[u8]) -> Option<ast::Node> {
	bytes = skip_comments(bytes);
	if let Option::Some((remainder, _, clauses)) = parse_header(bytes) {
		bytes = remainder;
		let mut query = ast::And::new();
		for _ in 0..clauses {
			if let Option::Some((remaining, or)) = parse_clause(bytes) {
				bytes = remaining;
				query.nodes.push(ast::Node::Or(or));
			} else {
				return Option::None;
			}
		}
		bytes = skip_end(bytes);
		if bytes.len() == 0 {
			Option::Some(ast::Node::And(query))
		} else { Option::None }
	} else { Option::None }
}