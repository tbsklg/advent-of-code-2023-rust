use std::collections::HashMap;

pub fn hash_sum(input: &str) -> u64 {
    input.replace('\n', "").split(',').map(hash_from).sum()
}

pub fn focusing_power(input: &str) -> u64 {
    let focus_map = input.replace('\n', "").split(',').fold(
        HashMap::<u64, Vec<String>>::new(),
        |mut acc, curr| {
            let label = box_index(curr);
            let box_index = hash_from(label);

            if acc.contains_key(&box_index) {
                let lenses = acc.get_mut(&box_index).unwrap();

                if curr.contains('-') {
                    let updated_lenses = remove_lens(lenses.to_vec(), label);
                    acc.insert(box_index, updated_lenses);
                } else {
                    let updated_lenses = update_or_insert(lenses.to_vec(), curr);
                    acc.insert(box_index, updated_lenses);
                }
            } else if curr.contains('=') {
                acc.insert(box_index, vec![curr.to_string()]);
            }
            acc
        },
    );

    calc_power(focus_map)
}

fn calc_power(focus_map: HashMap<u64, Vec<String>>) -> u64 {
    focus_map
        .iter()
        .map(|(key, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, lens)| (key + 1) * (i as u64 + 1) * focus_length(lens))
                .sum::<u64>()
        })
        .sum()
}

fn focus_length(lens: &str) -> u64 {
    lens.split('=').nth(1).unwrap().parse::<u64>().unwrap()
}

fn update_or_insert(mut lenses: Vec<String>, curr: &str) -> Vec<String> {
    let has_label = lenses
        .iter()
        .map(|x| box_index(x))
        .any(|x| x == box_index(curr));

    if has_label {
        lenses
            .iter()
            .map(|x| {
                if box_index(x) == box_index(curr) {
                    curr.to_string()
                } else {
                    x.to_string()
                }
            })
            .collect()
    } else {
        lenses.push(curr.to_string());
        lenses
    }
}

fn remove_lens(lenses: Vec<String>, label: &str) -> Vec<String> {
    lenses
        .iter()
        .filter(|x| box_index(x) != label)
        .map(|x| x.to_string())
        .collect()
}

fn box_index(input: &str) -> &str {
    let x: Vec<&str> = input.clone().split(|x| x == '=' || x == '-').collect();

    x.first().unwrap()
}

fn hash_from(input: &str) -> u64 {
    input
        .chars()
        .fold(0, |acc, curr| ((acc + curr as u64) * 17) % 256)
}

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
