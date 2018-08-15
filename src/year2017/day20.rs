use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use scan_rules::ScanError;

type V3 = [i64; 3];

#[derive(Debug, Clone)]
struct Particle {
    p: V3,
    v: V3,
    a: V3,
}

impl Particle {
    fn update(&mut self) {
        for (vc, ac) in self.v.iter_mut().zip(self.a.iter()) {
            *vc += ac;
        }
        for (pc, vc) in self.p.iter_mut().zip(self.v.iter()) {
            *pc += vc;
        }
    }
}

pub fn solve() -> Result<(usize, usize), Box<Error>> {
    let particles = input()?;

    Ok((part1(&particles), part2(&particles)))
}

fn input() -> Result<Vec<Particle>, Box<Error>> {
    let file = File::open("input/2017/20.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .flat_map(|x| x)
        .map(|line| parse(&line))
        .collect::<Result<_, _>>()?)
}

fn part1(particles: &[Particle]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, particle)| {
            let [ax, ay, az] = particle.a;
            ax.abs() + ay.abs() + az.abs()
        })
        .unwrap()
        .0
}

fn part2(particles: &[Particle]) -> usize {
    let mut particles = particles.to_vec();
    let mut seen = HashSet::new();
    let mut remove = HashSet::new();

    for _ in 0..1000 {
        for particle in &mut particles {
            particle.update();
            if !seen.insert(particle.p) {
                remove.insert(particle.p);
            }
        }
        seen.clear();
        particles.retain(|p| !remove.contains(&p.p));
    }

    particles.len()
}

fn parse(text: &str) -> Result<Particle, ScanError> {
    scan!(text;
        ("p=<", let px, ",", let py, ",", let pz, ">,",
         "v=<", let vx, ",", let vy, ",", let vz, ">,",
         "a=<", let ax, ",", let ay, ",", let az, ">")
            => Particle {
                p: [px, py, pz],
                v: [vx, vy, vz],
                a: [ax, ay, az],
            }
    )
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (157, 499));
}
