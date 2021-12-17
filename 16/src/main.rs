struct Header {
    version: u8,
    packet_type: u8,
}

struct Literal {
    header: Header,
    value: u64,
}

struct Operator {
    header: Header,
    packets: Vec<usize>,
}

enum Packet {
    Literal(Literal),
    Operator(Operator),
}

struct Transmission {
    nodes: Vec<Packet>,
    root_node: usize,
}

impl Transmission {
    fn sum_version_numbers(&self) -> u32 {
        let packet = &self.nodes[self.root_node];
        self.sum_versions(packet)
    }

    fn sum_versions(&self, packet: &Packet) -> u32 {
        match packet {
            Packet::Literal(literal) => literal.header.version as u32,
            Packet::Operator(operator) => operator
                .packets
                .iter()
                .map(|&index| self.sum_versions(&self.nodes[index]))
                .fold(operator.header.version as u32, |accum, curr| accum + curr),
        }
    }

    fn calc_expression(&self) -> u64 {
        let packet = &self.nodes[self.root_node];
        self.calc_subexpression(packet)
    }

    fn calc_subexpression(&self, packet: &Packet) -> u64 {
        match packet {
            Packet::Literal(literal) => literal.value,
            Packet::Operator(operator) => {
                let mut iter = operator
                    .packets
                    .iter()
                    .map(|&index| self.calc_subexpression(&self.nodes[index]));

                match operator.header.packet_type {
                    0 => iter.fold(0, |acc, curr| acc + curr),
                    1 => iter.fold(1, |acc, curr| acc * curr),
                    2 => iter.min().unwrap_or(0),
                    3 => iter.max().unwrap_or(0),
                    5 => {
                        let first = iter.next().unwrap();
                        let second = iter.next().unwrap();
                        (first > second) as u64
                    }
                    6 => {
                        let first = iter.next().unwrap();
                        let second = iter.next().unwrap();
                        (first < second) as u64
                    }
                    7 => {
                        let first = iter.next().unwrap();
                        let second = iter.next().unwrap();
                        (first == second) as u64
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

struct TransmissionParser {
    bytes: Vec<u8>,
    bit_pos: usize,
    nodes: Vec<Packet>,
    root_node: usize,
}

impl TransmissionParser {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            bit_pos: 0,
            nodes: Vec::new(),
            root_node: 0,
        }
    }

    fn parse(mut self) -> Transmission {
        self.root_node = self.parse_packet();

        Transmission {
            nodes: self.nodes,
            root_node: self.root_node,
        }
    }

    fn parse_packet(&mut self) -> usize {
        let header = self.read_header();
        match header.packet_type {
            4 => self.read_literal(header),
            _ => self.read_operator(header),
        }
    }

    fn read_header(&mut self) -> Header {
        let version = self.read_u8(3);
        let packet_type = self.read_u8(3);
        Header {
            version,
            packet_type,
        }
    }

    fn read_literal(&mut self, header: Header) -> usize {
        let mut value = 0;

        loop {
            let has_next = self.read_u8(1);

            let chunk = self.read_u8(4) as u64;
            value = (value << 4) | chunk;

            if has_next == 0 {
                break;
            }
        }

        let literal = Literal { header, value };
        let packet = Packet::Literal(literal);
        let index = self.nodes.len();
        self.nodes.push(packet);

        index
    }

    fn read_operator(&mut self, header: Header) -> usize {
        match self.read_u8(1) {
            0 => self.read_op_by_len(header),
            1 => self.read_op_by_count(header),
            _ => unreachable!(),
        }
    }

    fn read_op_by_len(&mut self, header: Header) -> usize {
        let content_width = self.read_u16(15) as usize;
        let end_pos = self.bit_pos + content_width;

        let mut operator = Operator {
            header,
            packets: Vec::new(),
        };

        while self.bit_pos < end_pos {
            let child_index = self.parse_packet();
            operator.packets.push(child_index);
        }

        let packet = Packet::Operator(operator);
        let index = self.nodes.len();
        self.nodes.push(packet);

        index
    }

    fn read_op_by_count(&mut self, header: Header) -> usize {
        let num_packets = self.read_u16(11);

        let mut operator = Operator {
            header,
            packets: Vec::new(),
        };

        for _ in 0..num_packets {
            let child_index = self.parse_packet();
            operator.packets.push(child_index);
        }

        let packet = Packet::Operator(operator);
        let index = self.nodes.len();
        self.nodes.push(packet);

        index
    }

    fn read_u8(&mut self, n: usize) -> u8 {
        let mut value = 0;

        for _ in 0..n {
            let index = self.bit_pos / 8;
            let shift = 8 - (self.bit_pos % 8) - 1;

            let current_bit = (self.bytes[index] & (1 << shift)) >> shift;
            value = (value << 1) | current_bit;
            self.bit_pos += 1;
        }

        value
    }

    fn read_u16(&mut self, n: usize) -> u16 {
        let mut value = 0;

        for _ in 0..n {
            let index = self.bit_pos / 8;
            let shift = 8 - (self.bit_pos % 8) - 1;

            let current_bit = ((self.bytes[index] & (1 << shift)) >> shift) as u16;
            value = (value << 1) | current_bit;
            self.bit_pos += 1;
        }

        value
    }
}

fn get_data() -> Vec<u8> {
    let input_str = include_str!("./input.txt");
    process_input(input_str)
}

fn process_input(input_str: &str) -> Vec<u8> {
    (0..input_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input_str[i..i + 2], 16).unwrap())
        .collect()
}

fn main() {
    let bytes = get_data();
    let transmission = TransmissionParser::new(bytes).parse();
    let versions_total = transmission.sum_version_numbers();
    println!("Versions total: {}", versions_total);
    let expression_result = transmission.calc_expression();
    println!("Expression result: {}", expression_result);
}

#[cfg(test)]
mod tests {
    use crate::{process_input, TransmissionParser};

    #[test]
    fn test_version_numbers() {
        let bytes_a = process_input("8A004A801A8002F478");
        let version_sum_a = TransmissionParser::new(bytes_a)
            .parse()
            .sum_version_numbers();

        assert_eq!(version_sum_a, 16);

        let bytes_b = process_input("620080001611562C8802118E34");
        let version_sum_b = TransmissionParser::new(bytes_b)
            .parse()
            .sum_version_numbers();

        assert_eq!(version_sum_b, 12);

        let bytes_c = process_input("C0015000016115A2E0802F182340");
        let version_sum_c = TransmissionParser::new(bytes_c)
            .parse()
            .sum_version_numbers();

        assert_eq!(version_sum_c, 23);

        let bytes_d = process_input("A0016C880162017C3686B18A3D4780");
        let version_sum_d = TransmissionParser::new(bytes_d)
            .parse()
            .sum_version_numbers();

        assert_eq!(version_sum_d, 31);
    }

    #[test]
    fn test_value_extraction() {
        let bytes = process_input("D2FE28");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 2021);
    }

    #[test]
    fn test_sum() {
        let bytes = process_input("C200B40A82");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 3);
    }

    #[test]
    fn test_product() {
        let bytes = process_input("04005AC33890");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 54);
    }

    #[test]
    fn test_minimum() {
        let bytes = process_input("880086C3E88112");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_maximum() {
        let bytes = process_input("CE00C43D881120");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 9);
    }

    #[test]
    fn test_less_than_true() {
        let bytes = process_input("D8005AC2A8F0");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_less_than() {
        let bytes = process_input("F600BC2D8F");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_greater_than() {
        let bytes = process_input("F600BC2D8F");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_equal() {
        let bytes = process_input("9C005AC2F8F0");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_hierarchy() {
        let bytes = process_input("9C0141080250320F1802104A08");
        let result = TransmissionParser::new(bytes).parse().calc_expression();

        assert_eq!(result, 1);
    }
}
