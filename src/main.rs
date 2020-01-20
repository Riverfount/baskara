use std::io;


fn calcula_delta(a: f64, b: f64, c: f64) -> f64 {
    ((b * b) - 4.0 * a * c).into()
}

fn main() {

    println!("********************************************");
    println!("********************************************");
    println!("********************************************\n");
    println!("** Soluciona uma equação do segundo grau! **\n");
    println!("********************************************");
    println!("********************************************");
    println!("********************************************\n");
    println!("Digite o valor de a:");
    
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Falha ao ler entrada");
    let a: f64 = a.trim().parse().expect("Por favor, digite um número!");


    println!("Digite o valor de b:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Falha ao ler entrada");
    let b: f64 = b.trim().parse().expect("Por favor, digite um número!");

    println!("Digite o valor de c:");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Falha ao ler entrada");
    let c: f64 = c.trim().parse().expect("Por favor, digite um número!");

    println!("");


    let delta = calcula_delta(a, b, c);
    let raiz_delta = delta.sqrt();
    
    if raiz_delta.is_nan() {
        println!("Não há raiz no conjunto dos Reais");
    } else {
        let x1 = (-b + raiz_delta)/(2.0 * a);
        let x2 = (-b - raiz_delta)/(2.0 * a);
        println!("Para a equação ax2 + bx + c = 0");
        println!("Cujos valores são: a = {}, b = {} e c = {}", a, b, c);
        println!("As raizes são: x1 = {} e x2 = {}", x1, x2);
    }
}