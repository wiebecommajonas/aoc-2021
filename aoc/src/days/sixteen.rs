use libaoc::{Day, DayNumber};

#[derive(Clone, Copy, Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Clone, Copy, Debug)]
enum PacketType {
    Literal,
    Operator(OperatorType),
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    id: u8,
    ty: PacketType,
    subpackets: Option<Vec<Packet>>,
    value: Option<usize>,
}

fn add_versions(packet: &Packet) -> usize {
    let mut version = packet.version as usize;

    if let Some(packets) = &packet.subpackets {
        for p in packets {
            version += add_versions(p);
        }
    }

    version
}

fn calculate_value(packet: &Packet) -> usize {
    match packet.ty {
        PacketType::Literal => packet.value.unwrap(),
        PacketType::Operator(OperatorType::Sum) => {
            let mut sum = 0;
            for p in packet.subpackets.as_ref().unwrap() {
                sum += calculate_value(p);
            }
            sum
        }
        PacketType::Operator(OperatorType::Product) => {
            let mut prod = 1;
            for p in packet.subpackets.as_ref().unwrap() {
                prod *= calculate_value(p);
            }
            prod
        }
        PacketType::Operator(OperatorType::Minimum) => {
            let mut min = usize::MAX;
            for p in packet.subpackets.as_ref().unwrap() {
                let value = calculate_value(p);
                if value < min {
                    min = value;
                }
            }
            min
        }
        PacketType::Operator(OperatorType::Maximum) => {
            let mut max = 0;
            for p in packet.subpackets.as_ref().unwrap() {
                let value = calculate_value(p);
                if value > max {
                    max = value;
                }
            }
            max
        }
        PacketType::Operator(OperatorType::GreaterThan) => {
            let subs = packet.subpackets.as_ref().unwrap();
            if calculate_value(&subs[0]) > calculate_value(&subs[1]) {
                1
            } else {
                0
            }
        }
        PacketType::Operator(OperatorType::LessThan) => {
            let subs = packet.subpackets.as_ref().unwrap();
            if calculate_value(&subs[0]) < calculate_value(&subs[1]) {
                1
            } else {
                0
            }
        }
        PacketType::Operator(OperatorType::EqualTo) => {
            let subs = packet.subpackets.as_ref().unwrap();
            if calculate_value(&subs[0]) == calculate_value(&subs[1]) {
                1
            } else {
                0
            }
        }
    }
}

fn parse_packet(input: &str) -> (Packet, String, usize) {
    let mut remaining = input.to_string();

    let mut length = 6;
    let version = u8::from_str_radix(remaining.drain(0..3).as_str(), 2).unwrap();
    let id = u8::from_str_radix(remaining.drain(0..3).as_str(), 2).unwrap();
    let ty = match id {
        0 => PacketType::Operator(OperatorType::Sum),
        1 => PacketType::Operator(OperatorType::Product),
        2 => PacketType::Operator(OperatorType::Minimum),
        3 => PacketType::Operator(OperatorType::Maximum),
        4 => PacketType::Literal,
        5 => PacketType::Operator(OperatorType::GreaterThan),
        6 => PacketType::Operator(OperatorType::LessThan),
        7 => PacketType::Operator(OperatorType::EqualTo),
        _ => unreachable!(),
    };

    if let PacketType::Operator(_) = ty {
        let length_type_id = &remaining[0..1];
        let mut sub_packets: Vec<Packet> = Vec::new();
        length += 1;

        match length_type_id {
            "0" => {
                length += 15;
                remaining.drain(0..1);
                let length_of_subs =
                    usize::from_str_radix(remaining.drain(0..15).as_str(), 2).unwrap();
                let mut total_length = 0;
                while total_length < length_of_subs {
                    let (packet, string, parsed_length) = parse_packet(&remaining);
                    length += parsed_length;
                    total_length += parsed_length;
                    sub_packets.push(packet);
                    remaining = string;
                }
            }
            "1" => {
                length += 11;
                remaining.drain(0..1);
                let sub_packet_count =
                    usize::from_str_radix(remaining.drain(0..11).as_str(), 2).unwrap();

                for _ in 0..sub_packet_count {
                    let (packet, string, parsed_length) = parse_packet(&remaining);
                    length += parsed_length;
                    sub_packets.push(packet);
                    remaining = string;
                }
            }
            _ => unreachable!(),
        }

        (
            Packet {
                version,
                id,
                ty,
                subpackets: Some(sub_packets),
                value: None,
            },
            remaining,
            length,
        )
    } else {
        let mut literal_bits = String::new();
        loop {
            literal_bits.push_str(&remaining[1..5]);
            if &remaining[0..1] == "0" {
                length += 5;
                remaining.drain(0..5);
                break;
            }
            length += 5;
            remaining.drain(0..5);
        }

        (
            Packet {
                version,
                id,
                ty,
                subpackets: None,
                value: Some(usize::from_str_radix(&literal_bits, 2).unwrap()),
            },
            remaining,
            length,
        )
    }
}

fn parse(input: &str) -> Packet {
    parse_packet(input).0
}

fn bits(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        })
        .collect::<Vec<&str>>()
        .concat()
}

pub fn sixteen() -> Day<2021> {
    Day::new(
        DayNumber::Sixteen,
        |input| Box::new(add_versions(&parse(&bits(input)))),
        |input| Box::new(calculate_value(&parse(&bits(input)))),
    )
}

#[cfg(test)]
mod tests {
    use crate::days::sixteen::calculate_value;

    use super::{add_versions, bits, parse};
    #[test]
    fn example_part1() {
        assert_eq!(add_versions(&parse(&bits("8A004A801A8002F478"))), 16);
        assert_eq!(
            add_versions(&parse(&bits("620080001611562C8802118E34"))),
            12
        );
        assert_eq!(
            add_versions(&parse(&bits("C0015000016115A2E0802F182340"))),
            23
        );
        assert_eq!(
            add_versions(&parse(&bits("A0016C880162017C3686B18A3D4780"))),
            31
        );
    }

    #[test]
    fn example_part2() {
        assert_eq!(calculate_value(&parse(&bits("C200B40A82"))), 3);
        assert_eq!(calculate_value(&parse(&bits("04005AC33890"))), 54);
        assert_eq!(calculate_value(&parse(&bits("880086C3E88112"))), 7);
        assert_eq!(calculate_value(&parse(&bits("CE00C43D881120"))), 9);
        assert_eq!(calculate_value(&parse(&bits("D8005AC2A8F0"))), 1);
        assert_eq!(calculate_value(&parse(&bits("F600BC2D8F"))), 0);
        assert_eq!(calculate_value(&parse(&bits("9C005AC2F8F0"))), 0);
        assert_eq!(
            calculate_value(&parse(&bits("9C0141080250320F1802104A08"))),
            1
        );
    }
}
