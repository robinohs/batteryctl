use log::error;
use uom::si::f64::MassPerEnergy;

use crate::features::Features;

pub struct BatteryData {
    batteries: Vec<PerBatteryData>,
}

impl BatteryData {
    pub fn gather(features: Features) -> eyre::Result<BatteryData> {
        let manager = battery::Manager::new()?;
        let bats = manager
            .batteries()?
            .map(|bat| {
                let bat = bat?;
                let carbon = if features.has_carbond {
                    // Placeholder for actual carbon retrieval logic
                    match carbond_lib::try_load_battery_energy_intensity_sync() {
                        Ok(intensity) => Some(intensity),
                        Err(err) => {
                            error!("Failed to retrieve carbon intensity from carbond: {}", err);
                            None
                        }
                    }
                } else {
                    None
                };
                Ok(PerBatteryData { bat, carbon })
            })
            .collect::<Result<Vec<PerBatteryData>, eyre::Report>>()?;
        Ok(BatteryData { batteries: bats })
    }

    pub fn batteries(&self) -> &Vec<PerBatteryData> {
        &self.batteries
    }
}

pub struct PerBatteryData {
    pub bat: battery::Battery,
    pub carbon: Option<MassPerEnergy>,
}
