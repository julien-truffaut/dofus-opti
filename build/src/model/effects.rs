use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Effects {
    pub ability_point: Option<i32>,
    pub ability_point_parry: Option<i32>,
    pub ability_point_reduction: Option<i32>,
    pub agility: Option<i32>,
    pub air_damage: Option<i32>,
    pub air_resistance: Option<i32>,
    pub air_resistance_percent: Option<i32>,
    pub chance: Option<i32>,
    pub critical: Option<i32>,
    pub critical_damage: Option<i32>,
    pub critical_resistance: Option<i32>,
    pub damage: Option<i32>,
    pub dodge: Option<i32>,
    pub earth_damage: Option<i32>,
    pub earth_resistance: Option<i32>,
    pub earth_resistance_percent: Option<i32>,
    pub fire_damage: Option<i32>,
    pub fire_resistance: Option<i32>,
    pub fire_resistance_percent: Option<i32>,
    pub heals: Option<i32>,
    pub initiative: Option<i32>,
    pub intelligence: Option<i32>,
    pub lock: Option<i32>,
    pub melee_damage: Option<i32>,
    pub melee_resistance: Option<i32>,
    pub movement_point: Option<i32>,
    pub movement_point_parry: Option<i32>,
    pub movement_point_reduction: Option<i32>,
    pub neutral_damage: Option<i32>,
    pub neutral_resistance: Option<i32>,
    pub neutral_resistance_percent: Option<i32>,
    pub pods: Option<i32>,
    pub power: Option<i32>,
    pub prospecting: Option<i32>,
    pub push_back_damage: Option<i32>,
    pub push_back_resistance: Option<i32>,
    pub range: Option<i32>,
    pub range_damage: Option<i32>,
    pub range_resistance: Option<i32>,
    pub reflected_damage: Option<i32>,
    pub spell_damage: Option<i32>,
    pub strength: Option<i32>,
    pub summon: Option<i32>,
    pub trap_damage: Option<i32>,
    pub trap_power: Option<i32>,
    pub vitality: Option<i32>,
    pub water_damage: Option<i32>,
    pub water_resistance: Option<i32>,
    pub water_resistance_percent: Option<i32>,
    pub weapon_damage: Option<i32>,
    pub wisdom: Option<i32>,
}

impl Effects {
    pub fn empty() -> Effects {
        Effects {
            ability_point: None,
            ability_point_parry: None,
            ability_point_reduction: None,
            agility: None,
            air_damage: None,
            air_resistance: None,
            air_resistance_percent: None,
            chance: None,
            critical: None,
            critical_damage: None,
            critical_resistance: None,
            damage: None,
            dodge: None,
            earth_damage: None,
            earth_resistance: None,
            earth_resistance_percent: None,
            fire_damage: None,
            fire_resistance: None,
            fire_resistance_percent: None,
            heals: None,
            initiative: None,
            intelligence: None,
            lock: None,
            melee_damage: None,
            melee_resistance: None,
            movement_point: None,
            movement_point_parry: None,
            movement_point_reduction: None,
            neutral_damage: None,
            neutral_resistance: None,
            neutral_resistance_percent: None,
            pods: None,
            power: None,
            prospecting: None,
            push_back_damage: None,
            push_back_resistance: None,
            range: None,
            range_damage: None,
            range_resistance: None,
            reflected_damage: None,
            spell_damage: None,
            strength: None,
            summon: None,
            trap_damage: None,
            trap_power: None,
            vitality: None,
            water_damage: None,
            water_resistance: None,
            water_resistance_percent: None,
            weapon_damage: None,
            wisdom: None,
        }
    }
}

macro_rules! impl_effects_ops {
    ($($field:ident),+) => {
        impl Add for Effects {
            type Output = Effects;

            fn add(self, other: Effects) -> Effects {
                Effects {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a + b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }

        impl Sub for Effects {
            type Output = Effects;

            fn sub(self, other: Effects) -> Effects {
                Effects {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a - b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(-b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }

        impl<'a, 'b> Add<&'b Effects> for &'a Effects {
            type Output = Effects;

            fn add(self, other: &'b Effects) -> Effects {
                Effects {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a + b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }

        impl<'a, 'b> Sub<&'b Effects> for &'a Effects {
            type Output = Effects;

            fn sub(self, other: &'b Effects) -> Effects {
                Effects {
                    $(
                        $field: match (self.$field, other.$field) {
                            (Some(a), Some(b)) => Some(a - b),
                            (Some(a), None) => Some(a),
                            (None, Some(b)) => Some(-b),
                            (None, None) => None,
                        },
                    )+
                }
            }
        }
    };
}

impl_effects_ops!(
    ability_point,
    ability_point_parry,
    ability_point_reduction,
    agility,
    air_damage,
    air_resistance,
    air_resistance_percent,
    chance,
    critical,
    critical_damage,
    critical_resistance,
    damage,
    dodge,
    earth_damage,
    earth_resistance,
    earth_resistance_percent,
    fire_damage,
    fire_resistance,
    fire_resistance_percent,
    heals,
    initiative,
    intelligence,
    lock,
    melee_damage,
    melee_resistance,
    movement_point,
    movement_point_parry,
    movement_point_reduction,
    neutral_damage,
    neutral_resistance,
    neutral_resistance_percent,
    pods,
    power,
    prospecting,
    push_back_damage,
    push_back_resistance,
    range,
    range_damage,
    range_resistance,
    reflected_damage,
    spell_damage,
    strength,
    summon,
    trap_damage,
    trap_power,
    vitality,
    water_damage,
    water_resistance,
    water_resistance_percent,
    weapon_damage,
    wisdom
);

impl AddAssign<&Effects> for Effects {
    fn add_assign(&mut self, other: &Self) {
        *self = &*self + &other;
    }
}

impl SubAssign<&Effects> for Effects {
    fn sub_assign(&mut self, other: &Self) {
        *self = &*self - &other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_EFFECT: Effects = Effects {
        ability_point: Some(1),
        ability_point_parry: Some(2),
        ability_point_reduction: Some(3),
        agility: Some(4),
        air_damage: Some(5),
        air_resistance: Some(6),
        air_resistance_percent: Some(7),
        chance: Some(8),
        critical: Some(9),
        critical_damage: Some(10),
        critical_resistance: Some(11),
        damage: Some(12),
        dodge: Some(13),
        earth_damage: Some(14),
        earth_resistance: Some(15),
        earth_resistance_percent: Some(16),
        fire_damage: Some(17),
        fire_resistance: Some(18),
        fire_resistance_percent: Some(19),
        heals: Some(20),
        initiative: Some(21),
        intelligence: Some(22),
        lock: Some(23),
        melee_damage: Some(24),
        melee_resistance: Some(25),
        movement_point: Some(26),
        movement_point_parry: Some(27),
        movement_point_reduction: Some(28),
        neutral_damage: Some(29),
        neutral_resistance: Some(30),
        neutral_resistance_percent: Some(31),
        pods: Some(32),
        power: Some(33),
        prospecting: Some(34),
        push_back_damage: Some(35),
        push_back_resistance: Some(36),
        range: Some(37),
        range_damage: Some(38),
        range_resistance: Some(39),
        reflected_damage: Some(40),
        spell_damage: Some(41),
        strength: Some(42),
        summon: Some(43),
        trap_damage: Some(44),
        trap_power: Some(45),
        vitality: Some(46),
        water_damage: Some(47),
        water_resistance: Some(48),
        water_resistance_percent: Some(49),
        weapon_damage: Some(50),
        wisdom: Some(51),
    };

    #[test]
    fn empty_noop() {
        assert_eq!(TEST_EFFECT.clone() + Effects::empty(), TEST_EFFECT.clone());
        assert_eq!(TEST_EFFECT.clone() - Effects::empty(), TEST_EFFECT.clone());
    }

    #[test]
    fn add() {
        let double = Effects {
            ability_point: Some(2),
            ability_point_parry: Some(4),
            ability_point_reduction: Some(6),
            agility: Some(8),
            air_damage: Some(10),
            air_resistance: Some(12),
            air_resistance_percent: Some(14),
            chance: Some(16),
            critical: Some(18),
            critical_damage: Some(20),
            critical_resistance: Some(22),
            damage: Some(24),
            dodge: Some(26),
            earth_damage: Some(28),
            earth_resistance: Some(30),
            earth_resistance_percent: Some(32),
            fire_damage: Some(34),
            fire_resistance: Some(36),
            fire_resistance_percent: Some(38),
            heals: Some(40),
            initiative: Some(42),
            intelligence: Some(44),
            lock: Some(46),
            melee_damage: Some(48),
            melee_resistance: Some(50),
            movement_point: Some(52),
            movement_point_parry: Some(54),
            movement_point_reduction: Some(56),
            neutral_damage: Some(58),
            neutral_resistance: Some(60),
            neutral_resistance_percent: Some(62),
            pods: Some(64),
            power: Some(66),
            prospecting: Some(68),
            push_back_damage: Some(70),
            push_back_resistance: Some(72),
            range: Some(74),
            range_damage: Some(76),
            range_resistance: Some(78),
            reflected_damage: Some(80),
            spell_damage: Some(82),
            strength: Some(84),
            summon: Some(86),
            trap_damage: Some(88),
            trap_power: Some(90),
            vitality: Some(92),
            water_damage: Some(94),
            water_resistance: Some(96),
            water_resistance_percent: Some(98),
            weapon_damage: Some(100),
            wisdom: Some(102),
        };
        assert_eq!(TEST_EFFECT.clone() + TEST_EFFECT.clone(), double);
    }

    #[test]
    fn sub() {
        let zero = Effects {
            ability_point: Some(0),
            ability_point_parry: Some(0),
            ability_point_reduction: Some(0),
            agility: Some(0),
            air_damage: Some(0),
            air_resistance: Some(0),
            air_resistance_percent: Some(0),
            chance: Some(0),
            critical: Some(0),
            critical_damage: Some(0),
            critical_resistance: Some(0),
            damage: Some(0),
            dodge: Some(0),
            earth_damage: Some(0),
            earth_resistance: Some(0),
            earth_resistance_percent: Some(0),
            fire_damage: Some(0),
            fire_resistance: Some(0),
            fire_resistance_percent: Some(0),
            heals: Some(0),
            initiative: Some(0),
            intelligence: Some(0),
            lock: Some(0),
            melee_damage: Some(0),
            melee_resistance: Some(0),
            movement_point: Some(0),
            movement_point_parry: Some(0),
            movement_point_reduction: Some(0),
            neutral_damage: Some(0),
            neutral_resistance: Some(0),
            neutral_resistance_percent: Some(0),
            pods: Some(0),
            power: Some(0),
            prospecting: Some(0),
            push_back_damage: Some(0),
            push_back_resistance: Some(0),
            range: Some(0),
            range_damage: Some(0),
            range_resistance: Some(0),
            reflected_damage: Some(0),
            spell_damage: Some(0),
            strength: Some(0),
            summon: Some(0),
            trap_damage: Some(0),
            trap_power: Some(0),
            vitality: Some(0),
            water_damage: Some(0),
            water_resistance: Some(0),
            water_resistance_percent: Some(0),
            weapon_damage: Some(0),
            wisdom: Some(0),
        };
        assert_eq!(TEST_EFFECT.clone() - TEST_EFFECT.clone(), zero);
    }
}
