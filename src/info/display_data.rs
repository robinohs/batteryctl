use colored::Colorize;
use eyre::{Context, Result};
use indexmap::IndexMap;
use serde::Serialize;
use uom::si::{electric_potential, energy, mass_per_energy, power, ratio, thermodynamic_temperature, time};

use crate::{
    battery_data::{BatteryData, PerBatteryData},
    info::command::InfoArgs,
};

#[derive(Debug, Serialize)]
pub(crate) struct DisplayData {
    data: Vec<BatteryRepresentation>,
}

impl DisplayData {
    /// Generate display data from battery data and command arguments
    pub fn generate_display_data(batteries: &BatteryData, args: &InfoArgs) -> DisplayData {
        let battery_representations = batteries
            .batteries()
            .iter()
            .map(|bat_data| BatteryRepresentation::from_per_battery_data(bat_data, args))
            .collect();
        DisplayData {
            data: battery_representations,
        }
    }

    /// Convert to a simple bare presentation format
    pub fn into_bare_presentation(self, args: &InfoArgs) -> String {
        let mut output = String::new();
        for bat in self.data.into_iter() {
            let indexed_map: IndexMap<String, String> = bat.into_indexed_map(args);
            for (label, value) in indexed_map {
                output += &format!("{}: {}\n", label, value);
            }
        }
        output
    }

    /// Convert to a default pretty representation format
    pub fn into_default_representation(self, args: &InfoArgs) -> String {
        let mut output = String::new();
        for bat in self.data.into_iter() {
            let bat_name = bat.name.clone();
            let indexed_map: IndexMap<String, String> = bat.into_indexed_map(args);
            let max_word_length = indexed_map.keys().map(|l| l.len()).max().unwrap_or(0);
            for (label, value) in indexed_map {
                if label == "Name" {
                    continue;
                }
                let padding = calculate_padding(&label, max_word_length);
                output += &format!("{}{}: {}\n", " ".repeat(padding), label.green(), value);
            }
            prepend_bat_title(bat_name, max_word_length, &mut output);
        }
        output
    }

    /// Convert to a JSON representation format
    pub fn into_json_representation(self, args: &InfoArgs) -> Result<String> {
        let reprs = self
            .data
            .into_iter()
            .map(|bat| bat.into_indexed_map(args))
            .collect::<Vec<IndexMap<String, String>>>();
        serde_json::to_string_pretty(&reprs).wrap_err("Failed to convert battery data to JSON")
    }
}

/// Generate a formatted title for the battery information section like ```======== Battery BAT_NAME ========```
fn prepend_bat_title(bat_name: Option<String>, max_word_length: usize, output: &mut String) {
    let name = match bat_name {
        Some(n) => format!("Battery {}", n),
        None => "Battery Unknown".to_string(),
    };
    let name_padding = max_word_length.saturating_sub(7);
    let top_label = format!("{:=<name_padding$} {} {:=<name_padding$}\n", "", name, "").green().bold();
    output.insert_str(0, &top_label);
}

/// Calculate padding for labels to align them properly
fn calculate_padding(label: &str, max_word: usize) -> usize {
    if label.len() < max_word { max_word - label.len() } else { 0 }
}

#[derive(Debug, Serialize, Clone)]
struct BatteryRepresentation {
    name: Option<String>,
    state_of_health: Option<String>,
    energy: Option<String>,
    energy_full: Option<String>,
    energy_full_design: Option<String>,
    energy_rate: Option<String>,
    state: Option<String>,
    voltage: Option<String>,
    temperature: Option<String>,
    vendor: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    technology: Option<String>,
    cycle_count: Option<String>,
    time_to_full: Option<String>,
    time_to_empty: Option<String>,
    carbon: Option<String>,
}

impl BatteryRepresentation {
    /// Create a BatteryRepresentation from PerBatteryData and InfoArgs
    fn from_per_battery_data(data: &PerBatteryData, args: &InfoArgs) -> Self {
        let bat = &data.bat;
        BatteryRepresentation {
            name: bat.name(),
            state_of_health: Some(format!("{:.2} %", bat.state_of_health().get::<ratio::percent>())),
            energy: Some(format!("{:.2} Wh", bat.energy().get::<energy::watt_hour>())),
            energy_full: Some(format!("{:.2} Wh", bat.energy_full().get::<energy::watt_hour>())),
            energy_full_design: Some(format!("{:.2} Wh", bat.energy_full_design().get::<energy::watt_hour>())),
            energy_rate: Some(format!("{:.2} W", bat.energy_rate().get::<power::watt>())),
            state: Some(bat.state().to_string()),
            voltage: Some(format!("{:.2} V", bat.voltage().get::<electric_potential::volt>())),
            temperature: bat.temperature().map(|t| {
                if args.fahrenheit {
                    format!("{:.2} °F", t.get::<thermodynamic_temperature::degree_fahrenheit>())
                } else {
                    format!("{:.2} °C", t.get::<thermodynamic_temperature::degree_celsius>())
                }
            }),
            vendor: bat.vendor().map(|s| s.to_string()),
            model: bat.model().map(|s| s.to_string()),
            serial_number: bat.serial_number().map(|s| s.to_string()),
            technology: Some(format!("{:?}", bat.technology())),
            cycle_count: bat.cycle_count().map(|c| c.to_string()),
            time_to_full: bat.time_to_full().map(|t| format!("{:.2} h", t.get::<time::hour>())),
            time_to_empty: bat.time_to_empty().map(|t| format!("{:.2} h", t.get::<time::hour>())),
            carbon: data
                .carbon
                .map(|c| format!("{:.2} gCO2/kWh", c.get::<mass_per_energy::gram_per_kilowatt_hour>())),
        }
    }

    fn into_indexed_map(self, info_args: &InfoArgs) -> IndexMap<String, String> {
        let mut map = IndexMap::new();

        Self::add_to_map(&mut map, "Name", &self.name, info_args);
        Self::add_to_map(&mut map, "State of Health", &self.state_of_health, info_args);
        Self::add_to_map(&mut map, "Energy", &self.energy, info_args);
        Self::add_to_map(&mut map, "Energy Full", &self.energy_full, info_args);
        Self::add_to_map(&mut map, "Energy Full Design", &self.energy_full_design, info_args);
        Self::add_to_map(&mut map, "Energy Rate", &self.energy_rate, info_args);
        Self::add_to_map(&mut map, "State", &self.state, info_args);
        Self::add_to_map(&mut map, "Voltage", &self.voltage, info_args);
        Self::add_to_map(&mut map, "Temperature", &self.temperature, info_args);
        Self::add_to_map(&mut map, "Vendor", &self.vendor, info_args);
        Self::add_to_map(&mut map, "Model", &self.model, info_args);
        Self::add_to_map(&mut map, "Serial Number", &self.serial_number, info_args);
        Self::add_to_map(&mut map, "Technology", &self.technology, info_args);
        Self::add_to_map(&mut map, "Cycle Count", &self.cycle_count, info_args);
        Self::add_to_map(&mut map, "Time to Full", &self.time_to_full, info_args);
        Self::add_to_map(&mut map, "Time to Empty", &self.time_to_empty, info_args);
        Self::add_to_map(&mut map, "Carbon Intensity", &self.carbon, info_args);
        map
    }

    fn add_to_map(map: &mut IndexMap<String, String>, key: &str, value: &Option<String>, info_args: &InfoArgs) {
        match value {
            Some(val) => map.insert(key.to_string(), val.to_string()),
            None if info_args.all => map.insert(key.to_string(), "Unavailable".to_string()),
            None => return,
        };
    }
}
