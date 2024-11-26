use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    // Chaîne statique partagée
    let bonjour: &'static str = "Bonjour";

    // Vecteur pour collecter les tâches
    let mut handles = Vec::new();

    // Créer plusieurs tâches utilisant la même chaîne
    for i in 0..10 {
        let handle = task::spawn(async move {
            println!("Tâche {} dit : {}", i, bonjour);
        });

        handles.push(handle);
    }

    // Attendre que toutes les tâches soient terminées
    for handle in handles {
        handle.await.unwrap();
    }
}
