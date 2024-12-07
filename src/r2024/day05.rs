// REQUIRED
use crate::utils::AOCResult;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    usize,
};

// OPTIONAL
use regex::Regex;

fn reader_to_groups<R: BufRead>(reader: R) -> (Vec<Regex>, Vec<String>) {
    let mut reg_vec: Vec<Regex> = Vec::new();
    let mut str_vec: Vec<String> = Vec::new();

    let mut str_flag = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            str_flag = true;
            continue;
        }
        // str_flag = line.is_empty();

        if str_flag {
            str_vec.push(line.to_string());
        } else {
            let (num1, num2) = line.split_once("|").unwrap();

            let rstr = format!(
                r"(?<good>({}).*({}))|(?<notgood>({}).*({}))",
                num1, num2, num2, num1
            );
            reg_vec.push(Regex::new(&rstr).unwrap());
        }
    }

    (reg_vec, str_vec)
}

fn reader_to_untyped<R: BufRead>(reader: R) -> (Vec<String>, Vec<String>) {
    let mut vec1: Vec<String> = Vec::new();
    let mut vec2: Vec<String> = Vec::new();

    let mut switch = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            switch = true;
            continue;
        }
        // str_flag = line.is_empty();

        if switch {
            vec2.push(line.to_string());
        } else {
            vec1.push(line.to_string());
        }
    }

    (vec1, vec2)
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let (reg_vec, str_vec) = reader_to_groups(reader);

    // Apply to instructions
    let valid_sum = str_vec.iter().fold(0, |runtot, row| {
        // Check if row passes regex
        let valid = reg_vec.iter().fold(true, |curr_flag, instr| {
            let caps = instr.captures(&row);
            let valid_instr = caps.map_or(true, |filters| filters.get(1).is_some());
            curr_flag && valid_instr
        });

        // Add middle index
        let newsum = {
            let nums = row.split(",").collect::<Vec<&str>>();
            nums[nums.len() / 2].parse::<usize>().unwrap() * valid as usize
        };

        runtot + newsum
    });

    Ok(valid_sum)
}

fn visit(
    word: String,
    dag: HashMap<&str, HashSet<&str>>,
    row: HashSet<&str>,
    mut sorted: Vec<String>,
    mut perm: HashSet<String>,
    mut temps: HashSet<String>,
) -> (Vec<String>, HashSet<String>, HashSet<String>) {
    // already handled
    if perm.contains(&word) {
        return (sorted, perm, temps);
    }

    // extra words in instructions not in row
    if !row.contains(&word.as_str()) {
        return (sorted, perm, temps);
    }

    // check if in cycle
    if temps.contains(&word) {
        panic!("NOT A DAG!");
    }

    // visit each node that is after this
    let def: HashSet<&str> = HashSet::new();
    temps.insert(word.clone());
    let set = dag.get(&word.as_str()).unwrap_or(&def);
    set.iter().for_each(|&edge| {
        (sorted, perm, temps) = visit(
            edge.to_string(),
            dag.clone(),
            row.clone(),
            sorted.clone(),
            perm.clone(),
            temps.clone(),
        );
    });

    // remove temp tag; add perm tag
    temps.remove(&word);
    perm.insert(word.clone());

    // prepend to maintain sorting direction
    let new = vec![word];
    let s2 = new
        .iter()
        .chain(sorted.iter())
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    (s2, perm, temps)
}

fn sort_via_dag(dag: HashMap<&str, HashSet<&str>>, row: &str) -> Vec<String> {
    let mut perm: HashSet<String> = HashSet::new();
    let mut temps: HashSet<String> = HashSet::new();
    let mut sorted: Vec<String> = Vec::new();

    let words = row.split(",").map(|word| word).collect::<HashSet<&str>>();

    words.iter().for_each(|word| {
        (sorted, perm, temps) = visit(
            word.to_string(),
            dag.clone(),
            words.clone(),
            sorted.clone(),
            perm.clone(),
            temps.clone(),
        );
    });

    sorted
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    // Split input into instructions and tests
    let (instr, rows) = reader_to_untyped(reader);

    // Convert instr to directed graph
    let mut dag: HashMap<&str, HashSet<&str>> = HashMap::new();
    instr.iter().for_each(|cmd| {
        let (num1, num2) = cmd.split_once("|").unwrap();
        dag.entry(num1).or_insert_with(HashSet::new).insert(num2);
    });

    // topological sort
    let pagesum = rows.iter().fold(0, |runtot, row| {
        let sorted_row = sort_via_dag(dag.clone(), row);
        let sorted_join = sorted_row.join(",");

        runtot
            + sorted_row[sorted_row.len() / 2]
                .as_str()
                .parse::<usize>()
                .unwrap()
                * ((sorted_join != *row) as usize)
    });

    Ok(pagesum)
}
