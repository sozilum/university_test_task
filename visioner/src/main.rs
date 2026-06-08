use std::{char, collections::HashMap};
use std::fs::{self, File};
use std::io::Write;
use std::io;

enum DownloadText {
    Usize(usize),
    Text(char),
}


pub fn visioner(phrase: String) -> std::io::Result<()>{

    let code_phrase: String = phrase;
    let alphabet: String = String::from("абвгдеёжзийклмнопрстуфхцчшщъыьэюя");
    let mut alphabet_code_phrase_index_list: Vec<usize> = Vec::new();
    let mut download_text_index_list: Vec<DownloadText> = Vec::new();
    let mut alphabet_index_dict: HashMap<char, usize> = HashMap::new();
    let mut inverted_alphabet_index_dict: HashMap<usize, char> = HashMap::new();
    let mut encripted_text:String = String::new();

    // Create two dicts
    for (index, letter ) in alphabet.chars().enumerate(){
        alphabet_index_dict.insert(letter, index.try_into().unwrap());
        inverted_alphabet_index_dict.insert(index.try_into().unwrap(), letter);
    }

    // create list with id's of letters
    for letter in code_phrase.chars(){
        
        if alphabet_index_dict.get(&letter).is_some(){
            let aux: usize = alphabet_index_dict[&letter];
            alphabet_code_phrase_index_list.push(aux);
        }
    }

    //download text and encription
    let dowload_text: String = fs::read_to_string("src/text.txt")?;

    for letter in dowload_text.chars(){
        
        let letter = letter.to_lowercase().next().unwrap_or(letter);

        if alphabet_index_dict.get(&letter).is_some(){
                let mut aux = alphabet_index_dict[&letter];
                if aux + alphabet_code_phrase_index_list[0] >= alphabet.chars().count(){
                    aux = aux + alphabet_code_phrase_index_list[0] - alphabet.chars().count();
                }else{
                    aux = aux + alphabet_code_phrase_index_list[0];
                }
            let poptd_value = alphabet_code_phrase_index_list.remove(0);
            alphabet_code_phrase_index_list.push(poptd_value);

            download_text_index_list.push(DownloadText::Usize(aux));

        }else{
            download_text_index_list.push(DownloadText::Text(letter));
        };
    }

    for symbol in &download_text_index_list{
        match symbol {
            DownloadText::Usize(letter) => {
                encripted_text.push(inverted_alphabet_index_dict[letter] as char)
            },
            DownloadText::Text(letter) => {
                encripted_text.push(*letter)
                
            },
        }
    }

    let mut new_file = File::create("src/encripted_text.txt")?;
    write!(new_file,"{}" ,encripted_text)?;
    Ok(())

}


fn main() {
    println!("Введите ключ: ");

    let mut phrase: String = String::new();



    io::stdin().read_line(&mut phrase).expect("Ошибка ввода");
    
    let _ = visioner(phrase);
}
