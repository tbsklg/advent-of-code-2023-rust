use std::{
    collections::{HashMap, VecDeque},
    default,
};

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
    fn current_pulse(&self) -> Pulse;
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

    fn current_pulse(&self) -> Pulse {
        self.state.clone()
    }

    fn next(&self, pulse: Pulse) -> bool {
        if pulse == Pulse::High {
            return true;
        }

        return false;
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
    fn current_pulse(&self) -> Pulse {
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
        println!("{}: {:?}", name, signal);
        self.inputs.insert(name, signal);

        self.state = match self.inputs.values().all(|x| x == &Pulse::High) {
            true => Pulse::High,
            false => Pulse::Low,
        };

        self.state.clone()
    }

    fn current_pulse(&self) -> Pulse {
        self.state.clone()
    }

    fn memory(&mut self, name: Name) {
        self.inputs.insert(name, Pulse::Low);
    }
}

pub fn pulses(vec: Vec<&str>) -> usize {
    let (mut network, next) = parse(vec);

    let mut low = 0;
    let mut high = 0;

    for (name, destinations) in next.iter() {
        for destination in destinations {
            network.get_mut(destination).map(|x| x.memory(name.clone()));
        }
    }

    for _ in 0..1 {
        let mut queue = VecDeque::new();
        // pushing the butten produces one low pulse
        low += 1;

        queue.push_back(("broadcaster".to_string(), Pulse::Low));
        while let Some((name, pulse)) = queue.pop_front() {
            for n in next.get(&name).unwrap() {
                match pulse {
                    Pulse::Low => low += 1,
                    Pulse::High => high += 1,
                }

                if n == "rx" {
                    continue;
                }

                let module = network.get_mut(n).unwrap();
                // only FlipFlop implements this
                if !module.next(pulse.clone()) {
                    continue;
                }

                let pulse = module.tick(pulse.clone(), name.clone());
                queue.push_back((n.clone(), pulse));
            }
        }
    }
    println!("low: {}, high: {}", low, high);

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
        "broadcaster" => (x.to_string(), Box::new(Broadcaster::default())),
        x => match x.chars().nth(0).unwrap() {
            '%' => (x.replace("%", ""), Box::new(FlipFlop::default())),
            '&' => (x.replace("&", ""), Box::new(Conjunction::default())),
            _ => panic!("Cannot parse module type!"),
        },
    }
}
