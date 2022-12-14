#![feature(let_chains)]

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}

fn parse_packet(txt: &str) -> Option<Packet> {
    let mut iter = txt.chars().filter(|c| !c.is_ascii_whitespace());
    if let Some(c) = iter.next() {
        if c == '[' {
            let mut depth = 1;
            Some(Packet::List(
                txt[1..iter
                    .enumerate()
                    .find(|(_, c)| {
                        depth += if *c == ']' { -1 } else { i32::from(*c == '[') };
                        depth == 0
                    })
                    .unwrap()
                    .0
                    + 1]
                    .split(|c| {
                        depth += if c == ']' { -1 } else { i32::from(c == '[') };
                        c == ',' && depth == 0
                    })
                    .flat_map(|p| parse_packet(p.trim()))
                    .collect(),
            ))
        } else {
            Some(Packet::Integer(
                (format!(
                    "{}{}",
                    c,
                    iter.take_while(|c| *c != ',').collect::<String>()
                ))
                .parse()
                .unwrap(),
            ))
        }
    } else {
        None
    }
}

fn parse_packets(input: &str) -> Vec<[Packet; 2]> {
    input
        .split("\n\n")
        .map(|x| {
            let (a, b) = x.split_once('\n').unwrap();
            [a, b].map(|packet_line| parse_packet(packet_line).unwrap())
        })
        .collect()
}

fn compare_packets(l: &Packet, r: &Packet) -> Option<bool> {
    // println!("l: {l:?}");
    // println!("r: {r:?}\n");
    match (l, r) {
        (Packet::Integer(l), Packet::Integer(r)) => (l != r).then_some(l < r),
        (Packet::List(l), Packet::List(r)) => {
            for (l, r) in l.iter().zip(r.iter()) {
                if let Some(x) = compare_packets(l, r) {
                    return Some(x);
                }
            }
            (l.len() != r.len()).then_some(l.len() < r.len())
        }
        (Packet::List(_), Packet::Integer(_)) => compare_packets(l, &Packet::List(vec![r.clone()])),
        (Packet::Integer(_), Packet::List(_)) => compare_packets(&Packet::List(vec![l.clone()]), r),
    }
}

fn in_order_distress_signals(input: &str) -> u32 {
    parse_packets(input)
        .iter()
        .enumerate()
        .filter_map(|(idx, [p1, p2])| {
            let x = compare_packets(p1, p2).unwrap();
            x.then_some(idx + 1)
        })
        .sum::<usize>() as u32
}

fn decoder_key(input: &str) -> u32 {
    let m1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let m2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    let packets = parse_packets(input);
    let mut packets = packets.iter().flatten().collect::<Vec<_>>();
    packets.push(&m1);
    packets.push(&m2);

    packets.sort_by(|p1, p2| match compare_packets(p1, p2) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Equal,
    });

    ((packets.iter().position(|x| **x == m1).unwrap() + 1)
        * (packets.iter().position(|x| **x == m2).unwrap() + 1)) as u32
}

fn main() {
    println!(
        "sum of in order distress signals: {}",
        in_order_distress_signals(include_str!("input.txt"))
    );
    println!("decoder key is: {}", decoder_key(include_str!("input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(in_order_distress_signals(include_str!("test.txt")), 13);
        assert_eq!(decoder_key(include_str!("test.txt")), 140);
    }
}
