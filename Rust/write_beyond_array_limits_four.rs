fn main() {
    let src = b"Hello"; // src è un array di byte (u8) con 5 elementi
    let mut buf = [0u8; 4]; // buf è un array di byte con 4 elementi, tutti inizializzati a zero
    buf.copy_from_slice(src); // ERRORE: src ha 5 byte, buf ne ha solo 4. copy_from_slice genera panic a runtime perché le lunghezze non coincidono
   }