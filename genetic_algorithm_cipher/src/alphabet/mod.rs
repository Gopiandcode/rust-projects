extern crate rand;

use self::rand::Rng;
use ::std::fmt::{
    Display,
    Formatter,
};
use ::std::fmt;

pub struct SubstitutionCipher {
    mapping: [u8; 26]
}

impl SubstitutionCipher {
    pub fn new() -> SubstitutionCipher {
       let mut chars = [ 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, ];

       rand::thread_rng().shuffle(&mut chars);

       SubstitutionCipher {
        mapping: chars
       }
    }

    pub fn crossover(parent_a : &SubstitutionCipher, parent_b : &SubstitutionCipher) -> SubstitutionCipher {
        let mut chars : [u8; 26] = [0; 26];
        let mut copied : [bool; 26] = [false; 26];
        let mut not_moved : Vec<usize> = vec!();

        let mut rng = rand::thread_rng();
        let crossover : usize = rng.gen::<usize>() % 26usize;
        println!("Crossing over at {}", crossover);

        for i in 0..26 {
            if i < crossover {
                chars[i] = parent_a.mapping[i];
                copied[(parent_a.mapping[i] - b'a') as usize] = true;
            } 
        }

        for i in crossover..26 {
            let b_char = parent_b.mapping[i];
            if !copied[(b_char - b'a') as usize] {
                chars[i] = b_char;
                copied[(b_char - b'a') as usize]  = true;
            } else {
                not_moved.push(i);
            }
        }

        for i in 0..26 {
            if !copied[i] {
                let index = not_moved.pop().unwrap();

                chars[index] = b'a' + i as u8;
            }
        }

        SubstitutionCipher {
            mapping: chars
        }
    }

    pub fn clone(&self) -> Self {
       let mut chars = [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];

       for i in 0..self.mapping.len() {
            chars[i] = self.mapping[i];
       }

       SubstitutionCipher {
        mapping: chars
       }
 
    }

    pub fn mutate(&mut self) {
        let positonA : usize = rand::thread_rng().gen::<usize>() % 26;
        let characterA_old = self.mapping[positonA];
        let characterA_new = self.mapping[(characterA_old - b'a') as usize];
       
        self.mapping[positonA] = characterA_new;
        self.mapping[(characterA_old - b'a') as usize] = characterA_old;
    }


    pub fn apply(&self, text : &String) -> String {
        let mut result : String = String::new();

        for character in text.chars() {
            let char_value = character as u8;
            if char_value  >=  b'a' && char_value - b'a'  < 26 {
                result.push(self.mapping[(char_value - b'a') as usize] as char);
            } else {
                result.push(character);
            }
        }

        result
    }
}

impl Display for SubstitutionCipher {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (index, character) in (0..26).zip((b'a'..b'z'+1)) {
           write!(f, "{}: {}, ", character as char, self.mapping[index] as char);
        }
        write!(f, "\n")
    }
}
