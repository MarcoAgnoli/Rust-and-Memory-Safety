/*
*   ATTENZIONE QUESTO FILE NON COMPILA, è solo a scopo didattico per mostrare come il bug non sia esprimibile in Rust
*/

fn cwe416_use_after_free_malloc_free_char_01_bad() {
    /*  
    *   Questo è un tentativo di porting fedele del caso bad in C ma NON compila in Rust. 
    *   L'assegnazione è fatta in due passaggi per essere fedele al codice C 
    *   ma in Rust è più idiomatico scrivere 'let data = "A".repeat(99);' direttamente, senza dichiarare prima la variabile e poi assegnarla. 
    */
    let data;
    data = String::from("A".repeat(99));
    
    drop(data);
    println!("{}", data);     // Errore atteso: "borrow of moved value: `data`"
}

// goodG2B: il dato viene allocato e poi usato
fn good_g2b() {
    // Qui, diversamente dalla funzione bad, uso una assegnazione idiomatica per creare la stringa, come andrebbe fatto normalmente in Rust.  
    let data = "A".repeat(99);
    println!("{}", data);
    // data viene distrutto automaticamente a fine scope
}

// goodB2G: il dato viene distrutto e poi non viene più usato
fn good_b2g() {
    let data = "A".repeat(99);
    drop(data);
    // non fare nulla
}

fn cwe416_use_after_free_malloc_free_char_01_good() {
    good_g2b();
    good_b2g();
}

fn main() {
    println!("Calling good()...");
    cwe416_use_after_free_malloc_free_char_01_good();
    println!("Finished good()");

    println!("Calling bad()...");
    cwe416_use_after_free_malloc_free_char_01_bad();
    println!("Finished bad()");
}