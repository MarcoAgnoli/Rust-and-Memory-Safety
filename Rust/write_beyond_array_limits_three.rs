fn main() {
    let buf = [10, 20, 30, 40];  // array di quattro elementi
    let indici = [0, 1, 2, 10];  // l'ultimo indice è fuori dai limiti
    for i in indici {
        let x = buf[i];          // bounds check a runtime: fallisce quando i = 10
        println!("{}", x);       // non viene raggiunta all'ultima iterazione
    }
}