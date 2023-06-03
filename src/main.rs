use std::{error::Error, fmt::Display};
use std::fs;

#[derive(Debug)]
struct Atom {
    id: String,
    coord: CoordCartesian,

    atomic_number: i32,
    electron_number: i32,
}

#[derive(Debug)]
struct CoordCartesian {
    x: f32,
    y: f32,
    z: f32
}

impl Atom {
    fn new(id: &str, coord: CoordCartesian) -> Atom {
        let atomic_number = match id {
            "H" => 1,
            "O" => 8,
            "C" => 6,
            _ => 0
        };

        Atom { id: id.to_owned(), coord: coord, atomic_number: atomic_number, electron_number: atomic_number }
    }
}

impl Display for Atom {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "Atom `{}` [Z:{}, N:{}] @ {}", self.id, self.atomic_number, self.electron_number, self.coord)
   } 
}

impl Display for CoordCartesian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}, {}, {}", self.x, self.y, self.z)
    }
}

fn parse_xyz(file: &str) -> Vec<Atom> {
    let m = "Malformed file.";
    let content = fs::read_to_string(file).expect("Cannot read input file.");

    let mut content_parts = content.split('\n');

    let number_of_atoms: i64 = content_parts.next()
        .expect("Did not find the number of atoms.")
        .trim().parse().expect("Could not parse number of atoms");

    let mut atoms: Vec<Atom> = Vec::with_capacity(number_of_atoms as usize);
    for line in content.split("\n").skip(2) {
        if line.trim().len() == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        atoms.push(
            Atom::new(
                parts[0],
                CoordCartesian {
                    x: parts[1].parse().expect(m), y: parts[2].parse().expect(m), z: parts[3].parse().expect(m)
                }
            )
        );
    }

    atoms

}

fn main() -> Result<(), Box<dyn Error>> {

    let atoms = parse_xyz("/home/hedmad/Files/repos/hfrs/prot_solv.xyz");

    for atom in atoms.iter() {
        println!("{}", atom)
    }

    println!("Done!");

    Ok(())
}
