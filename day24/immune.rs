use std::cmp::Ordering;

// TO REMOVE
#[derive(PartialEq)]
enum Army {
    ImmuneSystem,
    Infection
}

#[derive(PartialEq)]
enum Type {
    Radiation,
    Cold,
    Fire,
    Slashing,
    Bludgeoning,
}

struct Group {
    index: u32, // change it to id
    size: u32,
    hit_points: u32,
    attack_damage: u32,
    attack_type: Type,
    initiative: u32,
    weaknesses: Vec<Type>,
    immunities: Vec<Type>,
    selected_target: u32,
    selected: bool,
    damage_to_deal: u32
}

impl Group {
    fn new(
        index: u32,
        size: u32,
        hit_points: u32,
        attack_damage: u32,
        attack_type: Type,
        initiative: u32,
        weaknesses: Vec<Type>,
        immunities: Vec<Type>,
    ) -> Group {
        Group {
            index,
            size,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
            selected_target: 0,
            selected: false,
            damage_to_deal: 0 // REMOVE
        }
    }
}

fn calculate_damage(attacking_group: &Group, defending_group: &Group) -> u32 {
    let mut damage = attacking_group.size * attacking_group.attack_damage;
    if defending_group.weaknesses.contains(&attacking_group.attack_type) {
        damage *= 2;
    } else if defending_group.immunities.contains(&attacking_group.attack_type) {
        damage = 0;
    }
    damage
}

fn print_armies_state(immune_system_army: &Vec<Group>, infection_army: &Vec<Group>) {
    println!("Immune System:");
    for group in immune_system_army {
        println!("Group {} contains {} units", group.index, group.size);
    }
    println!("Infection:");
    for group in infection_army {
        println!("Group {} contains {} units", group.index, group.size);
    }
}

fn select_targets(attacking_army: &mut Vec<Group>, defending_army: &mut Vec<Group>, attacking_type: Army) {
    for attacking_group in attacking_army.iter_mut() {
        let mut selected_targets = Vec::new();

        for defending_group in defending_army.iter().filter(|group| !group.selected) {
            /*let mut damage = attacking_group.size * attacking_group.attack_damage;
            if defending_group.weaknesses.contains(&attacking_group.attack_type) {
                damage *= 2;
            } else if defending_group.immunities.contains(&attacking_group.attack_type) {
                damage = 0;
            }*/
            let damage = calculate_damage(&attacking_group, &defending_group);
            if attacking_type == Army::Infection {
                println!("Infection group {} would deal defending group {} {} damage", attacking_group.index, defending_group.index, damage);
            } else {
                println!("Immune System group {} would deal defending group {} {} damage", attacking_group.index, defending_group.index, damage);
            }

            if damage > attacking_group.damage_to_deal {
                selected_targets.clear();
                attacking_group.damage_to_deal = damage;
                selected_targets.push(defending_group.index);
            } else if damage == attacking_group.damage_to_deal {
                selected_targets.push(defending_group.index);
            }
        }
        if selected_targets.len() == 1 {
            attacking_group.selected_target = selected_targets[0];
            defending_army.iter_mut().filter(|group| group.index == attacking_group.selected_target).next().unwrap().selected = true;
        } else if selected_targets.len() > 1 {
            let mut values = Vec::new();
            defending_army.iter().filter(|group| selected_targets.contains(&group.index)).for_each(|group| values.push((group.index, group.size * group.attack_damage, group.initiative)));

            values.sort_by(|value_a, value_b| {
                let effective_power_order = value_b.1.cmp(&(value_a.1));
                if effective_power_order == Ordering::Equal {
                    return value_b.2.cmp(&value_a.2);
                } else {
                    return effective_power_order;
                }
            });

            /*let highest_effective_power_group = values.iter().max_by(|value_a, value_b| value_a.1.cmp(&value_b.1)).unwrap();
            let groups_with_highest_effective_power = values.iter().filter(|value| value.1 == highest_effective_power_group.1).count();
            if groups_with_highest_effective_power == 1 {
                attacking_group.selected_target = highest_effective_power_group.0;
                defending_army.iter_mut().filter(|group| group.index == attacking_group.selected_target).next().unwrap().selected = true;
            } else if groups_with_highest_effective_power > 1 {
                let highest_initiative_group = values.iter().max_by(|value_a, value_b| value_a.2.cmp(&value_b.2)).unwrap();
                attacking_group.selected_target = highest_initiative_group.0;
                defending_army.iter_mut().filter(|group| group.index == attacking_group.selected_target).next().unwrap().selected = true;
            }*/
            attacking_group.selected_target = values[0].0;
            defending_army.iter_mut().filter(|group| group.index == attacking_group.selected_target).next().unwrap().selected = true;
        }
    }
}

fn create_immune_system_army() -> Vec<Group> {
    let mut immune_system_army = Vec::new();
    immune_system_army.push(Group::new(
        1, // id
        479, // size
        3393, // hit points
        66, // attack_damage
        Type::Cold, // attack_type
        8, // initiative
        vec![Type::Radiation], // weaknesses
        vec![], // immunities
    ));
    immune_system_army.push(Group::new(
        2, // id
        2202, // size
        4950, // hit points
        18, // attack_damage
        Type::Cold, // attack_type
        2, // initiative
        vec![Type::Fire], // weaknesses
        vec![Type::Slashing], // immunities
    ));
    immune_system_army.push(Group::new(
        3, // id
        8132, // size
        9680, // hit points
        9, // attack_damage
        Type::Radiation, // attack_type
        7, // initiative
        vec![Type::Bludgeoning, Type::Fire], // weaknesses
        vec![Type::Slashing], // immunities
    ));
    immune_system_army.push(Group::new(
        4, // id
        389, // size
        13983, // hit points
        256, // attack_damage
        Type::Cold, // attack_type
        13, // initiative
        vec![], // weaknesses
        vec![Type::Bludgeoning], // immunities
    ));
    immune_system_army.push(Group::new(
        5, // id
        1827, // size
        5107, // hit points
        24, // attack_damage
        Type::Slashing, // attack_type
        18, // initiative
        vec![], // weaknesses
        vec![], // immunities
    ));
    immune_system_army.push(Group::new(
        6, // id
        7019, // size
        2261, // hit points
        3, // attack_damage
        Type::Fire, // attack_type
        16, // initiative
        vec![], // weaknesses
        vec![Type::Radiation, Type::Slashing, Type::Cold], // immunities
    ));
    immune_system_army.push(Group::new(
        7, // id
        4736, // size
        8421, // hit points
        17, // attack_damage
        Type::Slashing, // attack_type
        3, // initiative
        vec![Type::Cold], // weaknesses
        vec![], // immunities
    ));
    immune_system_army.push(Group::new(
        8, // id
        491, // size
        3518, // hit points
        65, // attack_damage
        Type::Radiation, // attack_type
        1, // initiative
        vec![Type::Cold], // weaknesses
        vec![Type::Fire, Type::Bludgeoning], // immunities
    ));
    immune_system_army.push(Group::new(
        9, // id
        2309, // size
        7353, // hit points
        31, // attack_damage
        Type::Bludgeoning, // attack_type
        20, // initiative
        vec![], // weaknesses
        vec![Type::Radiation], // immunities
    ));
    immune_system_army.push(Group::new(
        10, // id
        411, // size
        6375, // hit points
        151, // attack_damage
        Type::Bludgeoning, // attack_type
        14, // initiative
        vec![Type::Cold, Type::Fire], // weaknesses
        vec![Type::Slashing], // immunities
    ));
    immune_system_army
}

fn create_infection_army() -> Vec<Group> {
    let mut infection_army = Vec::new();
    infection_army.push(Group::new(
        11, // id
        148, // size
        31914, // hit points
        416, // attack_damage
        Type::Cold, // attack_type
        4, // initiative
        vec![Type::Bludgeoning], // weaknesses
        vec![Type::Radiation, Type::Cold, Type::Fire], // immunities
    ));
    infection_army.push(Group::new(
        12, // id
        864, // size
        38189, // hit points
        72, // attack_damage
        Type::Slashing, // attack_type
        6, // initiative
        vec![], // weaknesses
        vec![], // immunities
    ));
    infection_army.push(Group::new(
        13, // id
        2981, // size
        7774, // hit points
        4, // attack_damage
        Type::Fire, // attack_type
        15, // initiative
        vec![], // weaknesses
        vec![Type::Bludgeoning, Type::Cold], // immunities
    ));
    infection_army.push(Group::new(
        14, // id
        5259, // size
        22892, // hit points
        8, // attack_damage
        Type::Fire, // attack_type
        5, // initiative
        vec![], // weaknesses
        vec![], // immunities
    ));
    infection_army.push(Group::new(
        15, // id
        318, // size
        16979, // hit points
        106, // attack_damage
        Type::Bludgeoning, // attack_type
        9, // initiative
        vec![Type::Fire], // weaknesses
        vec![], // immunities
    ));
    infection_army.push(Group::new(
        16, // id
        5017, // size
        32175, // hit points
        11, // attack_damage
        Type::Bludgeoning, // attack_type
        17, // initiative
        vec![Type::Slashing], // weaknesses
        vec![Type::Radiation], // immunities
    ));
    infection_army.push(Group::new(
        17, // id
        4308, // size
        14994, // hit points
        5, // attack_damage
        Type::Fire, // attack_type
        10, // initiative
        vec![Type::Slashing], // weaknesses
        vec![Type::Fire, Type::Cold], // immunities
    ));
    infection_army.push(Group::new(
        18, // id
        208, // size
        14322, // hit points
        133, // attack_damage
        Type::Cold, // attack_type
        19, // initiative
        vec![Type::Radiation], // weaknesses
        vec![], // immunities
    ));
    infection_army.push(Group::new(
        19, // id
        3999, // size
        48994, // hit points
        20, // attack_damage
        Type::Cold, // attack_type
        11, // initiative
        vec![Type::Cold, Type::Slashing], // weaknesses
        vec![], // immunities
    ));
    infection_army.push(Group::new(
        20, // id
        1922, // size
        34406, // hit points
        35, // attack_damage
        Type::Slashing, // attack_type
        12, // initiative
        vec![Type::Slashing], // weaknesses
        vec![], // immunities
    ));
    infection_army
}

fn main() {
    let mut immune_system_army = create_immune_system_army();
    let mut infection_army = create_infection_army();
    /*let mut immune_system_army = Vec::new();
    immune_system_army.push(Group::new(
        1,
        17,
        5390,
        4507,
        Type::Fire,
        2,
        vec![Type::Radiation, Type::Bludgeoning],
        vec![],
    ));
    immune_system_army.push(Group::new(
        2,
        989,
        1274,
        25,
        Type::Slashing,
        3,
        vec![Type::Bludgeoning, Type::Slashing],
        vec![Type::Fire],
    ));

    let mut infection_army = Vec::new();
    infection_army.push(Group::new(
        3,
        801,
        4706,
        116,
        Type::Bludgeoning,
        1,
        vec![Type::Radiation],
        vec![],
    ));
    infection_army.push(Group::new(
        4,
        4485,
        2961,
        12,
        Type::Slashing,
        4,
        vec![Type::Fire, Type::Cold],
        vec![Type::Radiation],
    ));*/

    while immune_system_army.len() > 0 && infection_army.len() > 0 {
        println!("----------------------------------------------------------------------------");
        print_armies_state(&immune_system_army, &infection_army);
        // TARGET SELECTION PHASE
        let comparison = |group_a: &Group, group_b: &Group| {
            let effective_power_order = (group_b.size * group_b.attack_damage).cmp(&(group_a.size * group_a.attack_damage));
            if effective_power_order == Ordering::Equal {
                return group_b.initiative.cmp(&group_a.initiative);
            } else {
                return effective_power_order;
            }
        };
        immune_system_army.sort_by(comparison);
        infection_army.sort_by(comparison);

        select_targets(&mut infection_army, &mut immune_system_army, Army::Infection);
        select_targets(&mut immune_system_army, &mut infection_army, Army::ImmuneSystem);

        // ATTACKING PHASE
        let mut initiatives = Vec::new();
        immune_system_army.iter().chain(infection_army.iter()).for_each(|group| {
            if group.selected_target != 0 {
                initiatives.push((group.index, group.initiative));
            }
        });
        initiatives.sort_by(|initiative_a, initiative_b| initiative_b.1.cmp(&initiative_a.1));
        //println!("{:?}", initiatives);

        for initiative in initiatives {
            let mut attacking_group_selected_target;
            let mut attacking_group_damage;
            {
                let attacking_group = immune_system_army.iter().chain(infection_army.iter()).filter(|group| group.index == initiative.0).next().unwrap();
                //println!("HERE6 {}", attacking_group.size);
                let defending_group = immune_system_army.iter().chain(infection_army.iter()).filter(|group| group.index == attacking_group.selected_target).next().unwrap();
                //println!("HERE7");
                attacking_group_damage = calculate_damage(&attacking_group, &defending_group);
            }
            {
                let attacking_group = immune_system_army.iter().chain(infection_army.iter()).filter(|group| group.index == initiative.0).next().unwrap();
                attacking_group_selected_target = attacking_group.selected_target;
                //attacking_group_damage = attacking_group.damage_to_deal;
            }
            let defending_group = immune_system_army.iter_mut().chain(infection_army.iter_mut()).filter(|group| group.index == attacking_group_selected_target).next().unwrap();
            println!("Attacking damage: {}, defending size: {}, defending hit_points: {}", attacking_group_damage, defending_group.size, defending_group.hit_points);
            let units_killed = attacking_group_damage / defending_group.hit_points;

            if units_killed <= defending_group.size {
                println!("Group {} attacks group {}, killing {} units", initiative.0, defending_group.index, units_killed);
                println!("attacking_group_damage: {}, defending_group.hit_points: {}", attacking_group_damage, defending_group.hit_points);
                defending_group.size -= units_killed;
            } else if units_killed > defending_group.size {
                println!("Group {} attacks group {}, killing {} units", initiative.0, defending_group.index, defending_group.size);
                defending_group.size = 0;
            }
        }

        // CLEARING
        immune_system_army = immune_system_army.into_iter().filter(|group| group.size > 0).collect();
        infection_army = infection_army.into_iter().filter(|group| group.size > 0).collect();
        immune_system_army.iter_mut().chain(infection_army.iter_mut()).for_each(|group| {
            group.selected_target = 0;
            group.selected = false;
            group.damage_to_deal = 0;
        });
    }

    let sum: u32 = immune_system_army.iter().chain(infection_army.iter()).map(|group| group.size).sum();
    println!("{}", sum);
}
