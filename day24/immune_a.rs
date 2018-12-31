use std::cmp::Ordering;

#[derive(PartialEq)]
enum Type {
    Radiation,
    Cold,
    Fire,
    Slashing,
    Bludgeoning,
}

struct Group {
    id: u32,
    size: u32,
    hit_points: u32,
    attack_damage: u32,
    attack_type: Type,
    initiative: u32,
    weaknesses: Vec<Type>,
    immunities: Vec<Type>,
    selected_target: u32,
    selected: bool
}

impl Group {
    fn new(
        id: u32,
        size: u32,
        hit_points: u32,
        attack_damage: u32,
        attack_type: Type,
        initiative: u32,
        weaknesses: Vec<Type>,
        immunities: Vec<Type>,
    ) -> Group {
        Group {
            id,
            size,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
            selected_target: 0,
            selected: false
        }
    }
}

fn calculate_damage(attacking_group: &Group, defending_group: &Group) -> u32 {
    let mut damage = attacking_group.size * attacking_group.attack_damage;
    if defending_group
        .weaknesses
        .contains(&attacking_group.attack_type)
    {
        damage *= 2;
    } else if defending_group
        .immunities
        .contains(&attacking_group.attack_type)
    {
        damage = 0;
    }
    damage
}

fn select_targets(attacking_army: &mut Vec<Group>, defending_army: &mut Vec<Group>) {
    for attacking_group in attacking_army.iter_mut() {
        let mut selected_targets = Vec::new();
        let mut highest_damage = 0;

        for defending_group in defending_army.iter().filter(|group| !group.selected) {
            let damage = calculate_damage(&attacking_group, &defending_group);
            if damage == 0 {
                continue;
            }

            if damage > highest_damage {
                selected_targets.clear();
                highest_damage = damage;
                selected_targets.push(defending_group.id);
            } else if damage == highest_damage {
                selected_targets.push(defending_group.id);
            }
        }
        if selected_targets.len() == 1 {
            attacking_group.selected_target = selected_targets[0];
            defending_army
                .iter_mut()
                .filter(|group| group.id == attacking_group.selected_target)
                .next()
                .unwrap()
                .selected = true;
        } else if selected_targets.len() > 1 {
            let mut values = Vec::new();
            defending_army
                .iter()
                .filter(|group| selected_targets.contains(&group.id))
                .for_each(|group| {
                    values.push((group.id, group.size * group.attack_damage, group.initiative))
                });

            values.sort_by(|value_a, value_b| {
                let effective_power_order = value_b.1.cmp(&(value_a.1));
                if effective_power_order == Ordering::Equal {
                    return value_b.2.cmp(&value_a.2);
                } else {
                    return effective_power_order;
                }
            });

            attacking_group.selected_target = values[0].0;
            defending_army
                .iter_mut()
                .filter(|group| group.id == attacking_group.selected_target)
                .next()
                .unwrap()
                .selected = true;
        }
    }
}

fn create_immune_system_army() -> Vec<Group> {
    let mut immune_system_army = Vec::new();
    immune_system_army.push(Group::new(
        1,                     // id
        479,                   // size
        3393,                  // hit points
        66,                    // attack_damage
        Type::Cold,            // attack_type
        8,                     // initiative
        vec![Type::Radiation], // weaknesses
        vec![],                // immunities
    ));
    immune_system_army.push(Group::new(
        2,                    // id
        2202,                 // size
        4950,                 // hit points
        18,                   // attack_damage
        Type::Cold,           // attack_type
        2,                    // initiative
        vec![Type::Fire],     // weaknesses
        vec![Type::Slashing], // immunities
    ));
    immune_system_army.push(Group::new(
        3,                                   // id
        8132,                                // size
        9680,                                // hit points
        9,                                   // attack_damage
        Type::Radiation,                     // attack_type
        7,                                   // initiative
        vec![Type::Bludgeoning, Type::Fire], // weaknesses
        vec![Type::Slashing],                // immunities
    ));
    immune_system_army.push(Group::new(
        4,                       // id
        389,                     // size
        13983,                   // hit points
        256,                     // attack_damage
        Type::Cold,              // attack_type
        13,                      // initiative
        vec![],                  // weaknesses
        vec![Type::Bludgeoning], // immunities
    ));
    immune_system_army.push(Group::new(
        5,              // id
        1827,           // size
        5107,           // hit points
        24,             // attack_damage
        Type::Slashing, // attack_type
        18,             // initiative
        vec![],         // weaknesses
        vec![],         // immunities
    ));
    immune_system_army.push(Group::new(
        6,                                                 // id
        7019,                                              // size
        2261,                                              // hit points
        3,                                                 // attack_damage
        Type::Fire,                                        // attack_type
        16,                                                // initiative
        vec![],                                            // weaknesses
        vec![Type::Radiation, Type::Slashing, Type::Cold], // immunities
    ));
    immune_system_army.push(Group::new(
        7,                // id
        4736,             // size
        8421,             // hit points
        17,               // attack_damage
        Type::Slashing,   // attack_type
        3,                // initiative
        vec![Type::Cold], // weaknesses
        vec![],           // immunities
    ));
    immune_system_army.push(Group::new(
        8,                                   // id
        491,                                 // size
        3518,                                // hit points
        65,                                  // attack_damage
        Type::Radiation,                     // attack_type
        1,                                   // initiative
        vec![Type::Cold],                    // weaknesses
        vec![Type::Fire, Type::Bludgeoning], // immunities
    ));
    immune_system_army.push(Group::new(
        9,                     // id
        2309,                  // size
        7353,                  // hit points
        31,                    // attack_damage
        Type::Bludgeoning,     // attack_type
        20,                    // initiative
        vec![],                // weaknesses
        vec![Type::Radiation], // immunities
    ));
    immune_system_army.push(Group::new(
        10,                           // id
        411,                          // size
        6375,                         // hit points
        151,                          // attack_damage
        Type::Bludgeoning,            // attack_type
        14,                           // initiative
        vec![Type::Cold, Type::Fire], // weaknesses
        vec![Type::Slashing],         // immunities
    ));
    immune_system_army
}

fn create_infection_army() -> Vec<Group> {
    let mut infection_army = Vec::new();
    infection_army.push(Group::new(
        11,                                            // id
        148,                                           // size
        31914,                                         // hit points
        416,                                           // attack_damage
        Type::Cold,                                    // attack_type
        4,                                             // initiative
        vec![Type::Bludgeoning],                       // weaknesses
        vec![Type::Radiation, Type::Cold, Type::Fire], // immunities
    ));
    infection_army.push(Group::new(
        12,             // id
        864,            // size
        38189,          // hit points
        72,             // attack_damage
        Type::Slashing, // attack_type
        6,              // initiative
        vec![],         // weaknesses
        vec![],         // immunities
    ));
    infection_army.push(Group::new(
        13,                                  // id
        2981,                                // size
        7774,                                // hit points
        4,                                   // attack_damage
        Type::Fire,                          // attack_type
        15,                                  // initiative
        vec![],                              // weaknesses
        vec![Type::Bludgeoning, Type::Cold], // immunities
    ));
    infection_army.push(Group::new(
        14,         // id
        5259,       // size
        22892,      // hit points
        8,          // attack_damage
        Type::Fire, // attack_type
        5,          // initiative
        vec![],     // weaknesses
        vec![],     // immunities
    ));
    infection_army.push(Group::new(
        15,                // id
        318,               // size
        16979,             // hit points
        106,               // attack_damage
        Type::Bludgeoning, // attack_type
        9,                 // initiative
        vec![Type::Fire],  // weaknesses
        vec![],            // immunities
    ));
    infection_army.push(Group::new(
        16,                    // id
        5017,                  // size
        32175,                 // hit points
        11,                    // attack_damage
        Type::Bludgeoning,     // attack_type
        17,                    // initiative
        vec![Type::Slashing],  // weaknesses
        vec![Type::Radiation], // immunities
    ));
    infection_army.push(Group::new(
        17,                           // id
        4308,                         // size
        14994,                        // hit points
        5,                            // attack_damage
        Type::Fire,                   // attack_type
        10,                           // initiative
        vec![Type::Slashing],         // weaknesses
        vec![Type::Fire, Type::Cold], // immunities
    ));
    infection_army.push(Group::new(
        18,                    // id
        208,                   // size
        14322,                 // hit points
        133,                   // attack_damage
        Type::Cold,            // attack_type
        19,                    // initiative
        vec![Type::Radiation], // weaknesses
        vec![],                // immunities
    ));
    infection_army.push(Group::new(
        19,                               // id
        3999,                             // size
        48994,                            // hit points
        20,                               // attack_damage
        Type::Cold,                       // attack_type
        11,                               // initiative
        vec![Type::Cold, Type::Slashing], // weaknesses
        vec![],                           // immunities
    ));
    infection_army.push(Group::new(
        20,                   // id
        1922,                 // size
        34406,                // hit points
        35,                   // attack_damage
        Type::Slashing,       // attack_type
        12,                   // initiative
        vec![Type::Slashing], // weaknesses
        vec![],               // immunities
    ));
    infection_army
}

fn main() {
    let mut immune_system_army = create_immune_system_army();
    let mut infection_army = create_infection_army();

    while immune_system_army.len() > 0 && infection_army.len() > 0 {
        // TARGET SELECTION PHASE
        let comparison = |group_a: &Group, group_b: &Group| {
            let effective_power_order =
                (group_b.size * group_b.attack_damage).cmp(&(group_a.size * group_a.attack_damage));
            if effective_power_order == Ordering::Equal {
                return group_b.initiative.cmp(&group_a.initiative);
            } else {
                return effective_power_order;
            }
        };
        immune_system_army.sort_by(comparison);
        infection_army.sort_by(comparison);

        select_targets(&mut infection_army, &mut immune_system_army);
        select_targets(&mut immune_system_army, &mut infection_army);

        // ATTACKING PHASE
        let mut initiatives = Vec::new();
        immune_system_army
            .iter()
            .chain(infection_army.iter())
            .for_each(|group| {
                if group.selected_target != 0 {
                    initiatives.push((group.id, group.initiative));
                }
            });
        initiatives.sort_by(|initiative_a, initiative_b| initiative_b.1.cmp(&initiative_a.1));

        for initiative in initiatives {
            let mut attacking_group_selected_target;
            let mut attacking_group_damage;
            {
                let attacking_group = immune_system_army
                    .iter()
                    .chain(infection_army.iter())
                    .filter(|group| group.id == initiative.0)
                    .next()
                    .unwrap();
                let defending_group = immune_system_army
                    .iter()
                    .chain(infection_army.iter())
                    .filter(|group| group.id == attacking_group.selected_target)
                    .next()
                    .unwrap();
                attacking_group_damage = calculate_damage(&attacking_group, &defending_group);
            }
            {
                let attacking_group = immune_system_army
                    .iter()
                    .chain(infection_army.iter())
                    .filter(|group| group.id == initiative.0)
                    .next()
                    .unwrap();
                attacking_group_selected_target = attacking_group.selected_target;
            }
            let defending_group = immune_system_army
                .iter_mut()
                .chain(infection_army.iter_mut())
                .filter(|group| group.id == attacking_group_selected_target)
                .next()
                .unwrap();
            let units_killed = attacking_group_damage / defending_group.hit_points;

            if units_killed <= defending_group.size {
                defending_group.size -= units_killed;
            } else if units_killed > defending_group.size {
                defending_group.size = 0;
            }
        }

        // CLEARING
        immune_system_army = immune_system_army
            .into_iter()
            .filter(|group| group.size > 0)
            .collect();
        infection_army = infection_army
            .into_iter()
            .filter(|group| group.size > 0)
            .collect();
        immune_system_army
            .iter_mut()
            .chain(infection_army.iter_mut())
            .for_each(|group| {
                group.selected_target = 0;
                group.selected = false;
            });
    }

    let sum: u32 = immune_system_army
        .iter()
        .chain(infection_army.iter())
        .map(|group| group.size)
        .sum();
    println!("{}", sum);
}
