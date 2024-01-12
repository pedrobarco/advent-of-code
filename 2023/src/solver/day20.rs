use std::collections::{HashMap, VecDeque};

use num::integer::lcm;

use super::Solver;

pub struct Day20 {}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum ModuleType {
    Button,
    FlipFlop,
    Broadcast,
    Conjuction,
}

impl ModuleType {
    fn parse(label: &str) -> Self {
        return if label.contains("&") {
            ModuleType::Conjuction
        } else if label.contains("%") {
            ModuleType::FlipFlop
        } else if label == "broadcaster" {
            ModuleType::Broadcast
        } else {
            ModuleType::Button
        };
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Module<'a> {
    label: &'a str,
    mod_type: ModuleType,
    destinations: Vec<&'a str>,
    enabled: bool,
    memory: HashMap<&'a str, Pulse>,
    cycle: usize,
}

impl<'a> Module<'a> {
    fn new(label: &'a str, destinations: Vec<&'a str>) -> Self {
        let mod_type = ModuleType::parse(label);
        let label = match mod_type {
            ModuleType::FlipFlop => label.strip_prefix("%").unwrap(),
            ModuleType::Conjuction => label.strip_prefix("&").unwrap(),
            _ => label,
        };

        Self {
            label,
            mod_type,
            destinations,
            enabled: false,
            memory: HashMap::new(),
            cycle: 0,
        }
    }
}

fn push_button<'a>(
    network: &mut HashMap<&'a str, Module<'a>>,
    feed_label: &str,
    presses: usize,
) -> (usize, usize) {
    let mut hi = 0;
    let mut lo = 0;

    let mut queue: VecDeque<((&str, &str), Pulse)> =
        VecDeque::from([(("button", "broadcaster"), Pulse::Low)]);

    while let Some(((from_label, to_label), pulse)) = queue.pop_front() {
        match pulse {
            Pulse::High => hi += 1,
            Pulse::Low => lo += 1,
        };

        if network.get(to_label).is_none() {
            continue;
        }

        if to_label == feed_label && pulse == Pulse::High {
            let module = network.get_mut(from_label).unwrap();
            if module.cycle == 0 {
                module.cycle = presses.to_owned();
            }
        }

        let module = network.get(to_label).unwrap();
        let destinations = module.destinations.clone();

        let new_pulse: Pulse = match module.mod_type {
            ModuleType::FlipFlop => {
                if pulse == Pulse::High {
                    continue;
                }

                let to = network.get_mut(to_label).unwrap();
                to.enabled = !to.enabled;

                if to.enabled {
                    Pulse::High
                } else {
                    Pulse::Low
                }
            }
            ModuleType::Conjuction => {
                let to = network.get_mut(to_label).unwrap();
                to.memory.insert(from_label, pulse);

                if to.memory.values().all(|&v| v == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            _ => pulse,
        };

        for d in destinations.iter().rev() {
            queue.push_back(((to_label, d), new_pulse));
        }
    }

    return (lo, hi);
}

impl Solver for Day20 {
    fn p1(&self, input: &str) -> String {
        let mut network: HashMap<&str, Module> = HashMap::new();

        let modules: Vec<Module> = input
            .lines()
            .map(|l| {
                let matches: Vec<&str> = l.split("->").map(|x| x.trim()).collect();
                let source = matches[0];
                let destinations = matches[1].split(",").map(|x| x.trim()).collect();
                let module = Module::new(source, destinations);
                return module;
            })
            .collect();

        network.insert("button", Module::new("button", vec!["broadcaster"]));
        modules.iter().for_each(|m| {
            network.insert(m.label, m.to_owned());
        });

        for m in modules.clone().iter() {
            m.destinations.iter().for_each(|d| {
                if let Some(module) = network.get_mut(d) {
                    if module.mod_type == ModuleType::Conjuction {
                        module.memory.insert(m.label, Pulse::Low);
                    }
                }
            })
        }

        let (lo, hi): (usize, usize) = (0..1_000)
            .map(|i| push_button(&mut network, "", i + 1))
            .reduce(|acc, (lo, hi)| (acc.0 + lo, acc.1 + hi))
            .unwrap();

        let res = lo * hi;

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut network: HashMap<&str, Module> = HashMap::new();

        let modules: Vec<Module> = input
            .lines()
            .map(|l| {
                let matches: Vec<&str> = l.split("->").map(|x| x.trim()).collect();
                let source = matches[0];
                let destinations = matches[1].split(",").map(|x| x.trim()).collect();
                let module = Module::new(source, destinations);
                return module;
            })
            .collect();

        network.insert("button", Module::new("button", vec!["broadcaster"]));
        modules.iter().for_each(|m| {
            network.insert(m.label, m.clone());
        });

        for m in modules.clone().iter() {
            m.destinations.iter().for_each(|d| {
                if let Some(module) = network.get_mut(d) {
                    if module.mod_type == ModuleType::Conjuction {
                        module.memory.insert(m.label, Pulse::Low);
                    }
                }
            })
        }

        let feed_label = network
            .iter()
            .find_map(|(_, v)| v.destinations.contains(&"rx").then(|| v.label))
            .unwrap();

        let targets: Vec<&str> = network
            .iter()
            .filter_map(|(_, v)| v.destinations.contains(&feed_label).then(|| v.label))
            .collect();

        let mut presses = 0;
        for _ in 0..20000 {
            presses += 1;
            push_button(&mut network, feed_label, presses);
        }

        let res = targets
            .iter()
            .map(|&k| network.get(k).unwrap())
            .map(|m| m.cycle)
            .fold(1, |acc, x| lcm(acc, x));

        return res.to_string();
    }
}
