use crate::guest_gen;

#[derive(Debug)]
pub(crate) struct MemoryBuffer {
    // TEST pub
    pub(crate) id: u8,
}

#[derive(Debug)]
pub(crate) enum MemoryBufferCreateError {
    SizeCannotBeLargerThan1Kb,
    SimultaneousBufferLimitReached,
}

impl MemoryBuffer {
    pub(crate) fn new(size: u16) -> Result<Self, MemoryBufferCreateError> {
        if size > 1024 {
            return Err(MemoryBufferCreateError::SizeCannotBeLargerThan1Kb);
        }

        let id = guest_gen::imports::alloc_memory_buffer(size);

        if id == 0 {
            return Err(MemoryBufferCreateError::SimultaneousBufferLimitReached);
        }

        Ok(Self { id })
    }

    pub(crate) fn read(mut self) -> Vec<u8> {
        let content = guest_gen::imports::read_memory_buffer(self.id);
        self.id = 0;
        content
    }
}

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        guest_gen::imports::dealloc_memory_buffer(self.id);
    }
}
