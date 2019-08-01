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
        println!("\t{}", owoified);
    }
    #[test]
    fn all_match_owo() {
        let text = String::from("r l R L na Na NA ove !!");
        println!("\t{}", text.owoify());
    }
}

pub trait OwOifiable {
    /// The owoification method
    fn owoify(&self) -> Self;
}

impl OwOifiable for String {
    /// Owoifies a String
    ///
    /// # Examples
    ///
    /// ```
    /// use use owoify::OwOifiable;
    /// let owoified = String::from("Example text").owoify();
    /// ```
    fn owoify(&self) -> Self {
        let mut rng = rand::thread_rng();
        let faces = ["(・`ω´・)", "OwO", "owo", "oωo", "òωó", "°ω°", "UwU", ">w<", "^w^"];
        let face = &format!(" {} ", faces[rng.gen_range(0, faces.len())]).to_owned();
        let pats: Vec<(&str, &str)> = vec![
            ("(?:r|l)", "w"),
            ("(?:R|L)", "W"),
            ("n([aeiou])", "ny$1"),
            ("N([aeiou])", "Ny$1"),
            ("N([AEIOU])", "NY$1"),
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
