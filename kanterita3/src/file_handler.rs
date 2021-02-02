use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};
use crate::{models::CreatePerson};


pub fn handle() {

    // See Issue #6 
    if let Ok(lines) = read_lines("./tmp/input.csv") {
        
        //Read and process file
        for line in lines {
            if let Ok(line) = line {

            	//split line
            	let line = line.split(";");
            	let mut vec = line.collect::<Vec<&str>>();


            	validate(&mut vec);

            	thread::sleep(time::Duration::from_secs(1));
                //println!("{}", line);
            }
        }
        println!("File processed");
    }else {
    	println!("Could not read lines");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn validate (vec: &mut Vec<&str>){

	println!("Validating");

	let mut vec = vec;

	validate_identificacion(vec[0].to_string());
	validate_nombre(vec[1].to_string());
	validate_genero(vec[2].to_string());
	validate_estado_civil(vec[3].to_string());
	validate_fecha_nacimiento(vec[4].to_string());
	validate_numero_telefono(vec[5].to_string().parse().expect("Invalid string"));
	validate_direccion(vec[6].to_string());
	validate_correo(vec[7].to_string());

	parse2json(&mut vec);


}


fn is_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn parse2json (vec: &mut Vec<&str>){

	let person = CreatePerson {
	        identificacion: vec[0].to_owned(),
	        nombre: vec[1].to_owned(),
	        genero: vec[2].to_owned(),
	        estado_civil: vec[3].to_owned(),
	        fecha_nacimiento: vec[4].to_owned(),
	        numero_telefono: vec[5].to_string().parse().expect("Invalid string"),
	        direccion: vec[6].to_owned(),
	        correo: vec[7].to_owned(),
	        validado: false,
	        observacion: vec[7].to_owned(),
	    };

	let j = serde_json::to_string(&person).unwrap();

	println!("{}", j);

	post_thingy(j);
	
}

fn validate_identificacion (identificacion: String) {
	
	if is_numeric(identificacion) {
		println!("Is numeric");
	}else {
		println!("Is NOT numeric");
	}

}

fn validate_nombre (_nombre: String) {
	//println!("{}", nombre );
}


fn validate_genero (_genero: String) {
	//println!("{}", genero );
}


fn validate_estado_civil (_estado_civil: String) {
	//println!("{}", estado_civil );
}


fn validate_fecha_nacimiento (_fecha_nacimiento: String) {
	//println!("{}", fecha_nacimiento );
}


fn validate_numero_telefono (_numero_telefono: i64) {
	//println!("{}", numero_telefono );
}


fn validate_direccion (_direccion: String) {
	//println!("{}", direccion );
}


fn validate_correo (_correo: String) {
	//println!("{}", correo );
}

/*
fn validate_observacion (observacion: String) {
	println!("{}", observacion );
}
*/

fn post_thingy (_json: String) {
	//POST method using the parsed json
}
