use super::*;

pub(crate) fn as_str<'a>(obj: &'a Value, key: &str) -> Result<&'a str> {
    obj.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing string key: {}", key))
}

pub(crate) fn parse_stack_overrides_map(data: Option<&Value>) -> Result<HashMap<String, f64>> {
    let Some(raw) = data else {
        return Ok(HashMap::new());
    };
    let object = raw
        .as_object()
        .ok_or_else(|| anyhow!("stack_overrides must be an object keyed by stack identifier"))?;
    let mut out = HashMap::new();
    for (stack_identifier, value) in object {
        let stack_value = value
            .as_f64()
            .ok_or_else(|| anyhow!("stack_overrides.{} must be numeric", stack_identifier))?;
        if stack_value < 0.0 {
            bail!(
                "stack_overrides.{} must be >= 0.0, got {}",
                stack_identifier,
                stack_value
            );
        }
        out.insert(stack_identifier.clone(), stack_value);
    }
    Ok(out)
}
