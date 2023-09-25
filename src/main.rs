pub mod fileread;

fn main() {
    println!("Hello, world!");
    let lines = fileread::read_file(r#"teste.txt"#);
    for line in lines {
        println!("{}", line);
    }
}
