pub fn pangram_machine<F: FnMut([usize; 26])>(initial_constants: &[usize; 26], profiles: &[[usize; 26]; 51], on_solution: F) {

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::number_profiles::*;
    use crate::initial_text_constants::*;

    #[test]
    fn it_can_find_lee_sallows_first_ever_self_enumerating_pangram() {
        let profiles = number_profiles();
        let initial_constants = initial_text_constants("This pangram lists and", &profiles);

        let mut solutions = vec![];
        pangram_machine(&initial_constants, &profiles, |solution| solutions.push(solution));

        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]); // TODO
    }
}
