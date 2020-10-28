use super::chars::hangeul_to_char;
use hangeul::decompose_char;

pub fn from_hangeul(hangeul_str: &str) -> String {
	let chars = hangeul_str.chars();
	let mut result = String::new();
	for combo in chars {
		match decompose_char(&combo) {
			Ok((a, b, c)) => result.insert_str(
				result.len(),
				&format!(
					"{}{}{} ",
					hangeul_to_char(a),
					hangeul_to_char(b),
					match c {
						Some(c) => hangeul_to_char(c).to_string(),
						None => String::new(),
					}
				),
			),
			Err(_) => result.insert_str(
				result.len(),
				&format!("{} ", hangeul_to_char(combo).to_string()),
			),
		}
	}
	result.replace(" ㅎ", "").trim().to_string()
}
