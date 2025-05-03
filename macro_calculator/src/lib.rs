use json;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let cals_text = food.calories[1].clone();
        let cals_value = &cals_text[0..cals_text.len() - 4];
        cals += cals_value.parse::<f64>().unwrap() * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    let result = json::object! {
        cals: (cals * 100.0).round() / 100.0,
        carbs: (carbs * 100.0).round() / 100.0,
        proteins: (proteins * 100.0).round() / 100.0,
        fats: (fats * 100.0).round() / 100.0,
    };

    return result;
}
