use std::{collections::HashMap, path::Path, str::FromStr};

use anyhow::{bail, ensure, Context, Result};
use chrono::{DateTime, Local, Utc};
use serde_json::Value;

pub fn format_datetime(datetime: DateTime<Local>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[derive(Debug)]
pub struct Inputs {
    root: HashMap<String, DateTime<Utc>>,
}

impl Inputs {
    fn new(root: HashMap<String, DateTime<Utc>>) -> Result<Inputs> {
        ensure!(!root.is_empty(), "Flake has no inputs");
        Ok(Inputs { root })
    }

    pub fn from_file<P: AsRef<Path>>(lock_file: P) -> Result<Inputs> {
        let content = std::fs::read_to_string(lock_file)
            .context("Could not reading flake lock file as string")?;
        Inputs::from_json(Value::from_str(&content).context("Could not parse flake lock as json")?)
    }
    pub fn from_json(value: Value) -> Result<Inputs> {
        let err = "Failed parsing json";
        let Value::Object(arr) = &value["nodes"]["root"]["inputs"] else {
            bail!(err)
        };

        let mut root = HashMap::new();

        for (_name, v) in arr {
            let Value::String(name) = v else { bail!(err) };

            let Value::Number(number) = &value["nodes"][name]["locked"]["lastModified"] else {
                bail!(err)
            };

            let Some(secs) = number.as_i64() else {
                bail!(err)
            };

            root.insert(name.clone(), DateTime::from_timestamp(secs, 0).unwrap());
        }

        Inputs::new(root)
    }

    pub fn latest(&self) -> DateTime<Utc> {
        // With construction of self is guaranteed there to be at least one
        *self.root.values().max().unwrap()
    }
}
