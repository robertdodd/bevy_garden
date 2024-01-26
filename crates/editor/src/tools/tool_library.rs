use bevy::prelude::Resource;

use super::types::*;

/// Definition of a tool in the tool library
#[derive(Clone, Debug)]
pub struct ToolInfo {
    pub key: String,
    pub tool: Tool,
    pub name: String,
}

/// Resource containing the library of registered tool
#[derive(Resource)]
pub struct ToolLibrary {
    pub tools: Vec<ToolInfo>,
}

// TODO: Replace this default value and instead make each tool plugin responsible for adding itself to the library.
impl Default for ToolLibrary {
    fn default() -> Self {
        Self {
            tools: vec![
                ToolInfo {
                    key: "pointer".to_string(),
                    name: "Pointer".to_string(),
                    tool: Tool::Pointer,
                },
                ToolInfo {
                    key: "duplicate".to_string(),
                    name: "Duplicate".to_string(),
                    tool: Tool::Duplicate,
                },
            ],
        }
    }
}

impl ToolLibrary {
    pub fn contains_tool(&mut self, key: &String) -> bool {
        for tool in self.tools.iter() {
            if tool.key == *key {
                return true;
            }
        }
        false
    }

    pub fn register_tool(&mut self, tool_info: ToolInfo) {
        if self.contains_tool(&tool_info.key) {
            panic!("Tool already registered: {}", tool_info.key);
        }

        self.tools.push(ToolInfo {
            name: tool_info.name,
            key: tool_info.key,
            tool: tool_info.tool,
        });
    }
}
