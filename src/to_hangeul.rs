use super::chars::{char_to_hangeul, is_consonant, is_vowel, CONTINUE_CHAR, IGNORE_CHAR};
use hangeul::compose_char;
use regex::Regex;

pub fn to_hangeul(phrase: &str) -> String {
	let whitespace_regex = Regex::new(r"\W+").unwrap();
	let words: String = whitespace_regex.replace_all(phrase, " ").into();
	let mut hangeul_phrase = phrase.to_string();
	let words = words.as_str().split(" ");
	for word in words {
		hangeul_phrase = hangeul_phrase.replace(word, word_to_hangeul(word).as_str());
	}
	hangeul_phrase.replace(" ", "")
}

/// Turns a single romaized minno word into Hanguel-ized version
pub fn word_to_hangeul(word: &str) -> String {
	// Chars for the input word
	let mut word_chars = word.chars();
	match word_chars.next() {
		// First letter vowel e.g om
		Some(v) if is_vowel(v) => match word_chars.next() {
			// Second letter vowel e.g io
			Some(vv) if is_vowel(vv) => {
				format!(
					"{v}{rest}",
					v = char_to_hangeul(v),
					rest = match word_chars.next() {
						// Third letter consonant, this returns [mvc] e.g ian
						Some(vvc) if is_consonant(vvc) => match compose_char(
							&CONTINUE_CHAR,
							&char_to_hangeul(vv),
							Some(&char_to_hangeul(vvc)),
						) {
							Ok(han) => han,
							Err(_) => IGNORE_CHAR,
						}
						.to_string(),
						// Third letter vowel, this returns [mvv] e.g uai
						Some(vvv) if is_vowel(vvv) => format!(
							"{}{}",
							match compose_char(&CONTINUE_CHAR, &char_to_hangeul(vv), None,) {
								Ok(han) => han,
								Err(_) => IGNORE_CHAR,
							},
							match compose_char(&CONTINUE_CHAR, &char_to_hangeul(vvv), None,) {
								Ok(han) => han,
								Err(_) => IGNORE_CHAR,
							}
						),
						// No third letter, this returns [mv] e.g ue
						_ => match compose_char(&CONTINUE_CHAR, &char_to_hangeul(vv), None) {
							Ok(han) => han,
							Err(_) => IGNORE_CHAR,
						}
						.to_string(),
					}
				)
			}
			// Second letter consonant (vc) -> [ivc] e.g aq
			Some(vc) if is_consonant(vc) => match compose_char(
				&IGNORE_CHAR,
				&char_to_hangeul(v),
				Some(&char_to_hangeul(vc)),
			) {
				Ok(han) => han.to_string(),
				Err(_) => IGNORE_CHAR.to_string(),
			},
			// No other letters, so return it
			_ => char_to_hangeul(v).to_string(),
		},
		// First letter consonant [c] e.g to
		Some(c) if is_consonant(c) => {
			match word_chars.next() {
				Some(cv) if is_vowel(cv) => consonant_parsing(word.chars()),
				// Second is also a consonant. This will be numbers or "ts/tx" [ccv] or [ccc...]
				Some(cc) if is_consonant(cc) => match word_chars.next() {
					// Third is vowel, making a word. e.g tsa
					Some(ccv) if is_vowel(ccv) => {
						let mut cons_chars = word.chars();
						cons_chars.next();
						format!("{}{}", char_to_hangeul(c), consonant_parsing(cons_chars))
					}
					// Formats numbers (sfxm) [cccc]
					Some(ccc) if is_consonant(ccc) => format!(
						"{}{}{}{}",
						char_to_hangeul(c),
						char_to_hangeul(cc),
						char_to_hangeul(ccc),
						word_chars
							.map(|roman| char_to_hangeul(roman).to_string())
							.collect::<String>()
					),
					_ => format!("{}{}", char_to_hangeul(c), char_to_hangeul(cc)),
				},
				// No other letters, so return it e.g "m"
				_ => char_to_hangeul(c).to_string(),
			}
		}
		// No letters, return empty
		_ => String::new(),
	}
}

/// Transforms a consonant string into
fn consonant_parsing(mut cons_chars: std::str::Chars) -> String {
	match cons_chars.next() {
		Some(c) if is_consonant(c) => match cons_chars.next() {
			Some(cv) if is_vowel(cv) => match cons_chars.next() {
				Some(cvv) if is_vowel(cvv) => format!(
					"{cv}{rest}",
					cv = match compose_char(&char_to_hangeul(c), &char_to_hangeul(cv), None) {
						Ok(han) => han,
						_ => IGNORE_CHAR,
					},
					rest = match cons_chars.next() {
						Some(cvvv) if is_vowel(cvvv) => format!(
							"{mv}{rest}",
							mv = match compose_char(&CONTINUE_CHAR, &char_to_hangeul(cvv), None) {
								Ok(han) => han,
								_ => IGNORE_CHAR,
							},
							rest = match cons_chars.next() {
								Some(cvvvc) if is_consonant(cvvvc) => match compose_char(
									&CONTINUE_CHAR,
									&char_to_hangeul(cvvv),
									Some(&char_to_hangeul(cvvvc))
								) {
									Ok(han) => han,
									_ => IGNORE_CHAR,
								},
								_ =>
									match compose_char(&CONTINUE_CHAR, &char_to_hangeul(cvvv), None)
									{
										Ok(han) => han,
										_ => IGNORE_CHAR,
									},
							}
						),
						Some(cvvc) if is_consonant(cvvc) => match compose_char(
							&CONTINUE_CHAR,
							&char_to_hangeul(cvv),
							Some(&char_to_hangeul(cvvc))
						) {
							Ok(han) => han.into(),
							_ => IGNORE_CHAR.into(),
						},
						_ => match compose_char(&CONTINUE_CHAR, &char_to_hangeul(cvv), None) {
							Ok(han) => han.into(),
							_ => IGNORE_CHAR.into(),
						},
					},
				),
				Some(cvc) if is_consonant(cvc) => match compose_char(
					&char_to_hangeul(c),
					&char_to_hangeul(cv),
					Some(&char_to_hangeul(cvc)),
				) {
					Ok(han) => han.to_string(),
					Err(_) => IGNORE_CHAR.to_string(),
				},
				_ => match compose_char(&char_to_hangeul(c), &char_to_hangeul(cv), None) {
					Ok(han) => han.to_string(),
					Err(_) => IGNORE_CHAR.to_string(),
				},
			},
			_ => IGNORE_CHAR.to_string(),
		},
		_ => IGNORE_CHAR.to_string(),
	}
}
