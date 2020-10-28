pub const A: (char, char) = ('ㅣ', 'a');
pub const W: (char, char) = ('ㅡ', 'w');
pub const E: (char, char) = ('ㅜ', 'e');
pub const U: (char, char) = ('ㅗ', 'u');
pub const I: (char, char) = ('ㅏ', 'i');
pub const O: (char, char) = ('ㅓ', 'o');
pub const S: (char, char) = ('ㅇ', 's');
pub const F: (char, char) = ('ㅈ', 'f');
pub const X: (char, char) = ('ㅁ', 'x');
pub const M: (char, char) = ('ㄱ', 'm');
pub const P: (char, char) = ('ㅍ', 'p');
pub const Q: (char, char) = ('ㄴ', 'q');
pub const T: (char, char) = ('ㄷ', 't');
pub const K: (char, char) = ('ㅌ', 'k');
pub const B: (char, char) = ('ㅂ', 'b');
pub const N: (char, char) = ('ㅋ', 'n');
pub const L: (char, char) = ('ㄹ', 'l');
pub const H: (char, char) = ('ㅅ', 'h');
/// A character used to continue minno words when written in Hangeul
pub const CONTINUE_CHAR: char = 'ㅎ';
/// A character that is simply used for formatting and an be ignored.
/// Also used can be ㅊ
pub const IGNORE_CHAR: char = 'ㅃ';

/// Returns a pair of Hangeul and roman chars from roman input
pub fn char_to_set(roman: char) -> Option<(char, char)> {
	match roman {
		'a' => Some(A),
		'w' => Some(W),
		'e' => Some(E),
		'u' => Some(U),
		'i' => Some(I),
		'o' => Some(O),
		's' => Some(S),
		'f' => Some(F),
		'x' => Some(X),
		'm' => Some(M),
		'p' => Some(P),
		'q' => Some(Q),
		't' => Some(T),
		'k' => Some(K),
		'b' => Some(B),
		'n' => Some(N),
		'l' => Some(L),
		'h' => Some(H),
		_ => None,
	}
}

/// Returns a pair of Hangeul and roman chars from hangeul input
pub fn hangeul_to_set(hangeul: char) -> Option<(char, char)> {
	match hangeul {
		'ㅣ' => Some(A),
		'ㅡ' => Some(W),
		'ㅜ' => Some(E),
		'ㅗ' => Some(U),
		'ㅏ' => Some(I),
		'ㅓ' => Some(O),
		'ㅇ' => Some(S),
		'ㅈ' => Some(F),
		'ㅁ' => Some(X),
		'ㄱ' => Some(M),
		'ㅍ' => Some(P),
		'ㄴ' => Some(Q),
		'ㄷ' => Some(T),
		'ㅌ' => Some(K),
		'ㅂ' => Some(B),
		'ㅋ' => Some(N),
		'ㄹ' => Some(L),
		'ㅅ' => Some(H),
		_ => None,
	}
}

/// Transforms a roman input char into the Hangeul equivalent
pub fn char_to_hangeul(roman: char) -> char {
	match char_to_set(roman) {
		Some(c) => c.0,
		None => roman,
	}
}

/// Transforms a roman input char into the Hangeul equivalent
pub fn hangeul_to_char(hangeul: char) -> char {
	match hangeul_to_set(hangeul) {
		Some(c) => c.1,
		None => hangeul,
	}
}

/// Checks if a roman input is a vowel
pub fn is_vowel(roman: char) -> bool {
	match roman {
		'a' | 'w' | 'e' | 'u' | 'i' | 'o' => true,
		_ => false,
	}
}

/// Checks if a roman input is a consonant
pub fn is_consonant(roman: char) -> bool {
	match roman {
		's' | 'f' | 'x' | 'm' | 'p' | 'q' | 't' | 'k' | 'b' | 'n' | 'l' | 'h' => true,
		_ => false,
	}
}
