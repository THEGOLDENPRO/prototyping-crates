use core::fmt::{Error as FmtError, Result as FmtResult, Write};

// a temporary solution for string formatting until we get a memory allocator

pub struct SliceWriter<'a> {
    buffer: &'a mut [u8],
    offset: usize,
}

impl<'a> SliceWriter<'a> {
    pub fn new(bytes_buffer: &'a mut [u8]) -> Self {
        Self {
            buffer: bytes_buffer,
            offset: 0
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buffer[self.offset..self.offset + bytes.len()].copy_from_slice(bytes);
    }
}

impl<'a> Write for SliceWriter<'a> {
    fn write_str(&mut self, str: &str) -> FmtResult {
        let str_bytes = str.as_bytes();
        let str_length = str_bytes.len();

        if self.offset + str_length > self.buffer.len() {
            return Err(FmtError);
        }

        self.buffer[self.offset..self.offset + str_length].copy_from_slice(str_bytes);

        self.offset += str_length;

        Ok(())
    }
}