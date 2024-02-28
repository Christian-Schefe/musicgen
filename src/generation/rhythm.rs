use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};

fn one_beat() -> Vec<Vec<f64>> {
    vec![vec![1.0], vec![0.5, 0.5]]
}

fn two_beats() -> Vec<Vec<f64>> {
    vec![
        vec![2.0],
        vec![1.5, 0.5],
        vec![1.0, 1.0],
        vec![1.0, 0.5, 0.5],
        vec![0.5, 1.5],
        vec![0.5, 1.0, 0.5],
        vec![0.5, 0.5, 1.0],
        vec![0.5, 0.5, 0.5, 0.5],
    ]
}

fn three_beats() -> Vec<Vec<f64>> {
    let mut v = vec![
        vec![3.0],
        vec![2.5, 0.5],
        vec![2.0, 1.0],
        vec![2.0, 0.5, 0.5],
        vec![1.5, 1.5],
        vec![1.5, 1.0, 0.5],
        vec![1.5, 0.5, 1.0],
        vec![1.5, 0.5, 0.5, 0.5],
        vec![0.5, 2.0, 0.5],
        vec![0.5, 1.5, 1.0],
        vec![0.5, 1.5, 0.5, 0.5],
        vec![0.5, 1.0, 1.0, 0.5],
        vec![0.5, 1.0, 0.5, 1.0],
        vec![0.5, 1.0, 0.5, 0.5, 0.5],
    ];
    v.extend(
        two_beats()
            .into_iter()
            .map(|x| vec![1.0].into_iter().chain(x).collect()),
    );
    v.extend(
        two_beats()
            .into_iter()
            .map(|x| vec![0.5, 0.5].into_iter().chain(x).collect()),
    );
    v
}

fn four_beats() -> Vec<Vec<f64>> {
    let mut v = vec![vec![4.0]];

    let one = one_beat();
    let two = two_beats();
    let three = three_beats();

    for p in two.iter() {
        for p2 in two.iter() {
            v.push(p.iter().chain(p2).cloned().collect());
        }
    }
    for p in one.iter() {
        for p2 in three.iter() {
            v.push(p.iter().chain(p2).cloned().collect());
            v.push(p2.iter().chain(p).cloned().collect());
        }
    }
    v
}

pub fn gen_rythm(rng: &mut ThreadRng) -> Vec<f64> {
    let sections = four_beats();

    sections[rng.gen_range(0..sections.len())].clone()
}
