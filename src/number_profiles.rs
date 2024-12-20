use crate::profile::*;

pub const NUMBER_WORDS: [&str; 51] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "seventeen", "eighteen", "nineteen", "twenty", "twenty-one",
    "twenty-two", "twenty-three", "twenty-four", "twenty-five", "twenty-six",
    "twenty-seven", "twenty-eight", "twenty-nine", "thirty", "thirty-one",
    "thirty-two", "thirty-three", "thirty-four", "thirty-five", "thirty-six",
    "thirty-seven", "thirty-eight", "thirty-nine", "forty", "forty-one",
    "forty-two", "forty-three", "forty-four", "forty-five", "forty-six",
    "forty-seven", "forty-eight", "forty-nine", "fifty"
];

pub fn number_profiles() -> [Profile; 51] {
    let mut profiles = [Profile::from_array([0; 16]); 51];

    for (i, number_word) in NUMBER_WORDS.iter().enumerate() {
        let letters = number_word.chars().filter(|c| c.is_ascii_alphabetic());
        let profile = &mut profiles[i];

        for letter in letters {
            profile[letter] += 1;
        }

        if i != 1 {
            profile['s'] += 1; // Add the plural 's' except for 'one'.
        }
    }

    profiles
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_contains_a_tally_of_letters_at_each_index_including_the_plural_s_letter() {
        let profiles = number_profiles();

                                          // e  f  g  h  i  l  n  o  r  s  t  u  v  w  x  y
        assert_eq!(profiles[1].to_array(),  [1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]); // one (no plural)
        assert_eq!(profiles[2].to_array(),  [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0]); // two
        assert_eq!(profiles[3].to_array(),  [2, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0]); // three
        assert_eq!(profiles[15].to_array(), [2, 2, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0]); // fifteen
        assert_eq!(profiles[23].to_array(), [3, 0, 0, 1, 0, 0, 1, 0, 1, 1, 3, 0, 0, 1, 0, 1]); // twenty-three
    }
}
