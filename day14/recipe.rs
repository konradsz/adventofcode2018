fn part_1() {
    let mut recipes = vec![3, 7];
    let mut current_recipe_1 = 0;
    let mut current_recipe_2 = 1;

    const NUMBER_OF_RECIPES: usize = 635041;
    while recipes.len() < NUMBER_OF_RECIPES + 10 {
        let mut new_recipe = recipes[current_recipe_1] + recipes[current_recipe_2];
        if new_recipe > 9 {
            recipes.push(1);
            new_recipe %= 10;
        }

        recipes.push(new_recipe);

        current_recipe_1 += 1 + recipes[current_recipe_1];
        current_recipe_2 += 1 + recipes[current_recipe_2];

        current_recipe_1 %= recipes.len();
        current_recipe_2 %= recipes.len();
    }

    recipes
        .iter()
        .skip(NUMBER_OF_RECIPES)
        .take(10)
        .for_each(|recipe| print!("{}", recipe));
    println!("");
}

fn part_2() {
    let mut recipes = vec![3, 7];
    let mut current_recipe_1 = 0;
    let mut current_recipe_2 = 1;

    let expected = "635041";
    let expected_len = expected.len();
    let mut i = 1;
    loop {
        let new_recipe = recipes[current_recipe_1] + recipes[current_recipe_2];
        if new_recipe > 9 {
            recipes.push(1);
            recipes.push(new_recipe % 10);
            i += 2;
        } else {
            recipes.push(new_recipe);
            i += 1;
        }

        current_recipe_1 += 1 + recipes[current_recipe_1];
        current_recipe_2 += 1 + recipes[current_recipe_2];

        current_recipe_1 %= recipes.len();
        current_recipe_2 %= recipes.len();

        if recipes.len() >= expected_len + 1 {
            let recipe = recipes
                .iter()
                .skip(recipes.len() - expected_len)
                .take(expected_len)
                .map(|recipe| std::char::from_digit(*recipe as u32, 10).unwrap())
                .collect::<String>();
            if recipe == expected {
                println!("{}", i - expected_len + 1);
                break;
            }

            let recipe = recipes
                .iter()
                .skip(recipes.len() - expected_len - 1)
                .take(expected_len + 1)
                .map(|recipe| std::char::from_digit(*recipe as u32, 10).unwrap())
                .collect::<String>();
            if recipe[..recipe.len() - 1] == *expected {
                println!("{}", i - expected_len);
                break;
            }
        }
    }
}

fn main() {
    part_1();
    part_2();
}
