use std::fs::File;
use std::io::{self, BufRead};


struct Token {
    word: Vec<String>
}

impl Token {
    
    fn new() -> Token {
        Token { word: Vec::new() }
    }

    fn tokenizer(&mut self, valores: &Vec<String>) {

        for valor in valores {
            
            let clean_value = valor
                                        .replace("\'", "") // => pensar em um jeito melhor de fazer isso (pelo amor de Deus)
                                        .replace("\"", "")
                                        .replace("‘", "")
                                        .replace("!", "")
                                        .replace("?", "")
                                        .replace("(", "")
                                        .replace(")", "")
                                        .replace(":", "")
                                        .replace(";", "")
                                        .replace("’", "")
                                        .replace("--", "")
                                        .replace("&", "")
                                        .replace(".", "")
                                        .to_lowercase();

            self.word.extend(clean_value.split_whitespace().map(String::from)); // split do texto nos espaços em branco
        }
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

    for words in token.word {
        println!("{}", words);
    }

    Ok(())
}
