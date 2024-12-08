use crate::pairwise_sum::*;

const LETTERS_NOT_IN_NUMBER_WORDS: [char; 10] = ['a', 'b', 'c', 'd', 'j', 'k', 'm', 'p', 'q', 'z'];

pub fn initial_text_constants(string: &str, profiles: &[[usize; 26]; 51]) -> [usize; 26] {
    let mut profile = [1; 26]; // Add one of each letter.

    let lowercase = string.to_lowercase();
    let letters = lowercase.chars().filter(|c| c.is_ascii_alphabetic());

    for letter in letters {
        profile[(letter as u8 - b'a') as usize] += 1; // Add the letters from the string.
    }

    for letter in LETTERS_NOT_IN_NUMBER_WORDS.iter() {
        let count = profile[(*letter as u8 - b'a') as usize];
        profile = pairwise_sum(&profile, &profiles[count]); // Add letters from already-known number words.
    }

    for letter in LETTERS_NOT_IN_NUMBER_WORDS.iter() {
        let is_plural = profile[(*letter as u8 - b'a') as usize] != 1;

        if is_plural {
            profile[18] += 1; // Add a plural 's' is the already-known number is not 'one'.
        }
    }

    profile
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::number_profiles::*;

    #[test]
    fn it_calculates_the_initial_text_constants_from_a_string() {
        let profiles = number_profiles();
        let constants = initial_text_constants("This pangram contains and", &profiles);

                            // a  b  c  d  e  f  g  h  i  j  k  l  m  n   o   p  q  r  s   t  u  v  w  x  y  z
        assert_eq!(constants, [5, 1, 2, 2, 7, 2, 2, 2, 4, 1, 1, 1, 2, 10, 11, 2, 1, 2, 13, 7, 1, 2, 5, 1, 1, 1]);

        // Note that this example is from page 5 of the paper:
        // https://www.leesallows.com/files/In%20Quest%20of%20a%20Pangram1.pdf#page=5
        //
        // However, it excludes the plurals for letter in the number words.
        // These can't be pre-determined because some of the ranges start from
        // 1 and could mean no plural is added. The algorithm adds them later.
    }
}
