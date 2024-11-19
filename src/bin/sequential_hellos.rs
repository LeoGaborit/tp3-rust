use std::env;

fn main() {
    // Récupérer les arguments passés en ligne de commande
    let args: Vec<String> = env::args().collect();

    // Vérifier qu'un argument est fourni (le nombre n)
    if args.len() != 2 {
        eprintln!("Usage: sequential_hellos <n>");
        std::process::exit(1);
    }

    // Convertir l'argument en entier
    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("L'argument doit être un entier positif.");
            std::process::exit(1);
        }
    };

    // Boucle pour afficher les messages
    for i in 0..n {
        println!("Bonjour n°{}", i);
        println!("Au revoir n°{}", i);
    }
}
