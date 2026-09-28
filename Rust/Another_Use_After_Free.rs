fn main() {
    
    fn dangling() -> &i32 {
        let x = 42; // x è allocato nello stack
        &x          // ERRORE: non posso restituire un riferimento a x, perché x sarà deallocato quando la funzione termina
    }
    
    
    /* Prima alternativa: restituire il valore invece di un riferimento
    fn dangling() -> i32 {
        let x = 42;
        x
    }
    */

    /* Seconda alternativa: variabile locale statica
    fn dangling() -> &'static i32 {
        static X: i32 = 42;
        &X
    }
    */

    let r = dangling(); // r è un riferimento a un dato che è stato deallocato
    println!("{}", r);  // r userebbe un dato che sarebbe stato liberato
}