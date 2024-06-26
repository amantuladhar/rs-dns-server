use std::io::Read;

use tracing::debug;

use crate::{
    bits, bits16,
    common::{dns_reader::DnsReader, AsBytes, Parse},
};

use super::{label::Label, RecordClass, RecordType};

#[derive(Debug, Clone)]
pub struct Question {
    pub name: Label,
    pub typez: RecordType,
    pub class: RecordClass,
}

impl AsBytes for Question {
    fn as_bytes(&self) -> Vec<u8> {
        let mut buf = self.name.as_bytes();
        buf.extend(self.typez.as_bytes());
        buf.extend(self.class.as_bytes());
        return buf;
    }
}
impl Parse for Question {
    fn parse(reader: &mut DnsReader) -> Self {
        let label = Label::parse(reader);
        let record_type = RecordType::parse(reader);
        let record_class = RecordClass::parse(reader);
        Self {
            name: label,
            typez: record_type,
            class: record_class,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        common::{dns_reader::DnsReader, AsBytes, Parse},
        dns::{label::Label, question::Question, RecordClass, RecordType},
    };

    #[test]
    fn test_dns_message() {
        let message = Question {
            name: Label("google.com".to_string()),
            typez: RecordType::A,
            class: RecordClass::IN,
        };
        assert_eq!(
            message.as_bytes(),
            vec![
                0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
                // record_type
                0x00, 0x1, // record_class
                0x0, 0x1
            ]
        )
    }

    #[test]
    fn test_parse() {
        let message = Question {
            name: Label("google.com".to_string()),
            typez: RecordType::A,
            class: RecordClass::IN,
        };
        let mut actual_bytes = message.as_bytes();
        let mut reader = DnsReader::new(&actual_bytes);

        let parsed = Question::parse(&mut reader);
        assert_eq!(parsed.name.0, message.name.0);
        assert_eq!(parsed.typez, message.typez);
        assert_eq!(parsed.class, message.class);
    }
}
