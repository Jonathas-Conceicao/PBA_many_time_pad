fn cypher(input: &[u8], key: &[u8]) -> String {
    let mut res = Vec::default();
    for (i, k) in input.iter().zip(key) {
        res.push(i ^ k)
    }
    let ilen = input.len();
    let klen = key.len();
    let res = String::from_utf8_lossy(&res).to_string();
    if ilen > klen {
        return res + &cypher(&input[klen..], key);
    }
    res
}

fn draging(input: &[u8], key: &[u8]) -> String {
    let mut res = Vec::default();
    for (i, k) in input.iter().zip(key) {
        res.push(i ^ k)
    }
    String::from_utf8_lossy(&res).to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("fixtures/sample0")?;
    let lines = content
        .lines()
        .map(|s| String::from_utf8_lossy(&hex::decode(s).unwrap()).to_string())
        .collect::<Vec<String>>();
    let mut c1 = cypher(lines[0].as_bytes(), lines[1].as_bytes());
    // for w in [
    //     " the ",
    //     " polkadot ",
    //     " bitcoin ",
    //     " brink ",
    //     "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks",
    // ] {
    //     println!("  Trying {}", w);
    //     for i in 0..c1.len() {
    //         println!("{} {:?}", i, cypher(c1[i..].as_bytes(), w.as_bytes()));
    //     }
    // }

    print!("{}", lines.last().unwrap().len());

    for l in lines.iter() {
        println!(
            "{:?}",
            cypher(
                l.as_bytes(),
                "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. Digital signatures provide part of the solution, but the main benefits are lost if a trusted third party is still required to prevent double-spending. We propose a solution to the double-spending problem using a peer-to-peer network. The network timestamps transactions by hashing them into an ongoing chain of hash-based proof-of-work, forming a record that cannot be changed without redoing the proof-of-work. The longest chain not only serves as proof of the sequence of events witnessed, but proof that it came from the largest pool of CPU power. As long as a majority of CPU power is controlled by nodes that are not cooperating to attack the network, they'll generate the longest chain and outpace attackers. The network itself requires minimal structure. Messages are broadcast on a best effort basis, and nodes can leave and rejoin the network at will, accepting the longest proof-of-work chain as proof of what happened while they were gone."[..178].as_bytes(),
            )
        )
    }
    Ok(())
}
