use anyhow::{format_err, Result};

use super::super::fs;
use super::common;

type PayeeName = String;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Payees {
    active: PayeeName,
    payees: std::collections::BTreeMap<PayeeName, openfare_lib::lock::payee::Payee>,
}

impl Payees {
    pub fn add(&mut self, name: &PayeeName) -> Result<()> {
        if self.payees.contains_key(name) {
            return Err(format_err!(
                "Payee with given name already exists: {}",
                name
            ));
        }
        self.payees
            .insert(name.clone(), openfare_lib::lock::payee::Payee::default());
        Ok(())
    }

    pub fn remove(&mut self, name: &PayeeName) -> Result<()> {
        self.payees
            .remove(name)
            .ok_or(format_err!("Failed to remove unknown payee: {}", name))?;

        if name == &self.active {
            self.active = if let Some((name, _)) = self.payees.iter().next() {
                name.clone()
            } else {
                "".to_string()
            };
        }
        Ok(())
    }

    pub fn activate(&mut self, name: &PayeeName) -> Result<()> {
        if self.payees.contains_key(name) {
            self.active = name.clone();
            Ok(())
        } else {
            Err(format_err!(
                "Payee does not exist. Can not set active payee: {}",
                name
            ))
        }
    }

    pub fn rename(&mut self, old_name: &PayeeName, new_name: &PayeeName) -> Result<()> {
        if self.payees.contains_key(new_name) {
            return Err(format_err!(
                "Target payee profile name already exists: {}",
                new_name
            ));
        }

        if let Some(payee) = self.payees.remove(old_name) {
            self.payees.insert(new_name.clone(), payee);
            if &self.active == old_name {
                self.active = new_name.clone();
            }
        } else {
            return Err(format_err!("Can't find payee profile named: {}", old_name));
        }
        Ok(())
    }

    pub fn payees(
        &self,
    ) -> &std::collections::BTreeMap<PayeeName, openfare_lib::lock::payee::Payee> {
        &self.payees
    }

    pub fn active(&self) -> Result<Option<(&PayeeName, &openfare_lib::lock::payee::Payee)>> {
        Ok(if self.payees.is_empty() {
            None
        } else {
            Some((
                &self.active,
                self.payees.get(&self.active).ok_or(format_err!(
                    "Code error failed to find active payee: {}",
                    &self.active
                ))?,
            ))
        })
    }

    pub fn active_mut(
        &mut self,
    ) -> Result<Option<(&mut PayeeName, &mut openfare_lib::lock::payee::Payee)>> {
        Ok(if self.payees.is_empty() {
            None
        } else {
            let payee = self.payees.get_mut(&self.active).ok_or(format_err!(
                "Code error failed to find active payee: {}",
                &self.active
            ))?;
            Some((&mut self.active, payee))
        })
    }
}

impl common::FilePath for Payees {
    fn file_path() -> Result<std::path::PathBuf> {
        let paths = fs::ConfigPaths::new()?;
        Ok(paths.payees_file)
    }
}

impl std::fmt::Display for Payees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error::default())?
        )
    }
}
