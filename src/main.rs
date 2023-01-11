fn cypher(input: &str, key: &str) -> String {
    let res = input
        .chars()
        .zip(key.chars())
        .map(|(i, k)| i as u8 ^ k as u8)
        .collect::<Vec<_>>();
    String::from_utf8_lossy(&res).to_string()
}

fn draging(input: &str, key: &str) {
    if input.len() == 0 {
        return;
    }
    let res = cypher(input, key);
    println!("{res}");
    draging(&input[1..], key)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("fixtures/sample0")?;
    let lines = content
        .lines()
        .map(hex::decode)
        .map(Result::unwrap)
        .map(|s| String::from_utf8_lossy(&s).to_string())
        .collect::<Vec<String>>();

    let c1 = cypher(&lines[0], &lines[1]);
    for w in [
        " the ",
        " polkadot ",
        " Bitcoin ",
        "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks",
    ] {
        println!("  Trying {}", w);
        draging(&c1, w);
    }

    for l in lines.iter() {
        println!(
            "{}",
            cypher(
                l,
                &"Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. Digital signatures provide part of the solution, but the main benefits are lost if a trusted third party is still required to prevent double-spending. We propose a solution to the double-spending problem using a peer-to-peer network. The network timestamps transactions by hashing them into an ongoing chain of hash-based proof-of-work, forming a record that cannot be changed without redoing the proof-of-work. The longest chain not only serves as proof of the sequence of events witnessed, but proof that it came from the largest pool of CPU power. As long as a majority of CPU power is controlled by nodes that are not cooperating to attack the network, they'll generate the longest chain and outpace attackers. The network itself requires minimal structure. Messages are broadcast on a best effort basis, and nodes can leave and rejoin the network at will, accepting the longest proof-of-work chain as proof of what happened while they were gone."[..434],
            )
        )
    }
    Ok(())
}
