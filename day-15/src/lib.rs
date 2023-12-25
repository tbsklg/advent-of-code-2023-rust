#[test]
fn should_calculate_hash() {
    let input = "HASH";

    let hash = hash_from(input);

    assert_eq!(hash, 52);
}

#[test]
fn should_calculate_sum_of_seqence() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let hash = hash_sum(input);

    assert_eq!(hash, 1320);
}

pub fn hash_sum(input: &str) -> u64 {
    input.split(',').map(hash_from).sum()
}

fn hash_from(input: &str) -> u64 {
    input.chars().fold(0, |acc, curr| {
        if curr == '\n' {
            return acc;
        }
        ((acc + curr as u64) * 17) % 256
    })
}
