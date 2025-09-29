//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

const MAX_SYSCALL_NUM: usize = 1024;
#[derive(Clone, Copy)]
pub struct SystemCallCount {
    pub syscall_counts: [usize; MAX_SYSCALL_NUM],
}

impl SystemCallCount {
    pub const fn new() -> Self {
        SystemCallCount {
            syscall_counts: [0; MAX_SYSCALL_NUM],
        }
    }
}
