mod calculadora; // importamos el módulo

use calculadora::Calculadora;
use std::io;

// función auxiliar
fn leer_numero() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input.trim().parse().unwrap_or(0.0)
}

fn main() {
    loop {
        println!("\n--- CALCULADORA ---");
        println!("1. Sumar");
        println!("2. Restar");
        println!("3. Multiplicar");
        println!("4. Dividir");
        println!("5. Salir");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error");

        let opcion = opcion.trim();

        if opcion == "5" {
            println!("Gracias por usar el programa");
            break;
        }

        println!("Ingrese el primer número:");
        let a = leer_numero();

        println!("Ingrese el segundo número:");
        let b = leer_numero();

        let resultado = match opcion {
            "1" => Calculadora::sumar(a, b),
            "2" => Calculadora::restar(a, b),
            "3" => Calculadora::multiplicar(a, b),
            "4" => Calculadora::dividir(a, b),
            _ => {
                println!("Opción inválida");
                continue;
            }
        };

        println!("Resultado: {}", resultado);
    }
}