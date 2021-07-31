use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    wrapped: R,
    bytes: usize,
    ops: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.ops
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.wrapped.read(buf);
        self.ops += 1;
        match result {
            Ok(bytes) => {
                self.bytes += bytes;
                Ok(bytes)
            }
            x => x,
        }
    }
}

pub struct WriteStats<W: Write> {
    wrapped: W,
    bytes: usize,
    ops: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            bytes: 0,
            ops: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.ops
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.wrapped.write(buf);
        self.ops += 1;
        match result {
            Ok(bytes) => {
                self.bytes += bytes;
                Ok(bytes)
            }
            x => x,
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
