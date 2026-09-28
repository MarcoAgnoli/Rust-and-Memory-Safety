/* Use After Free esempio fatto nelle slide*/
fn main() {
    let p = Box::new(42);   // alloca un intero nello heap
    let q = &p;             // q prende in prestito p
    drop(p);                // ERRORE: non posso distruggere p mentre è preso in prestito
    println!("{}", q);      // q userebbe un dato che sarebbe stato liberato
}