use crate::extension::process::ProcessResult;
use anyhow::Result;

pub fn communicate_result<T: serde::Serialize + std::fmt::Debug>(result: Result<T>) -> Result<()> {
    let result = match result {
        Ok(r) => ProcessResult {
            ok: Some(r),
            err: None,
        },
        Err(e) => ProcessResult {
            ok: None,
            err: Some(e.to_string()),
        },
    };

    log::debug!("Communicating result: {:?}", result);

    let result = bincode::serialize(&result).expect("serialize result with bincode");
    let result = hex::encode(result);
    println!("{}", result);
    Ok(())
}
