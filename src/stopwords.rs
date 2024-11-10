use std::fs::File;
use std::io::{self, BufRead};



fn remove_stopwords() {

	let file_stopwords = File::open("C:/Users/Pichau/Documents/Rust/naive_bayes/src/stopwords.txt")?;
    let reader = io::BufReader::new(file_stopwords);

	for line in reader.lines {
		
		let lines = line?;
		let vec_stopwords: Vec<&str> = lines.collect();
	}


}