/* Non-copy types esempio delle slide */
fn main() {
    let a = vec![1,2,3]; // a è un tipo non-copy
    let b = a; // a viene spostato in b, a non è più valido
    /* Rimuovi il commento e poi commenta l'istruzione sopra se vuoi far vedere delle alternative che compiliano
    let b = a.clone(); // a viene copiato in b, a è ancora valido
    let b = &a; // b è un riferimento a a, a è ancora valido
    */
    println!("{:?}", a); // ERRORE: uso di a dopo che è stato spostato
}