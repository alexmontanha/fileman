use std::{fs::File, io::Read};

pub fn read_file(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).expect("Arquivo não encontrado");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Erro ao tentar ler o arquivo");
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    lines
}