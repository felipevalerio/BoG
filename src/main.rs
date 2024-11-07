use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};


struct Token {
    word: String
}

impl Token {
    
    fn new() -> Token {
        Token { word: String::new() }
    }

    fn tokenizer(&mut self, valores: &Vec<String>) {

        for valor in valores {
            
            self.word = valor.split_whitespace().map(String::from).collect();
        }
    }
}

impl fmt::Display for Token {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Token {{ word: '{}' }}", self.word)
    }
}


// printa as linhas de uma coluna específica
fn print_specific_column(valores: &Vec<String>) {

    for valor in valores {
        println!("{}", valor);
    }
}


fn main() -> io::Result<()> {

    let column_index = 2; // índice da coluna (headlines)
    let mut valores = Vec::new();
    let mut token = Token::new();

    // Abre o arquivo
    let file = File::open("C:/Users/Pichau/Documents/Rust/naive_bayes/src/articles.csv")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {

        let linha = line?;
        let valores_linha: Vec<&str> = linha.split(',').collect();

        // Verifica se há dados suficientes na linha antes de acessar o índice desejado
        if let Some(&valor) = valores_linha.get(column_index) {
            valores.push(valor.to_string());
        }
    }

    // print_specific_column(&valores);
    token.tokenizer(&valores);

    println!("{}", token);

    Ok(())
}
