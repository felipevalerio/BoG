use std::fs::File;
use std::io::{self, BufRead};

use crate::Token;


pub fn load_stopwords_file() -> io::Result<Vec<String>>{

	let mut vec_stopwords: Vec<String> = Vec::new();
	let file_stopwords = File::open("C:/Users/Pichau/Documents/Rust/naive_bayes/src/stopwords.txt")?;
    let reader = io::BufReader::new(file_stopwords);

	for line in reader.lines() {
		
		let lines = line?;
		vec_stopwords.push(lines);
	}

	Ok(vec_stopwords)
}


pub fn remove_stopwords(stopwords_list: Vec<String>, token: &mut Token) {

	token.word.retain(|word| !stopwords_list.contains(word) 
					&& !word.chars().all(|c| c.is_numeric())); // remove as palavras que estão presentes em stopwords_list
																	// e também caracteres numéricos
}