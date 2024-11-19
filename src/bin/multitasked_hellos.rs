use std::env;
use tokio::task;

#[tokio::main]
async fn main() {
    // Récupérer les arguments passés en ligne de commande
    let args: Vec<String> = env::args().collect();

    // Vérifier qu'un argument est fourni (le nombre n)
    if args.len() != 2 {
        eprintln!("Usage: multitasked_hellos <n>");
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

    // Vecteur pour collecter les tâches
    let mut handles = Vec::new();

    // Créer une tâche pour chaque "Bonjour n°i" et "Au revoir n°i"
    for i in 0..n {
        let handle = task::spawn(async move {
            println!("Bonjour n°{}", i);
            println!("Au revoir n°{}", i);
        });

        handles.push(handle);
    }

    // Attendre que toutes les tâches soient terminées
    for handle in handles {
        handle.await.unwrap();
    }
}
