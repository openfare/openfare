use anyhow::Result;

pub trait Subject<SubT> {
    fn subject(&self) -> &SubT;
    fn subject_mut(&mut self) -> &mut SubT;
}

pub trait Get<SubT>: Subject<SubT> {
    fn get(&self, field_path: &Option<String>) -> Result<serde_json::Value>;
}

impl<'de, T, SubT> Get<SubT> for T
where
    T: Subject<SubT>,
    SubT: serde::de::DeserializeOwned + serde::Serialize,
{
    fn get(&self, field_path: &Option<String>) -> Result<serde_json::Value> {
        let subject = self.subject();
        let value = serde_json::to_value(&subject)?;

        let value = if let Some(field_path) = field_path {
            let mut target = &value;
            for field in field_path.split('.') {
                target = target
                    .get(field)
                    .ok_or(anyhow::format_err!("Failed to find field: {}", field))?;
            }
            (*target).clone()
        } else {
            value
        };
        Ok(value)
    }
}

pub trait Set<SubT>: Subject<SubT> {
    fn set(&mut self, field_path: &str, value: &str) -> Result<()>;
}

impl<'de, T, SubT> Set<SubT> for T
where
    T: Subject<SubT>,
    SubT: serde::de::DeserializeOwned + serde::Serialize,
{
    fn set(&mut self, field_path: &str, value: &str) -> Result<()> {
        let subject = self.subject_mut();
        let mut json_value = serde_json::to_value(&subject)?;

        let mut target = &mut json_value;
        for field in field_path.split('.') {
            target = target
                .get_mut(field)
                .ok_or(anyhow::format_err!("Failed to find field: {}", field))?;
        }
        let value = match serde_json::from_str(value) {
            Ok(v) => v,
            Err(_) => serde_json::json!(value),
        };
        *target = value;
        *subject = serde_json::from_value(json_value)?;
        Ok(())
    }
}
