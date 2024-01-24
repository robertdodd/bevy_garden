use bevy::prelude::*;

/// The global registry of snapshots used for rollback / rollforward.
#[derive(Resource, Default)]
pub struct Rollbacks {
    pub(crate) checkpoints: Vec<Handle<DynamicScene>>,
    pub(crate) active: Option<usize>,
}

impl Rollbacks {
    /// Returns true if no checkpoints have been created.
    pub fn is_empty(&self) -> bool {
        self.checkpoints.is_empty()
    }

    /// Returns true if there is a checkpoint available in the direction of the given number of checkpoints.
    pub fn has_checkpoint(&self, checkpoint: isize) -> bool {
        if self.checkpoints.is_empty() {
            return false;
        }
        let next = self.active.unwrap_or(0) as isize - checkpoint;
        next >= 0 && next < self.checkpoints.len() as isize
    }

    /// Returns the number of checkpoints
    pub fn count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Returns the current checkpoint
    pub fn active(&self) -> Option<usize> {
        self.active
    }

    /// Clears all checkpoints
    pub fn clear_checkpoints(&mut self) {
        self.checkpoints.clear();
        self.active = None;
    }

    /// Given a new [`Rollback`], insert it and set it as the currently active rollback.
    ///
    /// If you rollback and then insert a checkpoint, it will erase all rollforward snapshots.
    pub fn push_checkpoint(&mut self, scene: Handle<DynamicScene>) {
        let active = self.active.unwrap_or(0);

        self.checkpoints.truncate(active + 1);
        self.checkpoints.push(scene);

        self.active = Some(self.checkpoints.len() - 1);
    }

    /// Rolls back the given number of checkpoints.
    ///
    /// If checkpoints is negative, it rolls forward.
    ///
    /// This function will always clamp itself to valid rollbacks.
    /// Rolling back or further farther than what is valid will just return the oldest / newest snapshot.
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_sign_loss)]
    pub fn rollback(&mut self, checkpoints: isize) -> Option<Handle<DynamicScene>> {
        if let Some(active) = self.active {
            let raw = active as isize - checkpoints;
            let new = raw.clamp(0, self.checkpoints.len() as isize - 1) as usize;

            self.active = Some(new);
            Some(self.checkpoints[new].clone())
        } else {
            None
        }
    }
}
