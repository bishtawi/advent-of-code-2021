use crate::shared;
use anyhow::{bail, Result};
use std::fmt::Write;

const DAY: i32 = 16;

enum PacketType {
    Literal(u64),
    Operator {
        type_id: u8, // 3 bits
        sub_packets: Vec<Packet>,
    },
}

struct Packet {
    version: u32, // 3 bits
    packet_type: PacketType,
}

pub fn solve() -> Result<()> {
    let input = parse()?;
    println!("Day {} = {} :: {}", DAY, part1(&input), part2(&input));

    Ok(())
}

fn decode_hex(s: &str) -> std::result::Result<Vec<u8>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn encode_binary(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 8);
    for &b in bytes {
        write!(&mut s, "{:08b}", b).unwrap();
    }
    s
}

fn parse_packet(binary: &str) -> Result<(Packet, usize)> {
    let mut index = 0;
    let packet = Packet {
        version: u32::from_str_radix(&binary[0..3], 2)?,
        packet_type: match &binary[3..6] {
            "100" => {
                let mut buffer = String::new();
                for i in (6..binary.len()).step_by(5) {
                    buffer.push_str(&binary[i + 1..i + 5]);
                    if binary.chars().nth(i) == Some('0') {
                        index = i + 5;
                        break;
                    }
                }
                let val = u64::from_str_radix(&buffer, 2)?;
                PacketType::Literal(val)
            }
            packet_type_id => {
                let mut sub_packets: Vec<Packet> = Vec::new();
                match binary.chars().nth(6) {
                    Some('0') => {
                        let len = usize::from_str_radix(&binary[7..7 + 15], 2)?;
                        index = 22;
                        while index - 22 < len {
                            let (packet, new_index) = parse_packet(&binary[index..])?;
                            sub_packets.push(packet);
                            index += new_index;
                        }
                    }
                    Some('1') => {
                        let count = u16::from_str_radix(&binary[7..7 + 11], 2)?;
                        index = 18;
                        for _ in 0..count {
                            let (packet, new_index) = parse_packet(&binary[index..])?;
                            sub_packets.push(packet);
                            index += new_index;
                        }
                    }
                    x => bail!("Invalid character {:?}", x),
                };
                PacketType::Operator {
                    type_id: u8::from_str_radix(packet_type_id, 2)?,
                    sub_packets,
                }
            }
        },
    };
    Ok((packet, index))
}

fn parse() -> Result<Packet> {
    let line = shared::read_input_as_string(DAY)?;
    let bytes = decode_hex(&line)?;
    let binary = encode_binary(&bytes);
    let (packet, _) = parse_packet(&binary)?;
    Ok(packet)
}

fn part1(packet: &Packet) -> u32 {
    packet.version
        + match &packet.packet_type {
            PacketType::Literal(_) => 0,
            PacketType::Operator {
                sub_packets,
                type_id: _,
            } => sub_packets.iter().map(part1).sum(),
        }
}

fn part2(packet: &Packet) -> u64 {
    match &packet.packet_type {
        PacketType::Literal(val) => *val,
        PacketType::Operator {
            type_id,
            sub_packets,
        } => {
            match type_id {
                0 => sub_packets.iter().map(part2).sum(),
                1 => sub_packets.iter().map(part2).product(),
                2 => sub_packets.iter().map(part2).min().unwrap(),
                3 => sub_packets.iter().map(part2).max().unwrap(),
                5 => {
                    // greater than
                    assert_eq!(sub_packets.len(), 2);
                    if part2(&sub_packets[0]) > part2(&sub_packets[1]) {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    // less than
                    assert_eq!(sub_packets.len(), 2);
                    if part2(&sub_packets[0]) < part2(&sub_packets[1]) {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    // equal to
                    assert_eq!(sub_packets.len(), 2);
                    if part2(&sub_packets[0]) == part2(&sub_packets[1]) {
                        1
                    } else {
                        0
                    }
                }
                x => panic!("Invalid packet type id {}", x),
            }
        }
    }
}
