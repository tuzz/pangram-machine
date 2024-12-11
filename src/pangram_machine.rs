use crate::profile::*;

pub fn pangram_machine<F: FnMut([usize; 26])>(initial_constants: &[usize; 26], profiles: &[Profile; 51], mut on_solution: F) {
    let sumprofile = Profile::from(initial_constants);

    // Iterate through every combination within the specified ranges.
    for e_counter in 25..=32 {
        let sumprofile = &sumprofile + &profiles[e_counter];
    for f_counter in 4..=9 {
        let sumprofile = &sumprofile + &profiles[f_counter];
    for h_counter in 3..=8 {
        let sumprofile = &sumprofile + &profiles[h_counter];
    for i_counter in 8..=14 {
        let sumprofile = &sumprofile + &profiles[i_counter];
    for n_counter in 17..=23 {
        let sumprofile = &sumprofile + &profiles[n_counter];
    for o_counter in 12..=17 {
        let sumprofile = &sumprofile + &profiles[o_counter];
    for r_counter in 3..=8 {
        let sumprofile = &sumprofile + &profiles[r_counter];
    for s_counter in 24..=30 {
        let sumprofile = &sumprofile + &profiles[s_counter];
    for t_counter in 18..=24 {
        let sumprofile = &sumprofile + &profiles[t_counter];
    for u_counter in 1..=6 {
        let sumprofile = &sumprofile + &profiles[u_counter];
    for v_counter in 3..=8 {
        let sumprofile = &sumprofile + &profiles[v_counter];
    for w_counter in 7..=13 {
        let sumprofile = &sumprofile + &profiles[w_counter];

        // Detect the values for g, l, x, y based on the window-detectors optimisation.
        let detected_g = sumprofile['g'];
        if !(2..=7).contains(&detected_g) || detected_g == 6 { continue; }

        let detected_l = sumprofile['l'];
        if detected_l > 4 { continue; }

        let detected_x = sumprofile['x'];
        if detected_x > 5 { continue; }

        let detected_y = sumprofile['y'];
        if !(3..=5).contains(&detected_y) { continue; }

        // Calculate the final sumprofile.
        let sumprofile = &sumprofile + &profiles[detected_g as usize];
        let sumprofile = &sumprofile + &profiles[detected_l as usize];
        let sumprofile = &sumprofile + &profiles[detected_x as usize];
        let sumprofile = &sumprofile + &profiles[detected_y as usize];

        // Check if the claimed numbers balance with the true numbers.
        let [e, f, _g, h, i, _l, n, o, r, s, t, u, v, w, _x, _y] = sumprofile.to_array();

        if e_counter as u8 != e { continue; }
        if f_counter as u8 != f { continue; }
        if h_counter as u8 != h { continue; }
        if i_counter as u8 != i { continue; }
        if n_counter as u8 != n { continue; }
        if o_counter as u8 != o { continue; }
        if r_counter as u8 != r { continue; }
        if s_counter as u8 != s { continue; }
        if t_counter as u8 != t { continue; }
        if u_counter as u8 != u { continue; }
        if v_counter as u8 != v { continue; }
        if w_counter as u8 != w { continue; }

        on_solution(sumprofile.with(initial_constants));
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

                               // a  b  c  d  e   f  g  h  i   j  k  l  m  n   o   p  q  r  s   t   u  v  w  x  y  z
        assert_eq!(solutions[0], [4, 1, 1, 2, 29, 8, 3, 5, 11, 1, 1, 3, 2, 22, 15, 2, 1, 7, 26, 19, 4, 5, 9, 2, 4, 1]); // page 18
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn it_can_find_lee_sallows_second_ever_self_enumerating_pangram() {
        let profiles = number_profiles();
        let initial_constants = initial_text_constants("This pangram contains &", &profiles);

        let mut solutions = vec![];
        pangram_machine(&initial_constants, &profiles, |solution| solutions.push(solution));

                               // a  b  c  d  e   f  g  h  i   j  k  l  m  n   o   p  q  r  s   t   u  v  w  x  y  z
        assert_eq!(solutions[0], [4, 1, 2, 1, 30, 6, 5, 7, 11, 1, 1, 2, 2, 18, 15, 2, 1, 5, 27, 18, 2, 7, 8, 2, 3, 1]); // page 18
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn it_can_find_lee_sallows_first_bimagic_self_enumerating_pangram() {
        let profiles = number_profiles();
        let initial_constants = initial_text_constants("This autogram contains and", &profiles);

        let mut solutions = vec![];
        pangram_machine(&initial_constants, &profiles, |solution| solutions.push(solution));

                               // a  b  c  d  e   f  g  h  i   j  k  l  m  n   o   p  q  r  s   t   u  v  w  x  y  z
        assert_eq!(solutions[0], [5, 1, 2, 2, 26, 6, 2, 4, 13, 1, 1, 1, 2, 21, 16, 1, 1, 5, 27, 20, 3, 6, 9, 5, 5, 1]); // page 23
        assert_eq!(solutions[1], [5, 1, 2, 2, 31, 5, 5, 8, 12, 1, 1, 2, 2, 18, 16, 1, 1, 6, 27, 21, 3, 7, 8, 3, 4, 1]); // page 22
        assert_eq!(solutions.len(), 2);
    }
}
