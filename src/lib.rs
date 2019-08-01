use rand::prelude::*;
use regex::Regex;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn owo() {
        let text = String::from("malfunction me mom.. t-till i break~~");
        let owoified = text.owoify();
        println!("\t\t{}", owoified);
    }
    #[test]
    fn all_match_owo() {
        let text = String::from("r l R L na Na NA ove !!");
        println!("\t\t{}", text.owoify());
    }
}

pub trait OwOifiable {
    /// The owoification method
    fn owoify(&self) -> Self;
}

impl OwOifiable for String {
    fn owoify(&self) -> Self {
        let mut rng = rand::thread_rng();
        let faces = ["(・`ω´・)", ";;w;;", "owo", "UwU", ">w<", "^w^"];
        let face = &format!(" {} ", faces[rng.gen_range(0, faces.len())]).to_owned();
        let pats: Vec<(&str, &str)> = vec![
            ("(?:r|l)", "w"),
            ("(?:R|L)", "W"),
            ("n([aeiou])", "ny$1"),
            ("N([aeiou])", "Ny$1"),
            ("N([AEIOU])", "Ny$1"),
            ("ove", "uv"),
            ("!+", face),
        ];

        let mut owoified = String::from_str(&self).unwrap();

        for &(f, t) in &pats {
            let re = Regex::new(f).unwrap();
            owoified = re.replace_all(&owoified, t).to_string();
        }

        owoified
    }
}
