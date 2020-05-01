use std::io::{self, Read, Write};

#[derive(Debug)]
enum Error {
    InputError(String),
    OutputError(String)
}


fn main() -> Result<(), Error> {
    let env_file_arg = std::env::args().nth(1);
    let env_file_path = match env_file_arg {
        Some(file) => String::from(file),
        None => String::from(".env")
    };

    let env = std::fs::read_to_string(&env_file_path)
        .map_err(|err| Error::InputError(format!("Cannot read file `{}`: {}", env_file_path, err)))?;

    let mut input_buffer = String::new();
    io::stdin().read_to_string(&mut input_buffer)
        .map_err(|err| Error::InputError(format!("Cannot read standard input: {}", err)))?;

    let replaced = find_and_replace(&env, input_buffer);

    io::stdout().write_all(&replaced.into_bytes())
        .map_err(|err| Error::OutputError(format!("Cannot write to standard output: {}", err)))?;
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

pub fn find_and_replace(env: &String, content: String) -> String {
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

#[cfg(test)]
mod tests {
    const ENV: &str = "
    TEST1=aaa
    TEST2=bbb
    TEST3=${should_be_kept}
    #Comment
    Should be ignored as well
    TEST4=ccc
    ";

    #[test]
    fn replace() {
        let input = "
        Lorem ipsum ${TEST1} lorem $NOACTION ${MISSING} text ${TEST2}
        ${TEST3}
        ${TEST4}
        ";
        let replaced = super::find_and_replace(&String::from(ENV), input.into());
        print!("{}", replaced);
        assert_eq!("
        Lorem ipsum aaa lorem $NOACTION ${MISSING} text bbb
        ${should_be_kept}
        ccc
        ", replaced);
    }

}