use crate::profile::*;

const LETTERS_NOT_IN_NUMBER_WORDS: [char; 10] = ['a', 'b', 'c', 'd', 'j', 'k', 'm', 'p', 'q', 'z'];

pub fn initial_text_constants(string: &str, number_profiles: &[Profile; 51]) -> [usize; 26] {
    let mut constants = [1; 26]; // Add one of each letter.

    let lowercase = string.to_lowercase();
    let letters = lowercase.chars().filter(|c| c.is_ascii_alphabetic());

    for letter in letters {
        constants[(letter as u8 - b'a') as usize] += 1; // Add the letters from the string.
    }

    for letter in LETTERS_NOT_IN_NUMBER_WORDS.iter() {
        let count = constants[(*letter as u8 - b'a') as usize];
        constants = &number_profiles[count] + &constants;
    }

    constants
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::number_profiles::*;

    #[test]
    fn it_calculates_the_initial_text_constants_from_a_string() {
        let profiles = number_profiles();
        let constants = initial_text_constants("This pangram contains and", &profiles);

                            // a  b  c  d  e  f  g  h  i  j  k  l  m  n   o   p  q  r  s  t  u  v  w  x  y  z
        assert_eq!(constants, [5, 1, 2, 2, 7, 2, 2, 2, 4, 1, 1, 1, 2, 10, 11, 2, 1, 2, 8, 7, 1, 2, 5, 1, 1, 1]);

        // Note that this example is from page 5 of the paper:
        // https://www.leesallows.com/files/In%20Quest%20of%20a%20Pangram1.pdf#page=5
        //
        // However, it excludes plurals since they are added by the algorithm.
        // These can't be pre-determined now because some ranges start from 1.
    }
}
