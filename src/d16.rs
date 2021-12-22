use std::sync::Arc;

// decoding bits rust
// |---|---|
// 3bits = version
// 3bits = type_id

// Types
// 4 = literal value

struct Packet {
    version: usize,
    t: PacketType,
}

enum PacketType {
    Literal(LiteralValue),
    Operator(OperatorPacket),
    Comparison(ComparisonPacket),
    Null,
}

struct LiteralValue(i64);

impl LiteralValue {
    pub fn parse_stream(stream: &mut impl Iterator<Item = char>) -> Self {
        LiteralValue(0)
    }
}

struct OperatorPacket {
    operator_init: i64,
    operator_fn: fn(i64, i64) -> i64,
    lhs: Arc<Packet>,
}

impl OperatorPacket {
    pub fn parse_stream(operator_init: i64, stream: &mut impl Iterator<Item = char>) -> Self {
        OperatorPacket { operator_init,
        match operator_init { 0=> ||,1=>,2=>,3=>,_=>} }
    }
}

struct ComparisonPacket {
    comparison_fn: fn(i64, i64) -> i64,
    lhs: Arc<Packet>,
    rhs: Arc<Packet>,
}

impl ComparisonPacket {
    pub fn parse_stream(stream: &mut impl Iterator<Item = char>) -> Self {
        ComparisonPacket {}
    }
}

fn parse_stream(stream: &mut impl Iterator<Item = char>) -> Packet {
    let version: usize = 0;
    let id: i64 = 0;

    let t = match id {
        0 | 1 | 2 | 3 => PacketType::Operator(OperatorPacket::parse_stream(id, stream)),
        4 => PacketType::Literal(LiteralValue::parse_stream(stream)),
        5 | 6 | 7 => PacketType::Comparison(ComparisonPacket::parse_stream(stream)),
        _ => unreachable!(),
    };

    Packet { version, t }
}

pub fn p1(input: &str) -> Result<usize, ()> {
    Ok(0)
}

pub fn p2(input: &str) -> Result<usize, ()> {
    Ok(0)
}

#[cfg(test)]
mod p1_tests {
    use super::*;

    const PROBLEM_INPUT: &str = "E20D41802B2984BD00540010F82D09E35880350D61A41D3004E5611E585F40159ED7AD7C90CF6BD6BE49C802DEB00525272CC1927752698693DA7C70029C0081002140096028C5400F6023C9C00D601ED88070070030005C2201448400E400F40400C400A50801E20004C1000809D14700B67676EE661137ADC64FF2BBAD745B3F2D69026335E92A0053533D78932A9DFE23AC7858C028920A973785338832CFA200F47C81D2BBBC7F9A9E1802FE00ACBA44F4D1E775DDC19C8054D93B7E72DBE7006AA200C41A8510980010D8731720CB80132918319804738AB3A8D3E773C4A4015A498E680292B1852E753E2B29D97F0DE6008CB3D4D031802D2853400D24DEAE0137AB8210051D24EB600844B95C56781B3004F002B99D8F635379EDE273AF26972D4A5610BA51004C12D1E25D802F32313239377B37100105343327E8031802B801AA00021D07231C2F10076184668693AC6600BCD83E8025231D752E5ADE311008A4EA092754596C6789727F069F99A4645008247D2579388DCF53558AE4B76B257200AAB80107947E94789FE76E36402868803F0D62743F00043A1646288800084C3F8971308032996A2BD8023292DF8BE467BB3790047F2572EF004A699E6164C013A007C62848DE91CC6DB459B6B40087E530AB31EE633BD23180393CBF36333038E011CBCE73C6FB098F4956112C98864EA1C2801D2D0F319802D60088002190620E479100622E4358952D84510074C0188CF0923410021F1CE1146E3006E3FC578EE600A4B6C4B002449C97E92449C97E92459796EB4FF874400A9A16100A26CEA6D0E5E5EC8841C9B8FE37109C99818023A00A4FD8BA531586BB8B1DC9AE080293B6972B7FA444285CC00AE492BC910C1697B5BDD8425409700562F471201186C0120004322B42489A200D4138A71AA796D00374978FE07B2314E99BFB6E909678A0";

    macro_rules! p1_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, p1(input));
            }
        )*
        }
    }

    p1_tests! {
        p1_0: ("8A004A801A8002F478", Ok(16)),
        p1_1: ("620080001611562C8802118E34", Ok(12)),
        p1_2: ("C0015000016115A2E0802F182340", Ok(23)),
        p1_3: ("A0016C880162017C3686B18A3D4780", Ok(31)),
        p1_problem: (PROBLEM_INPUT, Ok(31)),
    }
}

#[cfg(test)]
mod p2_tests {
    use super::*;

    const PROBLEM_INPUT: &str = "E20D41802B2984BD00540010F82D09E35880350D61A41D3004E5611E585F40159ED7AD7C90CF6BD6BE49C802DEB00525272CC1927752698693DA7C70029C0081002140096028C5400F6023C9C00D601ED88070070030005C2201448400E400F40400C400A50801E20004C1000809D14700B67676EE661137ADC64FF2BBAD745B3F2D69026335E92A0053533D78932A9DFE23AC7858C028920A973785338832CFA200F47C81D2BBBC7F9A9E1802FE00ACBA44F4D1E775DDC19C8054D93B7E72DBE7006AA200C41A8510980010D8731720CB80132918319804738AB3A8D3E773C4A4015A498E680292B1852E753E2B29D97F0DE6008CB3D4D031802D2853400D24DEAE0137AB8210051D24EB600844B95C56781B3004F002B99D8F635379EDE273AF26972D4A5610BA51004C12D1E25D802F32313239377B37100105343327E8031802B801AA00021D07231C2F10076184668693AC6600BCD83E8025231D752E5ADE311008A4EA092754596C6789727F069F99A4645008247D2579388DCF53558AE4B76B257200AAB80107947E94789FE76E36402868803F0D62743F00043A1646288800084C3F8971308032996A2BD8023292DF8BE467BB3790047F2572EF004A699E6164C013A007C62848DE91CC6DB459B6B40087E530AB31EE633BD23180393CBF36333038E011CBCE73C6FB098F4956112C98864EA1C2801D2D0F319802D60088002190620E479100622E4358952D84510074C0188CF0923410021F1CE1146E3006E3FC578EE600A4B6C4B002449C97E92449C97E92459796EB4FF874400A9A16100A26CEA6D0E5E5EC8841C9B8FE37109C99818023A00A4FD8BA531586BB8B1DC9AE080293B6972B7FA444285CC00AE492BC910C1697B5BDD8425409700562F471201186C0120004322B42489A200D4138A71AA796D00374978FE07B2314E99BFB6E909678A0";

    macro_rules! p2_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, p2(input));
            }
        )*
        }
    }

    p2_tests! {
        p1_0: ("8A004A801A8002F478", Ok(16)),
        p1_1: ("620080001611562C8802118E34", Ok(12)),
        p1_2: ("C0015000016115A2E0802F182340", Ok(23)),
        p1_3: ("A0016C880162017C3686B18A3D4780", Ok(31)),
        p1_problem: (PROBLEM_INPUT, Ok(31)),
    }
}