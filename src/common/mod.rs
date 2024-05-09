use anyhow::bail;
use thiserror::Error;

pub mod binary_macros;
pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unable to parse the data")]
    ParseError(#[from] std::io::Error),
    #[error("Unabl to read {n} bytes, current position = {cur_pos}, buf_length = {buf_length}")]
    BufOverflow {
        cur_pos: usize,
        n: usize,
        buf_length: usize,
    },
}

pub trait Parse {
    fn parse(reader: &mut DnsReader) -> Self;
}

pub struct DnsReader<'a> {
    pub buf: &'a [u8],
    pub cur_pos: usize,
}

impl<'a> DnsReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf: buf,
            cur_pos: 0,
        }
    }
    pub fn read_exact(&mut self, target_buf: &mut [u8]) -> anyhow::Result<()> {
        let len = target_buf.len();
        let upto = self.cur_pos + len;
        let buf_len = self.buf.len();
        if upto > buf_len {
            bail!(ParseError::BufOverflow {
                buf_length: buf_len,
                cur_pos: self.cur_pos,
                n: len
            })
        }
        let source_buf = &self.buf[self.cur_pos..upto];
        target_buf.copy_from_slice(source_buf);
        self.cur_pos += len;
        Ok(())
    }
}
