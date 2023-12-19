use std::collections::{BTreeMap};

fn main() {
    part1();
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    destination: String // if A, then it's accepted, else if R, then it's rejected otherwise it's a workflow
}

#[derive(Debug)]
struct Rule {
    category: String,
    check: String,
    number: i32,
    destination: String // if A, then it's accepted, else if R, then it's rejected otherwise it's a workflow
}

fn part1() {
    let input = include_str!("input.txt");

    let parts: Vec<&str> = input.splitn(2, "\r\n\r\n").collect();
    let workflows_strings: Vec<&str> = parts[0].split("\r\n").collect();
    let parts_strings: Vec<&str> = parts[1].split("\r\n").collect();

    println!("workflows: {:?}", workflows_strings);
    println!("parts: {:?}", parts_strings);

    // all parts begin through the workflow named in

    let mut workflows: BTreeMap<String, Workflow> = BTreeMap::new();
    for workflow_string in workflows_strings {
        // px{a<2006:qkq,m>2090:A,rfg}
        let id = workflow_string.splitn(2, "{").collect::<Vec<&str>>()[0];
        let mut rules_string = workflow_string
            .splitn(2, "{").collect::<Vec<&str>>()[1]
            .splitn(2, "}").collect::<Vec<&str>>()[0]
            .split(",").collect::<Vec<&str>>();
        let destination = rules_string.last().unwrap().to_string();
        rules_string.pop();

        let mut rules: Vec<Rule> = Vec::new();
        for rule_string in rules_string {
            // find < or >
            let check_index = rule_string.find('>').or_else(|| rule_string.find('<')).expect("no < or > found");
            let check = rule_string.chars().nth(check_index).unwrap().to_string();
            let category = rule_string[0..check_index].to_string();

            let colon_index = rule_string.find(":").unwrap();
            let number = rule_string[check_index+1..colon_index].parse::<i32>().unwrap();
            let destination = rule_string[colon_index+1..].to_string();

            rules.push(Rule {
                category,
                check,
                number,
                destination
            });
        }

        workflows.insert(id.to_string(), Workflow {
            rules,
            destination
        });
    }

    // parse the parts
    let mut parts: Vec<BTreeMap<String, i32>> = Vec::new();
    for part_string in parts_strings {
        let mut part: BTreeMap<String, i32> = BTreeMap::new();
        // {x=787,m=2655,a=1222,s=2876}
        let part_string = part_string
            .splitn(2, "{").collect::<Vec<&str>>()[1]
            .splitn(2, "}").collect::<Vec<&str>>()[0];
        let part_strings: Vec<&str> = part_string.split(",").collect();
        for part_string in part_strings {
            let key = part_string.split("=").collect::<Vec<&str>>()[0].to_string();
            let value = part_string.split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

            part.insert(key, value);
        }

        parts.push(part);
    }

    // the first workflow is always "in"
    let mut sum = 0;
    for part in parts {
        let mut current_workflow = "in";

        while current_workflow != "A" && current_workflow != "R" {
            let mut is_moved = false;
            let workflow = workflows.get(current_workflow).expect("no workflow found");
            println!("part: {:?} -> {}", part, current_workflow);
            for rule in &workflow.rules {
                let part_id = &rule.category;
                if part.contains_key(part_id) {
                    if rule.check == "<" {
                        println!("{}: {} < {}", part_id, part.get(part_id).unwrap(), rule.number);
                        if part.get(part_id).unwrap() < &rule.number {
                            current_workflow = &rule.destination;
                            is_moved = true;
                            break;
                        }
                    } else if rule.check == ">" {
                        println!("{}: {} > {}", part_id, part.get(part_id).unwrap(), rule.number);
                        if part.get(part_id).unwrap() > &rule.number {
                            current_workflow = &rule.destination;
                            is_moved = true;
                            break;
                        }
                    }
                }
            }

            if !is_moved {
                println!(" -> {}", workflow.destination);
                current_workflow = &workflow.destination;
            }
        }

        println!();

        if current_workflow == "A" {
            println!("approved");
            sum += part.get("x").unwrap();
            sum += part.get("m").unwrap();
            sum += part.get("a").unwrap();
            sum += part.get("s").unwrap();
        } else {
            println!("rejected");
        }
    }

    println!("Part 1 - sum: {}", sum);
}

