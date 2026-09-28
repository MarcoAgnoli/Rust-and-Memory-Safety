/*  32 byte di stringa che poi verrà copiata in un buffer di 16 byte, causando un overflow se non gestito correttamente
    Perchè ho scritto &str invece che str? Perché in Rust le stringhe sono rappresentate come slice di byte. 
    */
const SRC_STR: &str = "0123456789abcdef0123456789abcde";

struct CharVoid<'a> {
    /*  char_first è un array di 16 byte, che rappresenta il buffer sullo stack. 
        In C sarebbe un array di char, ma in Rust uso u8 perché un char in C è un byte, mentre in Rust un char è un carattere Unicode che occupa 4 byte mentre un u8 è un byte.
        void_second è un riferimento a una stringa statica, che rappresenta il campo "puntatore" del testcase C. 
        Qui uso il lifetime 'a per indicare che il riferimento a str vive almeno quanto l'istanza di CharVoid. 
        In alternativa avrei potuto usare &'static str se volevo indicare che la stringa vive per tutta la durata del programma, ma in questo caso 'a è meglio e fa deallocare la stringa solo quando viene deallocata l'istanza di CharVoid.
        void_third è un campo aggiuntivo che non viene usato nel testcase, ma lo includo per mantenere il layout concettuale simile a quello del struct in C.
        */
    char_first: [u8; 16],                      // buffer sullo stack, [u8; 16] è un array di 16 byte di tipo unsigned char in C
    void_second: &'a str,                      // campo "puntatore" come nel testcase C, uso il lifetime 'a per indicare che il riferimento a str vive almeno quanto l'istanza di CharVoid
    void_third: &'a str,                       // campo aggiuntivo per mantenere il layout concettuale simile
}

fn cwe121_stack_based_buffer_overflow_char_type_overrun_memcpy_01_bad() {
    let mut struct_char_void = CharVoid {
        char_first: [0; 16],                        // [0; 16] crea un array di 16 byte tutti inizializzati a zero
        void_second: SRC_STR,
        void_third: "",
    };
    println!("{}", struct_char_void.void_second);   // Stampa il contenuto iniziale puntato da void_second
    /*
    PANIC a RUNTIME: tenta di copiare tutta la stringa sorgente dentro un buffer di 16 byte.
    In safe Rust questo NON corrompe la memoria: copy_from_slice genera panic se le lunghezze non coincidono.
    Nota inoltre che copy_from_slice si aspetta una slice di byte, quindi è necessario convertire la stringa in byte usando as_bytes() prima di copiarla.
    Infatti in Rust c'è differenza tra stringhe (str) e array di byte (u8): una stringa è una sequenza di caratteri Unicode che occupa 4 byte per carattere.
    */
    let struct_size = std::mem::size_of::<CharVoid>();
    struct_char_void.char_first.copy_from_slice(&SRC_STR.as_bytes()[0..struct_size]);
    
    // Queste righe normalmente non vengono raggiunte, perché copy_from_slice genera panic. Uso from_utf8_lossy per stampare il buffer come stringa (altrimenti stamperebbe i valori dei byte raw)
    println!("{}", String::from_utf8_lossy(&struct_char_void.char_first));
    println!("{}", struct_char_void.void_second);

    // solo per evitare warning
    let _ = struct_char_void.void_third;
}

// good1 corrisponde al ramo FIX del testcase Juliet
fn good1() {
    let mut struct_char_void = CharVoid {
        char_first: [0; 16],
        void_second: SRC_STR,
        void_third: "",
    };

    // Stampa il contenuto iniziale puntato da void_second
    println!("{}", struct_char_void.void_second);

    // Ottiene la dimensione del buffer di destinazione
    let dst_len = struct_char_void.char_first.len();
    
    /*  Copia solo i primi 16 byte della stringa sorgente. In questo modo la lunghezza della sorgente coincide con quella del buffer di destinazione.
        Qui invece devo mettere & davanti a SRC_STR.as_bytes() perché copy_from_slice si aspetta una slice e non un array.
    */
    struct_char_void.char_first.copy_from_slice(&SRC_STR.as_bytes()[0..dst_len]);

    /*
    Stampiamo il buffer come stringa.
    from_utf8_lossy è comodo perché evita di dover gestire esplicitamente eventuali errori di conversione.
    */
    println!("{}", String::from_utf8_lossy(&struct_char_void.char_first));

    // Il campo "puntatore" resta valido perché non è stato sovrascritto
    println!("{}", struct_char_void.void_second);

    // solo per evitare warning sul campo inutilizzato
    let _ = struct_char_void.void_third;
}

fn cwe121_stack_based_buffer_overflow_char_type_overrun_memcpy_01_good() {
    good1();
}

fn main() {
    println!("Calling good()...");
    cwe121_stack_based_buffer_overflow_char_type_overrun_memcpy_01_good();
    println!("Finished good()");

    println!("Calling bad()...");
    cwe121_stack_based_buffer_overflow_char_type_overrun_memcpy_01_bad();
    println!("Finished bad()");
}