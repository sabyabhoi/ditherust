use std::{path::Path, fs::File, io::Read};

use image::{imageops::colorops, Rgb};
use serde::{Deserialize, Serialize};
use color_eyre::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme(pub Vec<[u8; 3]>);

impl Theme {
    pub fn read_from_file(path: &Path) -> Result<Self> {
	let mut file = File::open(path)?;
	let mut raw_data = String::new();

	file.read_to_string(&mut raw_data)?;

	Ok(serde_json::from_str(&raw_data)?)
    }
}

impl colorops::ColorMap for Theme {
    type Color = Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let mut ans: usize = self.0.len();
        let mut min_diff: u16 = u16::max_value();
        for (i, val) in self.0.iter().enumerate() {
            let diff: u16 = (val[0] as u16).abs_diff(color[0] as u16)
                + (val[1] as u16).abs_diff(color[1] as u16)
                + (val[2] as u16).abs_diff(color[2] as u16);
            if min_diff >= diff {
                ans = i;
                min_diff = diff;
            }
        }
        ans
    }

    fn map_color(&self, color: &mut Self::Color) {
        *color = Rgb(self.0[self.index_of(color)]);
    }
}

#[cfg(test)]
mod tests {
    use image::{imageops::ColorMap, Rgb};

    use super::*;

    #[test]
    fn theme_read_file() {
	assert!(true);
    }

    #[test]
    fn theme_index_of_test() {
        let t = Theme(vec![
            [0, 0, 0],
            [100, 100, 100],
            [200, 200, 200],
        ]);
        assert_eq!(t.index_of(&Rgb([100, 100, 100])), 1);
    }
}
