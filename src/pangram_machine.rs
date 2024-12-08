use crate::pairwise_sum::*;

pub fn pangram_machine<F: FnMut([usize; 26])>(initial_constants: &[usize; 26], profiles: &[[usize; 26]; 51], mut on_solution: F) {
    let sumprofile = initial_constants;

    // Iterate through every combination within the specified ranges.
    for e_counter in 25..=32 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[e_counter]);
    for f_counter in 4..=9 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[f_counter]);
    for h_counter in 3..=8 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[h_counter]);
    for i_counter in 8..=14 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[i_counter]);
    for n_counter in 17..=23 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[n_counter]);
    for o_counter in 12..=17 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[o_counter]);
    for r_counter in 3..=8 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[r_counter]);
    for s_counter in 24..=30 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[s_counter]);
    for t_counter in 18..=24 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[t_counter]);
    for u_counter in 1..=6 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[u_counter]);
    for v_counter in 3..=8 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[v_counter]);
    for w_counter in 7..= 13 {
        let sumprofile = pairwise_sum(&sumprofile, &profiles[w_counter]);

    // Detect the values for g, l, x, y based on the window-detectors optimisation.
    let detected_g = sumprofile[6];
    if detected_g < 2 || detected_g > 7 || detected_g == 6 { continue; }

    let detected_l = sumprofile[11];
    if detected_l > 4 { continue; }

    let detected_x = sumprofile[23];
    if detected_x > 5 { continue; }

    let detected_y = sumprofile[24];
    if detected_y < 3 || detected_y > 5 { continue; }

    // Calculate the final sumprofile.
    let sumprofile = pairwise_sum(&sumprofile, &profiles[detected_g]);
    let sumprofile = pairwise_sum(&sumprofile, &profiles[detected_l]);
    let sumprofile = pairwise_sum(&sumprofile, &profiles[detected_x]);
    let sumprofile = pairwise_sum(&sumprofile, &profiles[detected_y]);

    // Check if the claimed numbers balance with the true numbers.
    if e_counter != sumprofile[4] { continue; }
    if f_counter != sumprofile[5] { continue; }
    if h_counter != sumprofile[7] { continue; }
    if i_counter != sumprofile[8] { continue; }
    if n_counter != sumprofile[13] { continue; }
    if o_counter != sumprofile[14] { continue; }
    if r_counter != sumprofile[17] { continue; }
    if s_counter != sumprofile[18] { continue; }
    if t_counter != sumprofile[19] { continue; }
    if u_counter != sumprofile[20] { continue; }
    if v_counter != sumprofile[21] { continue; }
    if w_counter != sumprofile[22] { continue; }

    on_solution(sumprofile); // EUREKA!

    }}}}}}}}}}}}
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
        assert_eq!(solutions[0], [4, 1, 1, 2, 29, 8, 3, 5, 11, 1, 1, 3, 2, 22, 15, 2, 1, 7, 26, 19, 4, 5, 9, 2, 4, 1]);
    }
}
