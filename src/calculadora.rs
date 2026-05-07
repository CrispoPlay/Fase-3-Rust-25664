pub struct Calculadora;

impl Calculadora {
    pub fn sumar(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn restar(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn multiplicar(a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn dividir(a: f64, b: f64) -> f64 {
        if b != 0.0 {
            a / b
        } else {
            println!("No se puede dividir entre 0");
            0.0
        }
    }
}