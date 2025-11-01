use std::io; 
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let nb_secret = rand::thread_rng().gen_range(1..=100);
    println!("le nb secret est : {nb_secret} "); 

    loop{
        
        println!("saisi ta proposition :");

        let mut devine: String= String::new(); 

        io::stdin().read_line(&mut devine).expect("saisissez un nombre"); 

        let devine_nb: u32 = match devine.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("sATTENTION IL FAUT SAISIR UN NOMBRE "),

        };

        println!("nice try: {devine}");

            match devine_nb.cmp(&nb_secret) {

                Ordering::Less => println!("le nb saisi est plus petit que le nb secret "),
                Ordering::Greater => println!("le nb saisi est plus grand que le nb secret "),
                Ordering::Equal => {
                    println!("le nb saisi est egal au nb secret; YOU WIN!!!!");
                    break;
                }

            }
    }

}


