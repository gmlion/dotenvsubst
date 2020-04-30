use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let env_file_arg = std::env::args().nth(1);
    let env_file_path = match env_file_arg {
        Some(file) => std::path::PathBuf::from(file),
        None => std::path::PathBuf::from(".env")
    };

    let env = std::fs::read_to_string(env_file_path).expect("Error: cannot read .env file");

    let mut input_buffer = String::new();
    io::stdin().read_to_string(&mut input_buffer)?;

    let replaced = find_and_replace(&env, input_buffer);

    io::stdout().write_all(&replaced.into_bytes())?;
    Ok(())
}

fn get_env(env: &String, key: String) -> Option<String> {
    for line in env.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with(&key) {
            let equal_index = trimmed_line.find('=')?;
            let (_, value) = trimmed_line.split_at(equal_index + '='.len_utf8());
            return Some(value.into());
        }
    }
    
    None
}

fn find_and_replace(env: &String, content: String) -> String {
    let start_index = content.find("${");
    if start_index.is_none() {
        return content;
    } else {
        let start_index = start_index.expect("Unmanaged error") ;
        let (_, continuation) = content.split_at(start_index + "${".len());
        let end_index = continuation.find("}").expect("Error: unmatched ${");
        let (key, _) = continuation.split_at(end_index);
        let value = get_env(env, key.into()).unwrap_or(format!("${{{}}}", key));
        
        let (pre, _) = content.split_at(start_index);
        let (_, post) = content.split_at(start_index + "${".len() + end_index + "}".len());
        return format!("{}{}{}", pre, value, find_and_replace(env, post.into()));
    }
}
