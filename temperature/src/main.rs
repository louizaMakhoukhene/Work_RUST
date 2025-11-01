use std::env;

fn c_to_f(c:f64) -> f64{
        c * 9.0/5.0 + 32.0
    }

fn f_to_c(f: f64) -> f64{
    f - 32.0 * 5.0/9.0
}
fn main() {

    let arguments: Vec<String> = env::args().collect(); 
    //*on collecte en VECTEUR les areguments passé sur le terminal  */

    if arguments.len() != 3 {
        eprintln!("utilisation: cargo run -- <mode> <valeur>");
        return;
    }

    let mode= &arguments[1];     
    let valeur = &arguments[2];

    //*mode est valeur sont de type STRING cest pour ca que on va utiliser valeur.parse() pour le convertir en f64 */
    //*et on utilie mode.as_str() pour le convertir en str  */

    let val: f64= match valeur.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("valeur {} nest pas un nombre valide", valeur);
            return ;
        } 

    };

    let resultat_conversion = match mode.as_str(){

        "C" => {
            let f= c_to_f(val); 
            println!( "{val}°C = {f:.2}°F");
            f 
        }

        "F" => {
            let c= f_to_c(val);
            println!( "{val}°F= {c:.2}°C");
            c
        }

        _ => {
            eprintln!("mode inci=onnu utilisez c ou f ");
            return;

        }

    };




    
    


}
