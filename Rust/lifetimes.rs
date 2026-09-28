fn main() {
    let r; // dichiaro una variabile r senza inizializzarla
    {
        let x = 42; // x è allocato nello stack
        r = &x;     // ERRORE: non posso assegnare a r un riferimento a x, perché x sarà deallocato quando questo blocco termina
    }
    println!("{}", r); // r userebbe un dato che sarebbe stato liberato
}