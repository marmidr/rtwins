//! Demo - commands queue

use std::collections::vec_deque::VecDeque;

use rtwins::wgt::WId;

// ---------------------------------------------------------------------------------------------- //

pub enum Command {
    ShowPopup {
        title: String,
        message: String,
        buttons: &'static str,
        on_button: Box<dyn Fn(WId) + Send + Sync>,
    },
    HidePopup,
}

/// Deferred commands
#[derive(Default)]
pub struct CommandsQueue {
    commands: VecDeque<Command>,
}

impl CommandsQueue {
    /// Pushes a new command on the queue
    pub fn push(&mut self, cmd: Command) {
        self.commands.push_back(cmd);
    }

    /// Take out the commands queue, return Some if non-empty
    pub fn take_commands(&mut self) -> Option<VecDeque<Command>> {
        if self.commands.is_empty() {
            None
        }
        else {
            Some(std::mem::take(&mut self.commands))
        }
    }

    /// Number of commands waiting to be run
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.commands.len()
    }
}
