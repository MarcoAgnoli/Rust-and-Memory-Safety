fn main()  {
    let p = Box::new(42);   // alloca un intero nello heap
    drop (p);                // p viene deallocato
    drop(p);                // ERRORE: non posso deallocare p una seconda volta e ottengo l'errore "use of moved value: `p`" perché p è stato "mosso" (moved)
}