use rand::Rng;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
#[serde(untagged)]
enum YamlValue {
    String(String),
    Integer(i32),
    List(Vec<YamlValue>),
    Map(std::collections::HashMap<String, YamlValue>),
}

// Config
struct Cli {
    depth: usize,
    files: usize
}

impl Cli {
    fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut depth: usize = 3;
        let mut files: usize = 100;
        if args.len() > 1 {
            depth = args[1].parse().expect("Depth must be a number");
        }
        if args.len() > 2 {
            files = args[2].parse().expect("Files must be a number");
        }
        Cli { depth, files }
    }
}

fn generate_random_yaml(depth: usize, max_depth: usize) -> YamlValue {
    let mut rng = rand::thread_rng();
    if depth >= max_depth {
        match rng.gen_range(0..2) {
            0 => YamlValue::String("random_string".to_string()),
            1 => YamlValue::Integer(rng.gen_range(0..100)),
            _ => YamlValue::String("another_random_string".to_string()),
        }
    } else {
        if rng.gen_bool(0.6) {  // Increase probability for lists
            let list_size = rng.gen_range(1..5);
            let mut list = Vec::new();
            for _ in 0..list_size {
                list.push(generate_random_yaml(depth + 1, max_depth));
            }
            YamlValue::List(list)
        } else {
            let map_size = rng.gen_range(1..5);
            let mut map = std::collections::HashMap::new();
            for i in 0..map_size {
                map.insert(format!("key_{}", i), generate_random_yaml(depth + 1, max_depth));
            }
            YamlValue::Map(map)
        }
    }
}

fn write_yaml_to_file(path: &Path, depth: usize) {
    let mut rng = rand::thread_rng();
    let mut docs = Vec::new();

    // Determine the number of YAML documents to generate (1 to 3)
    let doc_count = rng.gen_range(1..=3);

    for _ in 0..doc_count {
        let yaml = generate_random_yaml(0, depth);
        docs.push(yaml);
    }

    // Serialize multiple YAML documents
    let yaml_string = docs
        .iter()
        .map(|doc| serde_yaml::to_string(doc).unwrap())
        .collect::<Vec<String>>()
        .join("---\n");

    fs::write(path, yaml_string).expect("Unable to write file");
}

fn create_random_subdirectories(base_dir: &Path, depth: usize, file_count: usize) {
    let mut rng = rand::thread_rng();
    if depth == 0 {
        for i in 0..file_count {
            let file_path = base_dir.join(format!("file_{}.yaml", i));
            write_yaml_to_file(&file_path, rng.gen_range(1..4));
        }
    } else {
        let subdir_count = rng.gen_range(1..4);
        for i in 0..subdir_count {
            let subdir_path = base_dir.join(format!("subdir_{}", i));
            fs::create_dir_all(&subdir_path).expect("Unable to create directory");
            create_random_subdirectories(&subdir_path, depth - 1, file_count);
        }
    }
}
fn main() {
    let base_dir = Path::new("output");
    let cli = Cli::parse();

    
    fs::create_dir_all(base_dir).expect("Unable to create base directory");
    create_random_subdirectories(base_dir, cli.depth, cli.files );
}
