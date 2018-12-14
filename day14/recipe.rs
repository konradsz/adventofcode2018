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

    let expected = &[6, 3, 5, 0, 4, 1];
    let expected_len = expected.len();

    loop {
        let mut new_recipe = recipes[current_recipe_1] + recipes[current_recipe_2];
        let mut two_recipes_pushed = false;
        if new_recipe > 9 {
            recipes.push(1);
            new_recipe %= 10;
            two_recipes_pushed = true;
        }

        recipes.push(new_recipe);

        current_recipe_1 += 1 + recipes[current_recipe_1];
        current_recipe_2 += 1 + recipes[current_recipe_2];

        current_recipe_1 %= recipes.len();
        current_recipe_2 %= recipes.len();

        if recipes.len() > expected_len {
            if &recipes[recipes.len() - expected_len..] == expected {
                println!("{}", recipes.len() - expected_len);
                break;
            }
        }

        if two_recipes_pushed && recipes.len() > expected_len + 1 {
            if &recipes[recipes.len() - expected_len - 1..recipes.len() - 1] == expected {
                println!("{}", recipes.len() - expected_len - 1);
                break;
            }
        }
    }
}

fn main() {
    part_1();
    part_2();
}
