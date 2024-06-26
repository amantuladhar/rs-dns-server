use crate::dns::header::OpCode;
use crate::dns::{answer::Answer, header::Header, question::Question};

use super::Packet;

#[derive(Default)]
pub struct PacketBuilder {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>,
}

impl PacketBuilder {
    pub fn header(mut self, header: Header) -> Self {
        self.header = header;
        self
    }
    pub fn question(mut self, question: Question) -> Self {
        self.questions.push(question);
        self
    }
    pub fn questions(mut self, questions: Vec<Question>) -> Self {
        self.questions = questions;
        self
    }
    pub fn answers(mut self, answers: Vec<Answer>) -> Self {
        self.answers = answers;
        self
    }
    pub fn answer(mut self, answer: Answer) -> Self {
        self.answers.push(answer);
        self
    }
    pub fn build(self) -> Packet {
        let packet = Packet {
            header: Header {
                qdcount: self.questions.len() as u16,
                ancount: self.answers.len() as u16,
                rcode: if self.header.opcode == OpCode::Query {
                    0
                } else {
                    4
                },
                ..self.header.clone()
            },
            questions: self.questions,
            answers: self.answers,
        };
        packet
    }
}

impl Packet {
    pub fn builder() -> PacketBuilder {
        PacketBuilder::default()
    }
}
