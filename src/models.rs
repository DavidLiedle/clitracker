use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: u64,
    pub name: String,
    #[serde(rename = "current_state")]
    pub current_state: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_project() {
        let data = r#"{\"id\":1,\"name\":\"Test Project\"}"#;
        let project: Project = serde_json::from_str(data).unwrap();
        assert_eq!(project.id, 1);
        assert_eq!(project.name, "Test Project");
    }

    #[test]
    fn deserialize_story() {
        let data = r#"{\"id\":2,\"name\":\"My Story\",\"current_state\":\"started\"}"#;
        let story: Story = serde_json::from_str(data).unwrap();
        assert_eq!(story.id, 2);
        assert_eq!(story.current_state, "started");
    }
}
