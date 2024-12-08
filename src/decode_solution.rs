use crate::number_profiles::*;

pub fn decode_solution(initial_text: &str, solution: &[usize; 26]) -> String {
    let (prefix, final_delimiter) = initial_text
        .rsplit_once(" and").map(|(prefix, _)| (prefix, " and "))
        .or_else(|| initial_text.rsplit_once(" &").map(|(prefix, _)| (prefix, " & ")))
        .unwrap_or((initial_text, ", "));

    let mut sentence = prefix.to_string();
    sentence.push(' ');

     for (i, count) in solution.iter().enumerate().take(26) {
        let number_word = NUMBER_WORDS[*count];

        let letter = (b'a' + i as u8) as char;
        let plural = if *count > 1 { "'s" } else { "" };

        let delimiter = match i { 24 => final_delimiter, 25 => "" , _ => ", " };
        let term = format!("{} {}{}{}", number_word, letter, plural, delimiter);

        sentence.push_str(&term);
    }

    sentence.push('.');
    sentence
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_decode_the_solution_to_the_first_ever_self_enumerating_pangram() {
        let initial_text = "This pangram lists and";
        let solution =  [4, 1, 1, 2, 29, 8, 3, 5, 11, 1, 1, 3, 2, 22, 15, 2, 1, 7, 26, 19, 4, 5, 9, 2, 4, 1]; // page 18
        let decoded = decode_solution(initial_text, &solution);

        assert_eq!(decoded, [
            "This pangram lists four a's, one b, one c, two d's, twenty-nine e's,",
            "eight f's, three g's, five h's, eleven i's, one j, one k, three l's,",
            "two m's, twenty-two n's, fifteen o's, two p's, one q, seven r's,",
            "twenty-six s's, nineteen t's, four u's, five v's, nine w's, two x's,",
            "four y's and one z."
        ].join(" "));
    }
}
