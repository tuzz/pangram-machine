use std::simd::Simd;
use std::ops::{Index, IndexMut, Add};

#[derive(Clone, Copy)]
pub struct Profile {
    pub vector: Simd<u8, 16>,
}

impl Profile {
    pub fn from_array(array: [u8; 16]) -> Self {
        Self { vector: Simd::from_array(array) }
    }

    pub fn to_array(self) -> [u8; 16] {
        self.vector.to_array()
    }

    pub fn from(array: &[usize; 26]) -> Self {
        let mut profile = Self::from_array([0; 16]);

        profile['e'] = array[4] as u8;
        profile['f'] = array[5] as u8;
        profile['g'] = array[6] as u8;
        profile['h'] = array[7] as u8;
        profile['i'] = array[8] as u8;
        profile['l'] = array[11] as u8;
        profile['n'] = array[13] as u8;
        profile['o'] = array[14] as u8;
        profile['r'] = array[17] as u8;
        profile['s'] = array[18] as u8;
        profile['t'] = array[19] as u8;
        profile['u'] = array[20] as u8;
        profile['v'] = array[21] as u8;
        profile['w'] = array[22] as u8;
        profile['x'] = array[23] as u8;
        profile['y'] = array[24] as u8;

        profile
    }

    pub fn with(self, initial_constants: &[usize; 26]) -> [usize; 26] {
        [
            initial_constants[0],
            initial_constants[1],
            initial_constants[2],
            initial_constants[3],
            self['e'] as usize,
            self['f'] as usize,
            self['g'] as usize,
            self['h'] as usize,
            self['i'] as usize,
            initial_constants[9],
            initial_constants[10],
            self['l'] as usize,
            initial_constants[12],
            self['n'] as usize,
            self['o'] as usize,
            initial_constants[15],
            initial_constants[16],
            self['r'] as usize,
            self['s'] as usize,
            self['t'] as usize,
            self['u'] as usize,
            self['v'] as usize,
            self['w'] as usize,
            self['x'] as usize,
            self['y'] as usize,
            initial_constants[25],
        ]
    }
}

impl Index<char> for Profile {
    type Output = u8;

    fn index(&self, letter: char) -> &Self::Output {
        match letter {
            'e' => &self.vector[0],
            'f' => &self.vector[1],
            'g' => &self.vector[2],
            'h' => &self.vector[3],
            'i' => &self.vector[4],
            'l' => &self.vector[5],
            'n' => &self.vector[6],
            'o' => &self.vector[7],
            'r' => &self.vector[8],
            's' => &self.vector[9],
            't' => &self.vector[10],
            'u' => &self.vector[11],
            'v' => &self.vector[12],
            'w' => &self.vector[13],
            'x' => &self.vector[14],
            'y' => &self.vector[15],

            _ => unimplemented!("{}", letter),
        }
    }
}

impl IndexMut<char> for Profile {
    fn index_mut(&mut self, letter: char) -> &mut u8 {
        match letter {
            'e' => &mut self.vector[0],
            'f' => &mut self.vector[1],
            'g' => &mut self.vector[2],
            'h' => &mut self.vector[3],
            'i' => &mut self.vector[4],
            'l' => &mut self.vector[5],
            'n' => &mut self.vector[6],
            'o' => &mut self.vector[7],
            'r' => &mut self.vector[8],
            's' => &mut self.vector[9],
            't' => &mut self.vector[10],
            'u' => &mut self.vector[11],
            'v' => &mut self.vector[12],
            'w' => &mut self.vector[13],
            'x' => &mut self.vector[14],
            'y' => &mut self.vector[15],

            _ => unimplemented!("{}", letter),
        }
    }
}

impl Add for &Profile {
    type Output = Profile;

    fn add(self, other: Self) -> Self::Output {
        Profile { vector: self.vector + other.vector }
    }
}

impl Add<&[usize; 26]> for &Profile {
    type Output = [usize; 26];

    fn add(self, other: &[usize; 26]) -> Self::Output {
        let mut sum = *other;

        sum[4] += self['e'] as usize;
        sum[5] += self['f'] as usize;
        sum[6] += self['g'] as usize;
        sum[7] += self['h'] as usize;
        sum[8] += self['i'] as usize;
        sum[11] += self['l'] as usize;
        sum[13] += self['n'] as usize;
        sum[14] += self['o'] as usize;
        sum[17] += self['r'] as usize;
        sum[18] += self['s'] as usize;
        sum[19] += self['t'] as usize;
        sum[20] += self['u'] as usize;
        sum[21] += self['v'] as usize;
        sum[22] += self['w'] as usize;
        sum[23] += self['x'] as usize;
        sum[24] += self['y'] as usize;

        sum
    }
}
