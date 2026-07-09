use crate::core::{
    domain::{class::Class, preference::Preference, preference_preset::PreferencePreset, stat::Stat},
    error::CoreResult,
};
use rusqlite::Connection;
use serde::Deserialize;

#[derive(Deserialize)]
struct DbPreference {
    stat_id: u16,
    min: u16,
    weight: u16,
}

pub fn get_all_preference_presets(connection: &Connection) -> CoreResult<Vec<PreferencePreset>> {
    let mut stmt = connection.prepare(
        "SELECT 
            pp.id,
            pp.class_id,
            pp.name,
            json_group_array(
                json_object('stat_id', pps.stat_id, 'min', pps.min, 'weight', pps.weight)
            ) FILTER (WHERE pps.stat_id IS NOT NULL) as preferences_json
         FROM preference_preset pp
         LEFT JOIN preference pps ON pps.preset_id = pp.id
         GROUP BY pp.id"
    )?;

    let presets = stmt.query_map([], |row| {
        let id: u32 = row.get(0)?;
        let class_id: u16 = row.get(1)?;
        let name: String = row.get(2)?;
        let preferences_json: Option<String> = row.get(3)?;
        
        let mut preferences = std::collections::HashMap::new();
        
        if let Some(json) = preferences_json {
            let parsed_stats: Vec<DbPreference> = serde_json::from_str(&json).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e))
            })?;

            for stat in parsed_stats {
                if let Some(domain_stat) = Stat::from_repr(stat.stat_id) {
                    preferences.insert(
                        domain_stat,
                        Preference {
                            min: stat.min,
                            weight: stat.weight,
                        }
                    );
                }
            }
        }

        let class = Class::from_repr(class_id).expect("Invalid class ID in database");

        Ok(PreferencePreset::new(id, class, name, preferences))
    })?;

    presets.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

/// Inserts a list of preference presets and their associated stats into the database.
pub fn insert_preference_presets(
    connection: &mut Connection,
    presets: &[PreferencePreset],
) -> CoreResult<()> {
    let tx = connection.transaction()?;
    {
        let mut preset_stmt = tx.prepare(
            "INSERT INTO preference_preset (id, class_id, name) VALUES (?1, ?2, ?3)"
        )?;

        let mut stat_stmt = tx.prepare(
            "INSERT INTO preference (preset_id, stat_id, min, weight) VALUES (?1, ?2, ?3, ?4)"
        )?;

        for preset in presets {
            preset_stmt.execute(rusqlite::params![
                preset.id(),
                preset.class().id(),
                preset.name()
            ])?;

            for (stat, pref) in preset.preferences() {
                stat_stmt.execute(rusqlite::params![
                    preset.id(),
                    stat.id(),
                    pref.min,
                    pref.weight
                ])?;
            }
        }
    }

    tx.commit()?;

    Ok(())
}