use minno::{to_hangeul, word_to_hangeul};

#[test]
fn word_convert() {
	assert_eq!(String::from("ㅡ"), word_to_hangeul("w"));
	assert_eq!(String::from("거"), word_to_hangeul("mo"));
	assert_eq!(String::from("킼"), word_to_hangeul("nan"));
	assert_eq!(String::from("갘"), word_to_hangeul("min"));
	assert_eq!(String::from("루하"), word_to_hangeul("lei"));
	assert_eq!(String::from("리헉"), word_to_hangeul("laom"));
	assert_eq!(String::from("바히헌"), word_to_hangeul("biaoq"));
	assert_eq!(String::from("ㅗ히하"), word_to_hangeul("uai"));
	assert_eq!(String::from("ㅗ후"), word_to_hangeul("ue"));
	assert_eq!(String::from("ㄷ이"), word_to_hangeul("tsa"));
	assert_eq!(String::from("ㄷ우하"), word_to_hangeul("tsei"));
	assert_eq!(String::from("ㄷ막"), word_to_hangeul("txim"));
}

#[test]
fn phrase_convert() {
	assert_eq!(String::from("다지허비."), to_hangeul("ti fao ba."));
	assert_eq!(
		String::from("쿠읻삔비다ㅣ"),
		to_hangeul("ne sat aq ba ti a")
	);
	assert_eq!(
		String::from("구,족키,포히하."),
		to_hangeul("me, fum na, puai.")
	);
}
