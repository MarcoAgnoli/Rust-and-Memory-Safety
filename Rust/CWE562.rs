/*
 *   ATTENZIONE QUESTO FILE NON COMPILA, è solo a scopo didattico per mostrare come il bug non sia esprimibile in Rust
 */

#[allow(dead_code)] // Per evitare warning su funzioni non utilizzate

/* 
*   helper_bad restituisce un riferimento a una stringa locale, che è un dato non valido dopo la fine della funzione
 *  Quindi cercando di compilare si ottiene "expected named lifetime parameter" perchè non ha senso restituire un riferimento a un dato che non esiste più dopo la fine della funzione
 *  Per far compilare questo codice devo commentare la funzione helper_bad e il suo utilizzo
 */

fn helper_bad() -> &str {
    // Tentativo di porting fedele del caso bad in C
    let char_string = String::from("helperBad string");
    &char_string

    /* Questo non compila:
     * il riferimento restituirebbe a un dato locale già distrutto
     * Errore atteso: missing lifetime / borrowed value does not live long enough
     * Interessante notare che se avessi scritto
     *      let char_string = "helperBad string";
     *      char_string
     * invece di usare String::from, allora il codice sarebbe compilato senza errori, 
     * perché i letterali stringa in Rust hanno lifetime 'static e sono nel segmento dati 
     * del binario, non sullo stack, quindi il riferimento è sempre valido. 
     * Ma tutto ciò non riprodurrebbe il bug.
     */ 

}

fn cwe562_return_of_stack_variable_address_return_buf_01_bad() {
    // In C il ramo bad stampa il valore restituito da helperBad()
    // In safe Rust questo ramo non è rappresentabile
}

// helper_good1 restituisce una stringa statica
fn helper_good1() -> &'static str {
    // Il riferimento resta valido per tutta la durata del programma
    "helperGood1 string"
}
// Alternativa più "Rust": restituire il valore invece del riferimento
/*
fn helper_good2() -> String {
    // Restituisce un valore posseduto, non un riferimento a memoria locale
    String::from("helperGood2 string")
}
*/

fn good1() {
    println!("{}", helper_good1());
}

fn cwe562_return_of_stack_variable_address_return_buf_01_good() {
    good1();
}

fn main() {
    println!("Calling good()...");
    cwe562_return_of_stack_variable_address_return_buf_01_good();
    println!("Finished good()");

    println!("Calling bad()...");
    cwe562_return_of_stack_variable_address_return_buf_01_bad();
    println!("Finished bad()");
}