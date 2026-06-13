use super::*;

/// System utility APIs exposed to scripts.
pub trait RocoSystemStdLib: Send {
    fn sleep(&mut self, _ms: i64) -> Result<()> {
        unsupported("system::sleep")
    }

    fn now_ms(&mut self) -> Result<i64> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| ScriptSystemError::CurrentTimeBeforeUnixEpoch {
                message: error.to_string(),
            })?;
        i64::try_from(now.as_millis())
            .map_err(|_| ScriptSystemError::CurrentTimestampExceedsI64.into())
    }

    fn sleep_until_ms(&mut self, target_ms: i64) -> Result<()> {
        let now = self.now_ms()?;
        if target_ms <= now {
            return Ok(());
        }
        self.sleep(target_ms - now)
    }

    fn format_time(&mut self, timestamp: i64) -> Result<String> {
        Ok(timestamp.to_string())
    }

    fn log(&mut self, _message: &str) -> Result<()> {
        unsupported("system::log")
    }

    fn assert(&mut self, condition: bool, message: &str) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(RocoError::AssertionError(message.to_string()))
        }
    }
}
