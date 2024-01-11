use std::collections::HashMap;

use super::Solver;

pub struct Day19 {}

#[derive(Debug)]
struct WorkflowItem<'a> {
    next_label: &'a str,
    validation_params: Option<(&'a str, char, usize)>,
}

impl WorkflowItem<'_> {
    fn is_valid(&self, rating: &HashMap<&str, usize>) -> bool {
        if self.validation_params.is_none() {
            return true;
        }

        let (label, op, n) = self.validation_params.as_ref().unwrap();
        let x = rating.get(label).unwrap();

        return if op == &'>' { x > &n } else { x < &n };
    }
}

impl<'a> WorkflowItem<'a> {
    fn new(next_label: &'a str, validation_params: Option<(&'a str, char, usize)>) -> Self {
        Self {
            next_label,
            validation_params,
        }
    }

    fn parse(input: &'a str) -> Self {
        let matches: Vec<&str> = input.split(":").collect();

        if matches.len() == 1 {
            return WorkflowItem::new(matches[0], None);
        }

        let next_label = matches[1];
        let op = if matches[0].contains("<") { '<' } else { '>' };
        let matches: Vec<&str> = matches[0].split(op).collect();
        let label = matches[0];
        let n = matches[1].parse().unwrap();

        return WorkflowItem::new(next_label, Some((label, op, n)));
    }
}

impl Solver for Day19 {
    fn p1(&self, input: &str) -> String {
        let matches: Vec<&str> = input.split("\n\n").collect();
        let mut workflows: HashMap<String, Vec<WorkflowItem<'_>>> = HashMap::new();

        matches[0]
            .lines()
            .map(|l| {
                let label: String = l.chars().take_while(|&x| x != '{').collect();
                let workflow: Vec<WorkflowItem> = l
                    .strip_prefix(&(label.clone() + "{"))
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .split(",")
                    .map(|cond| WorkflowItem::parse(cond))
                    .collect();

                return (label, workflow);
            })
            .for_each(|(k, v)| {
                workflows.insert(k, v);
            });

        let ratings: Vec<HashMap<&str, usize>> = matches[1]
            .lines()
            .map(|l| {
                let mut rating = HashMap::new();
                l.strip_prefix("{")
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .split(",")
                    .map(|assignment| {
                        let matches: Vec<&str> = assignment.split("=").collect();
                        return (matches[0], matches[1].parse().unwrap());
                    })
                    .for_each(|(k, v)| {
                        rating.insert(k, v);
                    });

                return rating;
            })
            .collect();

        fn solve(
            rating: &HashMap<&str, usize>,
            workflows: &HashMap<String, Vec<WorkflowItem>>,
            label: &str,
        ) -> bool {
            let next = workflows
                .get(label)
                .unwrap()
                .iter()
                .find_map(|item| item.is_valid(rating).then(|| item.next_label))
                .unwrap();

            if next == "R" {
                return false;
            }

            if next == "A" {
                return true;
            }

            return solve(rating, workflows, next);
        }

        let res: usize = ratings
            .iter()
            .filter(|rating| solve(&rating, &workflows, "in"))
            .map(|rating| rating.values().sum::<usize>())
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let matches: Vec<&str> = input.split("\n\n").collect();
        let mut workflows: HashMap<String, Vec<WorkflowItem<'_>>> = HashMap::new();

        matches[0]
            .lines()
            .map(|l| {
                let label: String = l.chars().take_while(|&x| x != '{').collect();
                let workflow: Vec<WorkflowItem> = l
                    .strip_prefix(&(label.clone() + "{"))
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .split(",")
                    .map(|cond| WorkflowItem::parse(cond))
                    .collect();

                return (label, workflow);
            })
            .for_each(|(k, v)| {
                workflows.insert(k, v);
            });

        fn solve(
            workflows: &HashMap<String, Vec<WorkflowItem>>,
            vars: &mut HashMap<String, (usize, usize)>,
            label: &str,
        ) -> usize {
            if label == "R" {
                return 0;
            }

            if label == "A" {
                return vars
                    .iter()
                    .filter(|&(k, _)| "xmas".contains(k))
                    .map(|(_, v)| v.1 - v.0 + 1)
                    .product();
            }

            let workflow = workflows.get(label).unwrap();

            let mut res = 0;
            for item in workflow {
                let next_label = item.next_label;

                if item.validation_params.is_none() {
                    res += solve(workflows, &mut vars.clone(), next_label);
                    continue;
                }

                let (lhs, op, rhs) = item.validation_params.unwrap();
                let x = vars.get(lhs).unwrap();

                if rhs < x.0 || rhs > x.1 {
                    continue;
                }

                let mut left = x.clone();
                let mut right = x.clone();

                if op == '>' {
                    left.0 = (rhs + 1).max(left.0);
                    right.1 = (rhs).min(right.1);
                } else {
                    left.1 = (rhs - 1).min(left.1);
                    right.0 = (rhs).max(right.0);
                }

                let mut next_vars = vars.clone();
                next_vars.insert(lhs.to_string(), left);
                res += solve(workflows, &mut next_vars, next_label);

                vars.insert(lhs.to_string(), right);
            }

            return res;
        }

        let res = solve(
            &workflows,
            &mut HashMap::from([
                ("x".to_string(), (1, 4000)),
                ("m".to_string(), (1, 4000)),
                ("a".to_string(), (1, 4000)),
                ("s".to_string(), (1, 4000)),
            ]),
            "in",
        );

        return res.to_string();
    }
}
