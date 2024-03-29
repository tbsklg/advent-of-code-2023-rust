use std::collections::{HashMap, VecDeque};

use num_integer::lcm;

type Name = String;
type Destinations = Vec<String>;
type Network = HashMap<Name, Box<dyn Module>>;
type Next = HashMap<Name, Destinations>;
type Memory = HashMap<Name, Pulse>;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl Pulse {
    fn invert(&self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

trait Module {
    fn tick(&mut self, pulse: Pulse, next: Name) -> Pulse;
    fn memory(&mut self, _: Name) {}
    fn next(&self, _: Pulse) -> bool {
        true
    }
}

#[derive(Default, Clone, Debug)]
struct FlipFlop {
    state: Pulse,
}

impl Module for FlipFlop {
    fn tick(&mut self, pulse: Pulse, _: Name) -> Pulse {
        if pulse == Pulse::Low {
            self.state = self.state.invert();
        }
        self.state.clone()
    }

    fn next(&self, pulse: Pulse) -> bool {
        pulse != Pulse::High
    }
}

#[derive(Default, Clone, Debug)]
struct Broadcaster {
    state: Pulse,
}

impl Module for Broadcaster {
    fn tick(&mut self, signal: Pulse, _: String) -> Pulse {
        self.state = signal;
        self.state.clone()
    }
}

#[derive(Default, Clone, Debug)]
struct Conjunction {
    inputs: Memory,
    state: Pulse,
}

impl Module for Conjunction {
    fn tick(&mut self, signal: Pulse, name: String) -> Pulse {
        self.inputs.insert(name, signal);

        self.state = match self.inputs.values().all(|x| x == &Pulse::High) {
            true => Pulse::Low,
            false => Pulse::High,
        };

        self.state.clone()
    }

    fn memory(&mut self, name: Name) {
        self.inputs.insert(name, Pulse::Low);
    }
}

fn build_conjunctions(network: &mut Network, next: &Next) {
    for (name, destinations) in next.iter() {
        for destination in destinations {
            if let Some(x) = network.get_mut(destination) {
                x.memory(name.clone())
            }
        }
    }
}

fn value_as_output(input: String, next: &Next) -> Vec<String> {
    next.iter()
        .filter(|(_, v)| v.contains(&input))
        .map(|(k, _)| k.clone())
        .collect()
}

// Solved part two with the help of:
// https://www.youtube.com/watch?v=lxm6i21O83k
pub fn reach_rx_low(vec: Vec<&str>) -> usize {
    let (mut network, next) = parse(vec);

    let feed = value_as_output("rx".to_string(), &next)[0].clone();
    let cycles = value_as_output(feed.to_string(), &next);

    let mut cycle_length: HashMap<String, usize> = HashMap::new();
    let mut seen = cycles
        .iter()
        .map(|v| (v.to_string(), 0))
        .collect::<HashMap<String, usize>>();

    build_conjunctions(&mut network, &next);

    let mut presses = 0;
    loop {
        presses += 1;
        let mut queue = VecDeque::new();

        queue.push_back(("broadcaster".to_string(), Pulse::Low));
        while let Some((name, pulse)) = queue.pop_front() {
            for n in next.get(&name).unwrap() {
                if *n == feed && pulse == Pulse::High {
                    let next_seen = seen.get_mut(&name).unwrap().saturating_add(1);
                    seen.insert(name.clone(), next_seen);

                    cycle_length.insert(name.clone(), presses);

                    if seen.iter().all(|(_, v)| v == &1) {
                        return cycle_length.iter().fold(1, |acc, (_, v)| lcm(acc, *v));
                    }
                }

                if n == "rx" || n == "output" {
                    continue;
                }

                let module = network.get_mut(n).unwrap();
                if !module.next(pulse.clone()) {
                    continue;
                }

                let pulse = module.tick(pulse.clone(), name.clone());
                queue.push_back((n.clone(), pulse));
            }
        }
    }
}

pub fn pulses(vec: Vec<&str>) -> usize {
    let (mut network, next) = parse(vec);

    let mut low = 0;
    let mut high = 0;

    build_conjunctions(&mut network, &next);

    for _ in 0..1_000 {
        let mut queue = VecDeque::new();
        low += 1;

        queue.push_back(("broadcaster".to_string(), Pulse::Low));
        while let Some((name, pulse)) = queue.pop_front() {
            for n in next.get(&name).unwrap() {
                match pulse {
                    Pulse::Low => low += 1,
                    Pulse::High => high += 1,
                }

                if n == "rx" || n == "output" {
                    continue;
                }

                let module = network.get_mut(n).unwrap();
                if !module.next(pulse.clone()) {
                    continue;
                }

                let pulse = module.tick(pulse.clone(), name.clone());
                queue.push_back((n.clone(), pulse));
            }
        }
    }

    low * high
}

fn parse(input: Vec<&str>) -> (Network, Next) {
    let network = input.iter().map(|x| parse_module(x)).collect::<Network>();
    let next = input
        .iter()
        .map(|x| parse_destinations(x))
        .collect::<Next>();

    (network, next)
}

fn parse_module(x: &str) -> (Name, Box<dyn Module>) {
    let mut slices = x.split(" -> ");
    parse_module_type(slices.next().unwrap())
}

fn parse_destinations(x: &str) -> (Name, Vec<String>) {
    let mut slices = x.split(" -> ");
    let name = slices
        .next()
        .unwrap()
        .to_string()
        .replace(['&', '%'].as_ref(), "");

    (
        name,
        slices
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    )
}

fn parse_module_type(x: &str) -> (Name, Box<dyn Module>) {
    match x {
        "broadcaster" => (x.to_string(), Box::<Broadcaster>::default()),
        x => match x.chars().next().unwrap() {
            '%' => (x.replace('%', ""), Box::<FlipFlop>::default()),
            '&' => (x.replace('&', ""), Box::<Conjunction>::default()),
            _ => panic!("Cannot parse module type!"),
        },
    }
}
