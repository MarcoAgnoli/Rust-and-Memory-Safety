fn  main () {
    let mut v = vec![1,2,3,4]; // v è un array di 4 elementi
    let i = 5; // indice fuori dai limiti dell'array
    v[i] = 5; // ERRORE rilevato dal compilatore: indice fuori dai limiti dell'array
   } 