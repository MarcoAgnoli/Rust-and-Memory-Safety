/*  32 byte di stringa che poi verrà copiata in un buffer di 16 byte, causando un overflow se non gestito correttamente
    Perchè ho scritto &str invece che str? Perché in Rust le stringhe sono rappresentate come slice di byte. 
    In C, una stringa è rappresentata come un array di char terminato da null, ma in Rust non esiste il concetto di stringa terminata da null. 
    Quindi &str è la rappresentazione più naturale per una stringa in Rust.
*/
const SRC_STR: &str = "0123456789abcdef0123456789abcde";

/*  char_first è un array di 16 byte, che rappresenta il buffer sullo stack. 
        In C sarebbe un array di char, ma in Rust uso u8 perché un char in C è un byte, mentre in Rust un char è un carattere Unicode che occupa 4 byte mentre un u8 è un byte.
        void_second è un riferimento a una stringa statica, che rappresenta il campo "puntatore" del testcase C. 
        Qui uso il lifetime 'a per indicare che il riferimento a str vive almeno quanto l'istanza di CharVoid. 
        In alternativa avrei potuto usare &'static str se volevo indicare che la stringa vive per tutta la durata del programma, ma in questo caso 'a è meglio e fa deallocare la stringa solo quando viene deallocata l'istanza di CharVoid.
        void_third è un campo aggiuntivo che non viene usato nel testcase, ma lo includo per mantenere il layout concettuale simile a quello del struct in C.
        */
struct CharVoid<'a> {
    char_first: [u8; 16],
    void_second: &'a str,
    void_third: &'a str,
}

fn cwe122_heap_based_buffer_overflow_char_type_overrun_memcpy_01_bad() {
    
    let mut struct_char_void = Box::new(CharVoid {
        char_first: [0; 16],
        void_second: SRC_STR,
        void_third: "",
    });

    println!("{}", struct_char_void.void_second);

    /*
    PANIC a RUNTIME: tenta di copiare tutta la stringa sorgente dentro un buffer di 16 byte.
    In safe Rust questo NON corrompe la memoria: copy_from_slice genera panic se le lunghezze non coincidono.
    Nota inoltre che copy_from_slice si aspetta una slice di byte, quindi è necessario convertire la stringa in byte usando as_bytes() prima di copiarla.
    Infatti in Rust c'è differenza tra stringhe (str) e array di byte (u8): una stringa è una sequenza di caratteri Unicode che occupa 4 byte per carattere.
    */
    let struct_size = std::mem::size_of::<CharVoid>();
    struct_char_void.char_first.copy_from_slice(&SRC_STR.as_bytes()[0..struct_size]);

    // Queste righe non vengono raggiunte a causa del panic
    println!("{}", String::from_utf8_lossy(&struct_char_void.char_first));
    println!("{}", struct_char_void.void_second);
    // Box viene deallocato automaticamente qui (Drop)

    // solo per evitare warning
    let _ = struct_char_void.void_third;
}

fn good1() {
    let mut struct_char_void = Box::new(CharVoid {
        char_first: [0u8; 16],
        void_second: SRC_STR,
        void_third: "",
    });

    println!("{}", struct_char_void.void_second);

    // FIX: usiamo la dimensione del buffer di destinazione char_first (16 byte)
    let dst_len = struct_char_void.char_first.len();
    struct_char_void.char_first.copy_from_slice(&SRC_STR.as_bytes()[0..dst_len]);

    println!("{}", String::from_utf8_lossy(&struct_char_void.char_first));
    println!("{}", struct_char_void.void_second);
    // Box viene deallocato automaticamente qui (Drop)

    // solo per evitare warning
    let _ = struct_char_void.void_third;
}

fn cwe122_heap_based_buffer_overflow_char_type_overrun_memcpy_01_good() {
    good1();
}

fn main() {
    println!("Calling good()...");
    cwe122_heap_based_buffer_overflow_char_type_overrun_memcpy_01_good();
    println!("Finished good()");

    println!("Calling bad()...");
    cwe122_heap_based_buffer_overflow_char_type_overrun_memcpy_01_bad();
    println!("Finished bad()");
}