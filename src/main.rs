#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod problem;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

// Ref: https://github.com/aylei/leetcode-rust/blob/master/src/main.rs
fn main() {
    println!("Welcome to leetcode-rust system.");
    let mut solved_ids = get_solved_ids();
    loop {
        println!("Please enter a frontend problem id, or \"random\" to generate a random one.");
        let mut is_random = false;
        let mut id: u32 = 0;
        let mut id_arg = String::new();
        io::stdin()
            .read_line(&mut id_arg)
            .expect("Failed to read line");
        let id_arg = id_arg.trim();
        match id_arg {
            "random" => {
                println!("You select random mode.");
                id = generate_random_id(&solved_ids);
                is_random = true;
                println!("Generate random problem: {}", &id);
            }
            _ => {
                id = id_arg
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("not a number: {}", id_arg));
                if solved_ids.contains(&id) {
                    println!(
                        "The problem you chose is invalid (the problem may have been solved \
                         or may have no rust version)."
                    );
                    continue;
                }
            }
        }

        let problem = problem::get_problem(id).unwrap_or_else(|| {
            panic!(
                "Error: failed to get problem #{} \
                 (The problem may be paid-only or may not be exist).",
                id
            )
        });
        let code = problem.code_definition.iter().find(|&d| d.value == "rust");
        if code.is_none() {
            println!("Problem {} has no rust version.", &id);
            solved_ids.push(problem.question_id);
            continue;
        }
        let code = code.unwrap();

        let file_name = format!(
            "n{:04}_{}",
            problem.question_id,
            problem.title_slug.replace("-", "_")
        );
        let file_path = Path::new("./src").join(format!("{}.rs", file_name));
        if file_path.exists() {
            panic!("problem already initialized");
        }

        let template = fs::read_to_string("./template.rs").unwrap();
        let (extra_use, test_extra_use) = parse_extra_use(&code.default_code);
        let source = template
            .replace("// __PROBLEM_DEFAULT_CODE__", &code.default_code)
            .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
            .replace("// __EXTRA_USE__\n", &extra_use)
            .replace("// __TEST_EXTRA_USE__\n", &test_extra_use);

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .unwrap();

        file.write_all(source.as_bytes()).unwrap();
        drop(file);

        let mut lib_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/lib.rs")
            .unwrap();
        writeln!(lib_file, "mod {};", file_name);
        break;
    }
}

fn generate_random_id(except_ids: &[u32]) -> u32 {
    use rand::Rng;
    use std::fs;
    let mut rng = rand::thread_rng();
    loop {
        let res: u32 = rng.gen_range(1, 1106);
        if !except_ids.contains(&res) {
            return res;
        }
        println!(
            "Generate a random num ({}), but it is invalid (the problem may have been solved \
             or may have no rust version). Regenerate..",
            res
        );
    }
}

fn get_solved_ids() -> Vec<u32> {
    let paths = fs::read_dir("./src").unwrap();
    let mut solved_ids = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let s = path.to_str().unwrap();
        if !s.starts_with('n') {
            continue;
        }
        let id = &s[7..11];
        let id = id.parse::<u32>().unwrap();
        solved_ids.push(id);
    }
    solved_ids
}

fn parse_extra_use(code: &str) -> (String, String) {
    let (mut extra_use_line, mut test_extra_use_line) = (String::new(), String::new());
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("use super::utils::linked_list::ListNode;\n");
        test_extra_use_line.push_str("use super::utils::linked_list::to_list;\n");
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("use super::utils::tree::TreeNode;\n");
        test_extra_use_line.push_str("use super::utils::tree::to_tree;\n");
    }
    if !extra_use_line.is_empty() {
        extra_use_line.push('\n');
    }
    (extra_use_line, test_extra_use_line)
}
