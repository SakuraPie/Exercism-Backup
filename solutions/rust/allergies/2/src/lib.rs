pub struct Allergies {
    pub score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let all = self.allergies();
        all.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut bin_arr: Vec<u32> = vec![];
        let mut all: Vec<Allergen> = vec![];
        let mut current = self.score;
        while current / 2 != 0 {
            bin_arr.push(current % 2);
            current /= 2;
        }
        if self.score >= 1 {
            bin_arr.push(1);
        } else {
            bin_arr.push(0);
        }
        while bin_arr.len() < 8 {
            bin_arr.push(0);
        }
        bin_arr = bin_arr[0..8].to_owned();
        for (i, n) in bin_arr.iter().enumerate() {
            if *n == 1 {
                if i == 0 {
                    all.push(Allergen::Eggs);
                }
                if i == 1 {
                    all.push(Allergen::Peanuts);
                }
                if i == 2 {
                    all.push(Allergen::Shellfish);
                }
                if i == 3 {
                    all.push(Allergen::Strawberries);
                }
                if i == 4 {
                    all.push(Allergen::Tomatoes);
                }
                if i == 5 {
                    all.push(Allergen::Chocolate);
                }
                if i == 6 {
                    all.push(Allergen::Pollen);
                }
                if i == 7 {
                    all.push(Allergen::Cats);
                }
            }
        }
        all
    }
}