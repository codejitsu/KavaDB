#[derive(Debug)]
pub enum Command {
    Put(String, String),
    Read(String),
    ReadKeyByRange(String, String),
    BatchPut(Vec<String>),
    Delete(String),
}

impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["PUT", key, value] => Ok(Command::Put(key.to_string(), value.to_string())),
            ["READ", key] => Ok(Command::Read(key.to_string())),
            ["READRANGE", start, end] => {
                Ok(Command::ReadKeyByRange(start.to_string(), end.to_string()))
            }
            ["BATCHPUT", rest @ ..] => Ok(Command::BatchPut(
                rest.iter().map(|&k| k.to_string()).collect(),
            )),
            ["DELETE", key] => Ok(Command::Delete(key.to_string())),
            _ => Err("Invalid command format".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_str_put() {
        let cmd_result = Command::try_from("PUT key value");

        assert!(matches!(cmd_result, Ok(Command::Put(ref k, ref v)) if k == "key" && v == "value"));
    }

    #[test]
    fn test_command_from_str_read() {
        let cmd_result = Command::try_from("READ key");

        assert!(matches!(cmd_result, Ok(Command::Read(ref k)) if k == "key"));
    }

    #[test]
    fn test_command_from_str_read_range() {
        let cmd_result = Command::try_from("READRANGE startkey endkey");

        assert!(
            matches!(cmd_result, Ok(Command::ReadKeyByRange(ref start, ref end)) if start == "startkey" && end == "endkey")
        );
    }

    #[test]
    fn test_command_from_str_batch_put() {
        let cmd_result = Command::try_from("BATCHPUT key1 value1 key2 value2");

        assert!(
            matches!(cmd_result, Ok(Command::BatchPut(ref kvs)) if *kvs == vec!["key1", "value1", "key2", "value2"])
        );
    }

    #[test]
    fn test_command_from_str_delete() {
        let cmd_result = Command::try_from("DELETE mykey");

        assert!(matches!(cmd_result, Ok(Command::Delete(ref k)) if k == "mykey"));
    }
}
