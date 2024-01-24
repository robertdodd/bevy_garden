use std::slice::Iter;

use bevy::prelude::*;

use crate::prelude::*;

/// System sets for the tool stack plugin
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ToolStackSet {
    /// Handle events that interact with the tool stack
    EventHandling,
    /// Handle updates to the tool stack resource
    HandleChanges,
}

/// Resource for handling tool transitions.
/// Tools are stored in a "stack", allowing you to back out of the current tool, through all the previous tools, by
/// calling `ToolStack::pop`.
#[derive(Resource)]
pub struct ToolStack {
    default_tool: Tool,
    stack: Vec<Tool>,
}

impl ToolStack {
    pub fn new(default_tool: Tool) -> Self {
        Self {
            stack: vec![],
            default_tool,
        }
    }

    pub fn contains(&self, tool: Tool) -> bool {
        self.stack.contains(&tool)
    }

    pub fn get(&self) -> Tool {
        self.stack
            .last()
            .cloned()
            .unwrap_or(self.default_tool.clone())
    }

    pub fn count(&self) -> usize {
        self.stack.len()
    }

    /// Push a tool to the top of the stack
    pub fn push(&mut self, tool: Tool) {
        self.stack.push(tool)
    }

    /// Removes the top-most tool and returns the next tool in the stack
    pub fn pop(&mut self) -> Tool {
        self.stack.pop();
        self.get()
    }

    pub fn clear(&mut self) -> Tool {
        self.stack.clear();
        self.default_tool.clone()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> Iter<'_, Tool> {
        self.stack.iter()
    }
}
