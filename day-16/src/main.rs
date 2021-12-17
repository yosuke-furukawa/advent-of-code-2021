#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    value: u64,
    sub_packets: Vec<Packet>,
}

fn parse(arg: &str) -> Vec<u8> {
    arg.chars()
        .map(|c| match c {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("no such char"),
        })
        .flatten()
        .collect()
}

fn decode_from_binary(bin: &[u8]) -> u64 {
    bin.iter().enumerate().rev().fold(0, |acc, (pos, num)| {
        acc + *num as u64 * 2_u64.pow(bin.len() as u32 - 1 - pos as u32)
    })
}

fn parse_packet(bin: &[u8], start: usize, ignore_leading_zero: bool) -> Option<(Packet, usize)> {
    if start >= bin.len() - 1 {
        return None;
    }
    let mut pos = start;
    let version = decode_from_binary(&bin[pos..pos + 3]);
    pos += 3;
    let type_id = decode_from_binary(&bin[pos..pos + 3]);
    pos += 3;

    let value: u64;
    let mut sub_packets = vec![];
    if type_id == 4 {
        let mut binary_value = vec![];
        let mut last = false;
        while !last {
            last = bin[pos] == 0;
            for b in bin[pos + 1..pos + 5].iter() {
                binary_value.push(*b);
            }
            pos += 5;
        }
        value = decode_from_binary(&binary_value);
    } else {
        let length_type_id = bin[pos];
        pos += 1;
        if length_type_id == 0 {
            // length version parse
            let len = decode_from_binary(&bin[pos..pos + 15]);
            pos += 15;
            let end = pos + len as usize;

            while pos < end {
                if let Some((sub_packet, pos2)) = parse_packet(bin, pos, false) {
                    pos = pos2;
                    sub_packets.push(sub_packet);
                }
            }
        } else {
            // count version parse
            let count = decode_from_binary(&bin[pos..pos + 11]);
            pos += 11;
            for _ in 0..count {
                if let Some((sub_packet, pos2)) = parse_packet(bin, pos, false) {
                    pos = pos2;
                    sub_packets.push(sub_packet);
                }
            }
        }
        value = op_sub_packets(type_id, &sub_packets);
    }
    if ignore_leading_zero {
        while (pos - start) % 8 != 0 {
            pos += 1;
        }
    }
    Some((
        Packet {
            version,
            type_id,
            value,
            sub_packets,
        },
        pos,
    ))
}

fn op_sub_packets(type_id: u64, sub_packets: &[Packet]) -> u64 {
    match type_id {
        0 => sub_packets.iter().map(|p| p.value).sum(),
        1 => sub_packets.iter().map(|p| p.value).product(),
        2 => sub_packets
            .iter()
            .map(|p| p.value)
            .fold(std::u64::MAX, |acc, cur| acc.min(cur)),
        3 => sub_packets
            .iter()
            .map(|p| p.value)
            .fold(std::u64::MIN, |acc, cur| acc.max(cur)),
        5 => {
            let values: Vec<u64> = sub_packets.iter().map(|p| p.value).collect();
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            let values: Vec<u64> = sub_packets.iter().map(|p| p.value).collect();
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            let values: Vec<u64> = sub_packets.iter().map(|p| p.value).collect();
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("no such id"),
    }
}

fn sum_version(packet: &Packet) -> u64 {
    let mut result = packet.version;
    for sub in packet.sub_packets.iter() {
        result += sum_version(sub);
    }
    result
}

fn part1(arg: &str) -> u64 {
    let bin = parse(arg);
    let mut result = 0;
    let mut start = 0;
    while let Some((packet, pos)) = parse_packet(&bin, start, true) {
        result += sum_version(&packet);
        start += pos;
    }
    result
}

fn part2(arg: &str) -> u64 {
    let bin = parse(arg);
    if let Some((packet, _)) = parse_packet(&bin, 0, true) {
        packet.value
    } else {
        0
    }
}

fn main() {
    println!("{:?}", part1("8A004A801A8002F478"));
    println!("{:?}", part1("620080001611562C8802118E34"));
    println!("{:?}", part1("C0015000016115A2E0802F182340"));
    println!("{:?}", part1("A0016C880162017C3686B18A3D4780"));
    println!("{:?}", part1("005532447836402684AC7AB3801A800021F0961146B1007A1147C89440294D005C12D2A7BC992D3F4E50C72CDF29EECFD0ACD5CC016962099194002CE31C5D3005F401296CAF4B656A46B2DE5588015C913D8653A3A001B9C3C93D7AC672F4FF78C136532E6E0007FCDFA975A3004B002E69EC4FD2D32CDF3FFDDAF01C91FCA7B41700263818025A00B48DEF3DFB89D26C3281A200F4C5AF57582527BC1890042DE00B4B324DBA4FAFCE473EF7CC0802B59DA28580212B3BD99A78C8004EC300761DC128EE40086C4F8E50F0C01882D0FE29900A01C01C2C96F38FCBB3E18C96F38FCBB3E1BCC57E2AA0154EDEC45096712A64A2520C6401A9E80213D98562653D98562612A06C0143CB03C529B5D9FD87CBA64F88CA439EC5BB299718023800D3CE7A935F9EA884F5EFAE9E10079125AF39E80212330F93EC7DAD7A9D5C4002A24A806A0062019B6600730173640575A0147C60070011FCA005000F7080385800CBEE006800A30C023520077A401840004BAC00D7A001FB31AAD10CC016923DA00686769E019DA780D0022394854167C2A56FB75200D33801F696D5B922F98B68B64E02460054CAE900949401BB80021D0562344E00042A16C6B8253000600B78020200E44386B068401E8391661C4E14B804D3B6B27CFE98E73BCF55B65762C402768803F09620419100661EC2A8CE0008741A83917CC024970D9E718DD341640259D80200008444D8F713C401D88310E2EC9F20F3330E059009118019A8803F12A0FC6E1006E3744183D27312200D4AC01693F5A131C93F5A131C970D6008867379CD3221289B13D402492EE377917CACEDB3695AD61C939C7C10082597E3740E857396499EA31980293F4FD206B40123CEE27CFB64D5E57B9ACC7F993D9495444001C998E66B50896B0B90050D34DF3295289128E73070E00A4E7A389224323005E801049351952694C000"));
    println!("{:?}", part2("C200B40A82"));
    println!("{:?}", part2("04005AC33890"));
    println!("{:?}", part2("880086C3E88112"));
    println!("{:?}", part2("CE00C43D881120"));
    println!("{:?}", part2("D8005AC2A8F0"));
    println!("{:?}", part2("F600BC2D8F"));
    println!("{:?}", part2("9C005AC2F8F0"));
    println!("{:?}", part2("9C0141080250320F1802104A08"));
    println!("{:?}", part2("005532447836402684AC7AB3801A800021F0961146B1007A1147C89440294D005C12D2A7BC992D3F4E50C72CDF29EECFD0ACD5CC016962099194002CE31C5D3005F401296CAF4B656A46B2DE5588015C913D8653A3A001B9C3C93D7AC672F4FF78C136532E6E0007FCDFA975A3004B002E69EC4FD2D32CDF3FFDDAF01C91FCA7B41700263818025A00B48DEF3DFB89D26C3281A200F4C5AF57582527BC1890042DE00B4B324DBA4FAFCE473EF7CC0802B59DA28580212B3BD99A78C8004EC300761DC128EE40086C4F8E50F0C01882D0FE29900A01C01C2C96F38FCBB3E18C96F38FCBB3E1BCC57E2AA0154EDEC45096712A64A2520C6401A9E80213D98562653D98562612A06C0143CB03C529B5D9FD87CBA64F88CA439EC5BB299718023800D3CE7A935F9EA884F5EFAE9E10079125AF39E80212330F93EC7DAD7A9D5C4002A24A806A0062019B6600730173640575A0147C60070011FCA005000F7080385800CBEE006800A30C023520077A401840004BAC00D7A001FB31AAD10CC016923DA00686769E019DA780D0022394854167C2A56FB75200D33801F696D5B922F98B68B64E02460054CAE900949401BB80021D0562344E00042A16C6B8253000600B78020200E44386B068401E8391661C4E14B804D3B6B27CFE98E73BCF55B65762C402768803F09620419100661EC2A8CE0008741A83917CC024970D9E718DD341640259D80200008444D8F713C401D88310E2EC9F20F3330E059009118019A8803F12A0FC6E1006E3744183D27312200D4AC01693F5A131C93F5A131C970D6008867379CD3221289B13D402492EE377917CACEDB3695AD61C939C7C10082597E3740E857396499EA31980293F4FD206B40123CEE27CFB64D5E57B9ACC7F993D9495444001C998E66B50896B0B90050D34DF3295289128E73070E00A4E7A389224323005E801049351952694C000"));
}
