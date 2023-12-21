use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/input.txt").expect("could not read file");
    let m = modules_from_str(&input);
    let (low, high) = press_button_a_thousand_times(m);
    println!("{} {} = {}", low, high, low * high);
}

#[derive(Debug, Clone, PartialEq)]
enum Module {
    FlipFlop(FlipFlop),
    Conj(Conj),
    Broadcaster(Broadcaster),
}

impl Module {
    fn targets(&self) -> Vec<String> {
        match self {
            Module::FlipFlop(m) => m.targets.clone(),
            Module::Conj(m) => m.targets.clone(),
            Module::Broadcaster(b) => b.targets.clone(),
        }
    }
    fn value(&self) -> bool {
        match self {
            Module::FlipFlop(m) => m.value,
            Module::Conj(m) => m.value,
            Module::Broadcaster(b) => false,
        }
    }
    fn accept(&self, source: String, signal: bool) -> (Module, bool) {
        match self {
            Module::FlipFlop(m) => {
                if signal {
                    return (Module::FlipFlop(m.clone()), false);
                }

                return (
                    Module::FlipFlop(FlipFlop {
                        value: !m.value,
                        ..m.clone()
                    }),
                    true,
                );
            }
            Module::Conj(c) => {
                let mut new_c = c.clone();
                new_c.inputs.insert(source, signal);

                new_c.value = if new_c.inputs.iter().all(|kv| *kv.1 == true) {
                    false
                } else {
                    true
                };

                (Module::Conj(new_c), true)
            }
            any => (any.clone(), true),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FlipFlop {
    targets: Vec<String>,
    value: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Conj {
    targets: Vec<String>,
    value: bool,
    inputs: HashMap<String, bool>,
}

#[derive(Debug, Clone, PartialEq)]
struct Broadcaster {
    targets: Vec<String>,
}

fn targets_from_str(s: &str) -> (String, Vec<String>) {
    let mut parts = s.split("->");
    (
        parts
            .next()
            .unwrap()
            .trim()
            .trim_start_matches("%")
            .trim_start_matches("&")
            .to_owned(),
        parts
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>(),
    )
}

fn module_from_str(s: &str) -> (String, Module) {
    let (name, targets) = targets_from_str(s);
    match s.chars().nth(0).unwrap() {
        '%' => (
            name,
            Module::FlipFlop(FlipFlop {
                targets: targets,
                value: false,
            }),
        ),
        '&' => (
            name,
            Module::Conj(Conj {
                targets: targets,
                value: false,
                inputs: HashMap::new(),
            }),
        ),
        _ => (name, Module::Broadcaster(Broadcaster { targets: targets })),
    }
}

fn modules_from_str(s: &str) -> HashMap<String, Module> {
    let mut hm = HashMap::new();

    for line in s.lines() {
        let (name, m) = module_from_str(line.trim());
        hm.insert(name, m);
    }

    let mut updated_hm = HashMap::new();
    updated_hm.extend(hm.clone());

    for (key, module) in hm.iter() {
        for target_name in module.targets() {
            if !updated_hm.contains_key(&target_name) {
                continue;
            }

            let target = updated_hm
                .get_mut(&target_name)
                .expect(&format!("key {} must exist", target_name));

            match target {
                Module::Conj(c) => {
                    c.inputs.insert(key.to_string(), false);
                }
                _ => {}
            }
        }
    }

    updated_hm
}

fn press_button_a_thousand_times(modules: HashMap<String, Module>) -> (usize, usize) {
    let mut m = modules.clone();
    let mut low = 0;
    let mut high = 0;

    for i in 0..1000 {
        let (next_m, next_low, next_high) = propagate(
            m,
            "broadcaster".to_string(),
            "broadcaster".to_string(),
            false,
        );
        m = next_m;
        low += next_low + 1;
        high += next_high;
    }

    return (low, high);
}

fn propagate(
    modules: HashMap<String, Module>,
    source: String,
    module_name: String,
    signal: bool,
) -> (HashMap<String, Module>, usize, usize) {
    let mut low = 0;
    let mut high = 0;

    if !modules.contains_key(&module_name) {
        return (modules, low, high);
    }

    let m = modules.get(&module_name).unwrap();
    let (updated, should_continue) = m.accept(source, signal);
    let updated_value = updated.value();
    let mut next_modules = modules.clone();
    next_modules.insert(module_name.clone(), updated);

    if !should_continue {
        return (next_modules, low, high);
    }

    for target in m.targets().iter() {
        if updated_value == true {
            high += 1
        } else {
            low += 1
        }

        let (propagated_modules, prop_low, prop_high) = propagate(
            next_modules.clone(),
            module_name.clone(),
            target.clone(),
            updated_value,
        );

        low += prop_low;
        high += prop_high;
        next_modules.extend(propagated_modules);
    }

    (next_modules, low, high)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modules_from_str() {
        let m = modules_from_str(
            "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a",
        );
        assert_eq!(5, m.len());
        println!("{:?}", m);
    }

    #[test]
    fn test_press_n() {
        let m = modules_from_str(
            "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a",
        );
        let (low, high) = press_button_a_thousand_times(m);
        assert_eq!(4000, high);
        assert_eq!(8000, low);
    }

    #[test]
    fn test_propagate() {
        let m = modules_from_str(
            "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a",
        );
        let (m2, low, high) = propagate(
            m,
            "broadcaster".to_string(),
            "broadcaster".to_string(),
            false,
        );
        assert_eq!(5, m2.len());
        assert_eq!(4, high);
        assert_eq!(7, low);
        println!("{:?}", m2);
    }

    #[test]
    fn test_module_from_str() {
        let (bn, b) = module_from_str("broadcaster -> a, b, c");
        assert_eq!(bn, "broadcaster");
        assert_eq!(
            "Broadcaster(Broadcaster { targets: [\"a\", \"b\", \"c\"] })",
            format!("{:?}", b)
        );

        let (_, a) = module_from_str("%a -> b");
        assert_eq!(
            "FlipFlop(FlipFlop { targets: [\"b\"], value: false })",
            format!("{:?}", a)
        );

        let (_, inv) = module_from_str("&inv -> a");
        assert_eq!(
            "Conj(Conj { targets: [\"a\"], value: false, inputs: {} })",
            format!("{:?}", inv)
        );
    }
}
