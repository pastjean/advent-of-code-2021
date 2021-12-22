use std::sync::Arc;

fn hex_to_binary(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            c if c.is_digit(10) => {
                let mut binary = format!("{:b}", c.to_digit(10).unwrap());
                let len = binary.chars().count();
                for _ in len..4 {
                    binary = ["0", &binary].join("");
                }
                binary
            }
            'A' => "1010".to_string(),
            'B' => "1011".to_string(),
            'C' => "1100".to_string(),
            'D' => "1101".to_string(),
            'E' => "1110".to_string(),
            'F' => "1111".to_string(),
            _ => panic!("Invalid hex character: {}", c),
        })
        .collect::<String>()
}

fn binary_to_decimal(binary: &str) -> i64 {
    isize::from_str_radix(binary, 2).unwrap() as i64
}

#[derive(Clone)]
enum Packet {
    Literal(usize, i64),
    Operator(usize, (i64, fn(i64, i64) -> i64), Vec<Packet>),
    Comparison(usize, fn(i64, i64) -> i64, Arc<Packet>, Arc<Packet>),
}

fn parse_packet(stream: &mut impl Iterator<Item = char>) -> Packet {
    let version = binary_to_decimal(&stream.take(3).collect::<String>()) as usize;
    let id = binary_to_decimal(&stream.take(3).collect::<String>());
    match id {
        4 => {
            let value = parse_value(stream);
            Packet::Literal(version, value)
        }
        _ => {
            let subpackets = parse_subpackets(stream);
            match id {
                0 | 1 | 2 | 3 => Packet::Operator(
                    version,
                    match id {
                        0 => (0, |acc, x| -> i64 { acc + x }),
                        1 => (1, |acc, x| -> i64 { acc * x }),
                        2 => (i64::MAX, |acc, x| -> i64 { std::cmp::min(acc, x) }),
                        3 => (i64::MIN, |acc, x| -> i64 { std::cmp::max(acc, x) }),
                        _ => unreachable!(),
                    },
                    subpackets,
                ),
                5 | 6 | 7 => Packet::Comparison(
                    version,
                    match id {
                        5 => |x, y| -> i64 {
                            if x > y {
                                1
                            } else {
                                0
                            }
                        },
                        6 => |x, y| -> i64 {
                            if x < y {
                                1
                            } else {
                                0
                            }
                        },
                        7 => |x, y| -> i64 {
                            if x == y {
                                1
                            } else {
                                0
                            }
                        },
                        _ => unreachable!(),
                    },
                    Arc::new(subpackets[0].clone()),
                    Arc::new(subpackets[1].clone()),
                ),
                _ => unreachable!(),
            }
        }
    }
}

fn parse_value(stream: &mut impl Iterator<Item = char>) -> i64 {
    let mut binary = String::new();
    while {
        // append the next 4 chars to the binary string
        let c = stream.next().unwrap();
        binary.push_str(&stream.take(4).collect::<String>());
        // only continue if we have encountered a '1' before the 4 bits
        c == '1'
    } {}
    binary_to_decimal(&binary)
}

fn parse_subpackets(stream: &mut impl Iterator<Item = char>) -> Vec<Packet> {
    let length_type = stream.next().unwrap();
    match length_type {
        '0' => {
            let total_length = binary_to_decimal(&stream.take(15).collect::<String>()) as usize;
            let anticipated = stream.take(total_length).collect::<String>();
            let mut anticipated = anticipated.chars().peekable();

            let mut subpackets = Vec::new();
            while anticipated.peek().is_some() {
                let packet = parse_packet(&mut anticipated);
                subpackets.push(packet);
            }
            subpackets
        }
        '1' => {
            let num_packets = binary_to_decimal(&stream.take(11).collect::<String>());
            let mut subpackets = Vec::new();
            for _ in 0..num_packets {
                let packet = parse_packet(stream);
                subpackets.push(packet);
            }
            subpackets
        }
        _ => unreachable!(),
    }
}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(version, _) => *version,
        Packet::Operator(version, _, subpackets) => {
            version + subpackets.iter().map(sum_versions).sum::<usize>()
        }
        Packet::Comparison(version, _, a, b) => version + sum_versions(a) + sum_versions(b),
    }
}

fn part1(input: &str) -> i64 {
    let binary = hex_to_binary(input);
    let mut stream = binary.chars();
    let packet = parse_packet(&mut stream);
    sum_versions(&packet) as i64
}

fn fold_packet(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal(_, value) => *value,
        Packet::Operator(_, (initial, f), subpackets) => {
            subpackets.iter().map(fold_packet).fold(*initial, f)
        }
        Packet::Comparison(_, f, a, b) => {
            let a = fold_packet(a);
            let b = fold_packet(b);
            f(a, b)
        }
    }
}

fn part2(input: &str) -> i64 {
    let binary = hex_to_binary(input);
    let mut stream = binary.chars();
    let packet = parse_packet(&mut stream);
    fold_packet(&packet)
}

#[cfg(test)]
mod day16 {
    use super::*;
    const SOLUTION: (i64, i64) = (984, 1015320896946);

    const PROBLEM_INPUT: &str = "E20D41802B2984BD00540010F82D09E35880350D61A41D3004E5611E585F40159ED7AD7C90CF6BD6BE49C802DEB00525272CC1927752698693DA7C70029C0081002140096028C5400F6023C9C00D601ED88070070030005C2201448400E400F40400C400A50801E20004C1000809D14700B67676EE661137ADC64FF2BBAD745B3F2D69026335E92A0053533D78932A9DFE23AC7858C028920A973785338832CFA200F47C81D2BBBC7F9A9E1802FE00ACBA44F4D1E775DDC19C8054D93B7E72DBE7006AA200C41A8510980010D8731720CB80132918319804738AB3A8D3E773C4A4015A498E680292B1852E753E2B29D97F0DE6008CB3D4D031802D2853400D24DEAE0137AB8210051D24EB600844B95C56781B3004F002B99D8F635379EDE273AF26972D4A5610BA51004C12D1E25D802F32313239377B37100105343327E8031802B801AA00021D07231C2F10076184668693AC6600BCD83E8025231D752E5ADE311008A4EA092754596C6789727F069F99A4645008247D2579388DCF53558AE4B76B257200AAB80107947E94789FE76E36402868803F0D62743F00043A1646288800084C3F8971308032996A2BD8023292DF8BE467BB3790047F2572EF004A699E6164C013A007C62848DE91CC6DB459B6B40087E530AB31EE633BD23180393CBF36333038E011CBCE73C6FB098F4956112C98864EA1C2801D2D0F319802D60088002190620E479100622E4358952D84510074C0188CF0923410021F1CE1146E3006E3FC578EE600A4B6C4B002449C97E92449C97E92459796EB4FF874400A9A16100A26CEA6D0E5E5EC8841C9B8FE37109C99818023A00A4FD8BA531586BB8B1DC9AE080293B6972B7FA444285CC00AE492BC910C1697B5BDD8425409700562F471201186C0120004322B42489A200D4138A71AA796D00374978FE07B2314E99BFB6E909678A0";

    #[test]
    fn test() {
        let result = (part1(PROBLEM_INPUT), part2(PROBLEM_INPUT));
        assert_eq!(result, SOLUTION);
    }

    #[test]
    fn test_decode_hex() {
        let input = "0123456789ABCDEF";
        assert_eq!(
            "0000000100100011010001010110011110001001101010111100110111101111",
            hex_to_binary(input)
        );
    }

    #[test]
    fn test_example1() {
        let input = "8A004A801A8002F478";
        assert_eq!(part1(&input), 16);
    }

    #[test]
    fn test_example2() {
        let input = "620080001611562C8802118E34";
        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn test_example3() {
        let input = "C0015000016115A2E0802F182340";
        assert_eq!(part1(&input), 23);
    }

    #[test]
    fn test_example4() {
        let input = "A0016C880162017C3686B18A3D4780";
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_example5() {
        let input = "38006F45291200";
        assert_eq!(part1(&input), 9);
    }
}
