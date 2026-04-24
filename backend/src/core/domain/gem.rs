//! Defines the `Gem` struct and associated logic for generating spellcrafting gems based on target stats.

use std::collections::HashSet;

use super::stat::Stat;
use crate::core::domain::stat_category::StatCategory;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct Gem {
    pub id: u32,
    pub tier: u16,
    pub stat: Stat,
    pub value: u16,
    pub ip_cost: f32,
}

impl Gem {
    /// Generates the canonical catalog of all supported spellcrafting gems.
    ///
    /// Gem IDs are derived from this fixed order and therefore remain stable
    /// regardless of the current optimization request.
    pub fn all() -> Vec<Gem> {
        let mut gems = Vec::new();
        let mut current_id = 1;

        for stat in Stat::iter() {
            for (tier, (value, ip_cost)) in Self::tiers_for_stat(stat).into_iter().enumerate() {
                gems.push(Gem {
                    id: current_id,
                    stat,
                    tier: tier as u16,
                    value,
                    ip_cost,
                });
                current_id += 1;
            }
        }

        gems
    }

    /// Generates all valid spellcrafting gems for a given set of target stats.
    pub fn generate_for_targets(target_stats: &HashSet<Stat>) -> Vec<Gem> {
        Self::all()
            .into_iter()
            .filter(|gem| target_stats.contains(&gem.stat))
            .collect()
    }

    fn tiers_for_stat(stat: Stat) -> Vec<(u16, f32)> {
        match stat.category() {
            StatCategory::PhysicalStats | StatCategory::AcuityStats => {
                if stat == Stat::Hitpoints {
                    (0..10).map(|l| (4 + l * 8, 0.5 + l as f32)).collect()
                } else {
                    (0..10).map(|l| (1 + l * 3, 0.5 + l as f32)).collect()
                }
            }
            StatCategory::Resists => [1, 2, 3, 5, 7, 9, 11, 13, 15, 17]
                .into_iter()
                .map(|v| (v, if v == 1 { 0.5 } else { (v - 1) as f32 }))
                .collect(),
            StatCategory::MagicSkills
            | StatCategory::MeleeSkills
            | StatCategory::OtherSkills
            | StatCategory::DualWieldingSkills
            | StatCategory::ArcherySkills => {
                if matches!(
                    stat,
                    Stat::AllMagicSkills
                        | Stat::AllMeleeSkills
                        | Stat::AllArcherySkills
                        | Stat::AllDualWieldingSkills
                ) {
                    vec![(1, 0.5)]
                } else {
                    (1..=10)
                        .map(|v| (v, if v == 1 { 0.5 } else { (v - 1) as f32 * 2.5 }))
                        .collect()
                }
            }
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap};

    use itertools::Itertools;

    use super::*;

    #[test]
    fn generate_for_targets_preserves_canonical_ids() {
        let target_stats: HashSet<Stat> = [Stat::Dexterity, Stat::CrushResist, Stat::Sword]
            .iter()
            .copied()
            .collect();

        let expected = Gem::all()
            .into_iter()
            .filter(|gem| target_stats.contains(&gem.stat))
            .map(|gem| (gem.id, gem.stat, gem.tier, gem.value))
            .collect::<Vec<_>>();

        let actual = Gem::generate_for_targets(&target_stats)
            .into_iter()
            .map(|gem| (gem.id, gem.stat, gem.tier, gem.value))
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_stuff() {
        let target_stats: HashSet<Stat> = [
            Stat::Dexterity,
            Stat::DexterityCap,
            Stat::Constitution,
            Stat::ConstitutionCap,
            Stat::Charisma,
            Stat::CharismaCap,
            Stat::Hitpoints,
            Stat::HitpointsCap,
            Stat::BodyResist,
            Stat::ColdResist,
            Stat::HeatResist,
            Stat::CrushResist,
            Stat::EnergyResist,
            Stat::SpiritResist,
            Stat::ThrustResist,
            Stat::MatterResist,
            Stat::SlashResist,
        ]
        .iter()
        .cloned()
        .collect();
        let gems = Gem::generate_for_targets(&target_stats);

        println!(
            "Generated {} gems from {} preferences",
            gems.len(),
            target_stats.len()
        );

        let mut all_combinations: Vec<Vec<Gem>> = Vec::new();
        let mut current_combo: Vec<Gem> = Vec::with_capacity(4);

        fn backtrack_combinations(
            start_index: usize,
            pool: &[Gem],
            current_combo: &mut Vec<Gem>,
            results: &mut Vec<Vec<Gem>>,
        ) {
            if current_combo.len() == 4 {
                let max_ip_cost = current_combo
                    .iter()
                    .map(|g| g.ip_cost)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0);

                let base_sum: f32 = current_combo.iter().map(|g| g.ip_cost).sum();
                let total_ip = base_sum + max_ip_cost;

                if total_ip >= 36.0 && total_ip <= 37.5 {
                    results.push(current_combo.clone());
                }

                return;
            }

            for i in start_index..pool.len() {
                let candidate = &pool[i];

                if current_combo.iter().any(|g| g.stat == candidate.stat) {
                    continue;
                }

                current_combo.push(candidate.clone());

                backtrack_combinations(i + 1, pool, current_combo, results);

                current_combo.pop();
            }
        }

        backtrack_combinations(0, &gems, &mut current_combo, &mut all_combinations);

        println!(
            "Generated {} total raw combinations",
            all_combinations.len()
        );

        let mut variants_by_stat_set: BTreeMap<Vec<Stat>, Vec<Vec<u16>>> = BTreeMap::new();
        for combo in &all_combinations {
            let mut profile = combo
                .iter()
                .map(|gem| (gem.stat, gem.value))
                .collect::<Vec<_>>();
            profile.sort_by_key(|(stat, _)| *stat);

            let stat_set = profile.iter().map(|(stat, _)| *stat).collect::<Vec<_>>();
            let values = profile.iter().map(|(_, value)| *value).collect::<Vec<_>>();
            variants_by_stat_set
                .entry(stat_set)
                .or_default()
                .push(values);
        }

        let mut strictly_dominated_variants = 0usize;
        let mut undominated_variants = 0usize;

        for variants in variants_by_stat_set.values() {
            for (current_index, current) in variants.iter().enumerate() {
                let is_dominated = variants.iter().enumerate().any(|(other_index, other)| {
                    current_index != other_index
                        && other
                            .iter()
                            .zip(current.iter())
                            .all(|(left, right)| left >= right)
                        && other
                            .iter()
                            .zip(current.iter())
                            .any(|(left, right)| left > right)
                });

                if is_dominated {
                    strictly_dominated_variants += 1;
                } else {
                    undominated_variants += 1;
                }
            }
        }

        println!(
            "Pruned {} strictly dominated combinations; {} remain on the skyline",
            strictly_dominated_variants, undominated_variants
        );

        let x = gems.iter().cloned().combinations(4).collect::<Vec<_>>();
        println!("Generated {} total raw itertools combinations", x.len());
        assert_eq!(
            all_combinations.len(),
            strictly_dominated_variants + undominated_variants
        );
        assert!(undominated_variants <= all_combinations.len());
    }
}
