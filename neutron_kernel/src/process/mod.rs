pub mod elf;
pub mod scheduler;
pub mod thread;

use alloc::{string::String, vec, vec::Vec};

#[derive(Debug, Default)]
pub struct Process {
    id: u32,
    name: String,
    space_allocated: u32,
    space_used: u32,
    status: ProcessStatus,
    priority: i64,
}

#[derive(Debug, Default)]
pub enum ProcessStatus {
    #[default]
    UP,
    DOWN,
    BLOCKED,
}

#[derive(Debug, Default)]
pub enum ProcessPrivilege {
    #[default]
    FULL,
    RD_ONLY,
    RD_WRITE,
    NONE,
}

#[derive(Debug)]
pub struct ElfBinary(Vec<u8>);

#[derive(Debug, Default)]
pub enum ProcessExitStatus {
    #[default]
    SUCCESS,
    BAD,
    PANICKED,
}

// Instead of space allocated, own an AddrSpace instead that manages it

impl Process {
    pub fn new(
        id: u32,
        name: String,
        space_allocated: u32,
        space_used: u32,
        status: ProcessStatus,
        priority: i64,
    ) -> Self {
        Self {
            id,
            name,
            space_allocated,
            space_used,
            status,
            priority,
        }
    }

    /// A process has 5 regions, https://en.wikipedia.org/wiki/File:Program_memory_layout.pdf. Should return process exit code
    pub fn execute_elf64(&self, validated_elf_bin: &ElfBinary) -> ProcessExitStatus {
        // call elf function. When it returns, return success to the kernel process subsystem (manager) / sched

        ProcessExitStatus::SUCCESS
    }

    /// Called when userspace process calls thread.create() or any async/await code that generates a new user thread. Backs up that user thread with a kernel thread in kheap,
    pub fn create_thread(&self) {}
}

// -------------
// TEST
// -------------

#[test]
fn test_process() {
    let process = Process::default();
    std::println!("Process succesfully created: {process:?}!");
}
