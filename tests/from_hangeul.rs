use minno::{from_hangeul, to_hangeul};

#[test]
fn word_convert() {
	assert_eq!(String::from("w"), from_hangeul("ㅡ"));
	assert_eq!(String::from("min"), from_hangeul("갘"));
	assert_eq!(
		String::from("mo li pol lu ti ."),
		from_hangeul("거라펄로다.")
	);
}
