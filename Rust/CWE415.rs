
fn cwe415_double_free_malloc_free_char_01_bad() {
    /*  
    *   Questo è un tentativo di porting fedele del caso bad in C ma NON compila in Rust. L'assegnazione è fatta in due passaggi per essere fedele al codice C 
    *   ma in Rust è più idiomatico scrivere let data = "A".repeat(99); direttamente, senza dichiarare prima la variabile e poi assegnarla. 
    */
    let data;
    data = String::from("A".repeat(99));

    drop(data);
    // Questo non compila: dopo il primo drop(data), data non è più disponibile. Errore atteso: "use of moved value: `data`"
    drop(data);

}

// goodG2B: il dato viene allocato e liberato una sola volta
fn good_g2b() {
    let data = "A".repeat(99);
    drop(data);
}

// goodB2G: il dato viene liberato e poi non viene più usato
fn good_b2g() {
    let data = "A".repeat(99);
    drop(data);
    // non fare nulla
}

fn cwe415_double_free_malloc_free_char_01_good() {
    good_g2b();
    good_b2g();
}

fn main() {
    println!("Calling good()...");
    cwe415_double_free_malloc_free_char_01_good();
    println!("Finished good()");

    println!("Calling bad()...");
    cwe415_double_free_malloc_free_char_01_bad();
    println!("Finished bad()");
}