use std::env;
use std::error::Error;
use std::hash::Hash;
use std::process::exit;
use std::fs::read_to_string;
use sha1::Digest;

static SHA_1_Tamanho_Maximo: usize = 40;

pub struct Recev_args{
    wordlist: String,
    Hash: String,

}

impl Recev_args {
    fn new(WordList: String, Hash:String) -> Recev_args {
        Recev_args{wordlist:WordList,Hash: Hash}
    }
}

fn main() {
    println!("Vamos fazer um Quebrador de senha Sha-1");

    let recv_args: Vec<String> = env::args().collect(); // Recev args 

    let args = Verificar_argumentos(&recv_args);

    let Valores = ReadBufferArq(&args.wordlist);
    
    for lines in Valores.lines(){
        let lines = lines.trim();
        if &args.Hash == &hex::encode(sha1::Sha1::digest(lines.as_bytes())){
            println!("Password found: {}", &lines);
        }
    }

    
}

fn Verificar_argumentos(Lista: &[String]) -> Recev_args{
    {
        if Lista.len() !=3 {
            println!("-------Ajuda------\nVocê Tem Que Executar da Seguinte Maneira\n \r./binario Wordlist.aqui Hash.aqui");
            exit(1);
        }   
    }
    // 0 ./bin
    let wordlist = Lista[1].clone();
    let hash = Lista[2].clone();

    {
        if hash.len() != SHA_1_Tamanho_Maximo {
            println!("Sha-1 Invalido ");
            exit(1);
        }
    }

    Recev_args::new(wordlist, hash)
}

fn ReadBufferArq(Arquivo: &String) -> String{
    let bufferArq = read_to_string(Arquivo).unwrap_or_else(|Erro_| {
        println!("Erro no arquivo: {}", Erro_);
        exit(1);});

        bufferArq
}


