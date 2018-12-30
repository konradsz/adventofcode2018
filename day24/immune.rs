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

fn main() {
    let mut immune_system_army = Vec::new();
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
    ));

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
