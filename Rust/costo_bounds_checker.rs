use std::hint::black_box;
/* Importa black_box: una funzione che impedisce al compilatore di eliminare
    calcoli "inutili". Senza di essa, il compilatore potrebbe accorgersi che
    il risultato della somma non viene mai usato e non eseguire il loop. 
*/

use std::time::{Duration, Instant};
/*  Importa gli strumenti per misurare il tempo:
    - Instant: un istante preciso nel tempo (come un cronometro)
    - Duration: una durata (differenza tra due Instant)
*/

const ARRAY_SIZE: usize = 100_000_000;  // Dimensione dell'array: 100 milioni di elementi.

const ITERATIONS: u32 = 5;              // Numero di ripetizioni per ogni variante.

fn sum_safe(data: &[u64]) -> u64 {      //  Riceve un riferimento immutabile a uno slice di u64. Ricorda che &[u64] è un "fat pointer"
    let mut acc: u64 = 0;               // Accumulatore inizializzato a zero.
    for i in 0..data.len() {
        acc = acc.wrapping_add(data[i]);
        /*  data[i]: accesso safe con bounds check implicito.
            Il compilatore verifica che i < data.len() prima di accedere.
            wrapping_add: somma con wrap-around in caso di overflow,
            evitando un panic in debug mode o UB in release mode.
        */
    }
    acc
}

fn sum_unsafe(data: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for i in 0..data.len() {
        acc = acc.wrapping_add(unsafe { *data.get_unchecked(i) });
        // metto il codice unsafe e poi get_unchecked(i): accesso diretto senza bounds check.
    }
    acc
}     

fn sum_iter(data: &[u64]) -> u64 {
    data.iter().fold(0u64, |acc, &x| acc.wrapping_add(x))
    /*  data.iter(): crea un iteratore sugli elementi di data.
        fold(0u64, |acc, &x| ...): accumula i valori partendo da 0,
        applicando l'equivalente di una lambda function |paramentri| corpo.
        |acc, &x| acc.wrapping_add(x) a ogni elemento.
    */
}

fn bench<F: Fn(&[u64]) -> u64>(label: &str, f: F, data: &[u64]) -> f64 {
/*  Funzione generica: F è un tipo che implementa Fn(&[u64]) -> u64
    Questo permette di passare sum_safe, sum_unsafe o sum_iter senza duplicare il codice di misurazione.
*/
    let mut times: Vec<f64> = Vec::with_capacity(ITERATIONS as usize);
    // Vettore che raccoglie i tempi di ogni iterazione, pre-allocato con la capacità esatta per evitare riallocazioni
    for _ in 0..ITERATIONS {
        let start = Instant::now();                         // Salva l'istante corrente — avvia il cronometro.

        let result = f(black_box(data));                    // Esegue la funzione da misurare. black_box(data): impedisce al compilatore di ottimizzare
        let elapsed = start.elapsed();                      // Calcola il tempo trascorso dall'avvio del cronometro.
        black_box(result);                                  // Impedisce al compilatore di eliminare il calcolo di result
        times.push(elapsed.as_secs_f64() * 1000.0);         // Converte la durata in millisecondi e la aggiunge al vettore.
    }

    let mean = times.iter().sum::<f64>() / times.len() as f64;
    let variance = times.iter().map(|t| (t - mean).powi(2)).sum::<f64>()
                   / times.len() as f64;
    let stddev = variance.sqrt();
    println!("{:<12} media: {:7.2} ms   stddev: {:5.2} ms", label, mean, stddev);
    mean
    }

fn main() {
    let data: Vec<u64> = (0..ARRAY_SIZE as u64).collect();
    // Alloca l'array e lo riempie con i valori 0, 1, 2, ..., 99999999. collect() consuma l'iteratore e costruisce il Vec.
    let _ = black_box(sum_safe(&data));
    /*  Warm-up: una passata ignorata prima delle misurazioni reali. Serve a portare i dati nella cache della CPU, in modo che
        la prima misurazione reale non sia penalizzata da cache miss.
    */
    let t_safe   = bench("safe",   sum_safe,   &data);
    let t_unsafe = bench("unsafe", sum_unsafe, &data);
    let t_iter   = bench("iter",   sum_iter,   &data);

    let overhead_pct = (t_safe - t_unsafe) / t_unsafe * 100.0;
    let overhead_iter = (t_iter - t_unsafe) / t_unsafe * 100.0;
}