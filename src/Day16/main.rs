type Version = u32;

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    Literal(Version, u64),
    Operator(Version, u8, Vec<Packet>),
}

use Packet::*;

fn parse_packet(packet: &str, start_idx: usize) -> (Packet, usize) {
    let packet_bytes = packet.as_bytes();
    let version =
        u32::from_str_radix(&packet[start_idx..start_idx + 3], 2).expect("Invalid version number");
    let packet_id =
        u8::from_str_radix(&packet[start_idx + 3..start_idx + 6], 2).expect("Invalid packet id");
    match packet_id {
        4 => {
            let iter = packet[start_idx + 6..].as_bytes().chunks(5);
            let mut value = 0;
            for (idx, chunk) in iter.enumerate() {
                value <<= 4;
                value += u64::from_str_radix(std::str::from_utf8(&chunk[1..]).unwrap(), 2).unwrap();
                if chunk[0] == b'0' {
                    let remainder_idx = 6 + (idx + 1) * 5;
                    return (Literal(version, value), start_idx + remainder_idx);
                }
            }
            unreachable!()
        }
        _ => {
            if packet_bytes[start_idx + 6] == b'0' {
                let packet_length =
                    usize::from_str_radix(&packet[start_idx + 7..start_idx + 22], 2).unwrap();
                let stop_idx = start_idx + 22 + packet_length;
                let mut packets = Vec::new();
                let mut sub_idx = start_idx + 22;
                loop {
                    let (subpacket, remainder) = parse_packet(packet, sub_idx);
                    packets.push(subpacket);
                    if remainder == stop_idx {
                        return (Operator(version, packet_id, packets), remainder);
                    }
                    sub_idx = remainder;
                }
            } else {
                let num_packets =
                    usize::from_str_radix(&packet[start_idx + 7..start_idx + 18], 2).unwrap();
                let mut packets = Vec::new();
                let mut sub_idx = start_idx + 18;
                for _ in 0..num_packets {
                    let (subpacket, remainder) = parse_packet(packet, sub_idx);
                    packets.push(subpacket);
                    sub_idx = remainder;
                }
                (Operator(version, packet_id, packets), sub_idx)
            }
        }
    }
}

fn sum_version(packet: &Packet) -> u64 {
    match packet {
        Literal(ver, _) => *ver as u64,
        Operator(ver, _, packets) => *ver as u64 + packets.iter().map(sum_version).sum::<u64>(),
    }
}

fn eval(packet: &Packet) -> u64 {
    match packet {
        Literal(_, val) => *val,
        Operator(_, op, packets) => {
            let packets = packets.iter().map(eval);
            match *op {
                0 => packets.sum::<u64>(),
                1 => packets.product::<u64>(),
                2 => packets.min().unwrap(),
                3 => packets.max().unwrap(),
                5 => {
                    let evaled = packets.collect::<Vec<_>>();
                    assert_eq!(evaled.len(), 2);
                    if evaled[0] > evaled[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let evaled = packets.collect::<Vec<_>>();
                    assert_eq!(evaled.len(), 2);
                    if evaled[0] < evaled[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let evaled = packets.collect::<Vec<_>>();
                    assert_eq!(evaled.len(), 2);
                    if evaled[0] == evaled[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn hex_to_bin(s: &str) -> String {
    s
        .bytes()
        .collect::<Vec<_>>()
        .iter()
        .map(|&byte| {
            let val = u64::from_str_radix(std::str::from_utf8(&[byte]).unwrap(), 16).unwrap();
            let s = format!("{:04b}", val);
            s
        })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal() {
        let literal = "110100101111111000101";
        assert_eq!(parse_packet(literal, 0), (Literal(6, 2021), literal.len()));
    }

    #[test]
    fn test_operator_length_type_0() {
        let operator = "0011100000000000011011110100010100101001000100100";
        assert_eq!(
            parse_packet(operator, 0),
            (
                Operator(1, 6, vec![Literal(6, 10), Literal(2, 20)]),
                operator.len()
            )
        );
    }

    #[test]
    fn test_operator_length_type_1() {
        let operator = "111011100000000011010100000011001000001000110000011";
        assert_eq!(
            parse_packet(operator, 0),
            (
                Operator(7, 3, vec![Literal(2, 1), Literal(4, 2), Literal(1, 3)]),
                operator.len()
            )
        );
    }

    #[test]
    fn eval_sum() {
        let packet = parse_packet(&hex_to_bin("C200B40A82"), 0).0;
        assert_eq!(eval(&packet), 3);
    }

    #[test]
    fn eval_product() {
        let packet = parse_packet(&hex_to_bin("04005AC33890"), 0).0;
        assert_eq!(eval(&packet), 54);
    }

    #[test]
    fn eval_min() {
        let packet = parse_packet(&hex_to_bin("880086C3E88112"), 0).0;
        assert_eq!(eval(&packet), 7);
    }

    #[test]
    fn eval_max() {
        let packet = parse_packet(&hex_to_bin("CE00C43D881120"), 0).0;
        assert_eq!(eval(&packet), 9);
    }

    #[test]
    fn eval_le() {
        let packet = parse_packet(&hex_to_bin("D8005AC2A8F0"), 0).0;
        assert_eq!(eval(&packet), 1);
    }

    #[test]
    fn eval_ge() {
        let packet = parse_packet(&hex_to_bin("F600BC2D8F"), 0).0;
        assert_eq!(eval(&packet), 0);
    }

    #[test]
    fn eval_eq() {
        let packet = parse_packet(&hex_to_bin("9C005AC2F8F0"), 0).0;
        assert_eq!(eval(&packet), 0);
    }

    #[test]
    fn eval_multi() {
        let packet = parse_packet(&hex_to_bin("9C0141080250320F1802104A08"), 0).0;
        assert_eq!(eval(&packet), 1);

    }
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt").trim();
    let packet_binary = hex_to_bin(contents);
    let (packet, _) = parse_packet(&packet_binary, 0);
    println!("{}", sum_version(&packet));
    println!("{}", eval(&packet));
    Ok(())
}
