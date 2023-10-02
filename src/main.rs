use rand::seq::SliceRandom;
use ureq::Error;

const REMOTE_FILE: &str =
    "https://raw.githubusercontent.com/ngerakines/commitment/master/commit_messages.txt";

fn load_commit_file() -> Result<String, Error> {
    // download file
    let response = ureq::get(REMOTE_FILE).call()?;
    let content = response.into_string()?;
    // TODO: save file to temp dir:
    // const TEMP_FILE: &str = "/tmp/commit_messages.txt";
    // return content
    Ok(content)
}

fn main() {
    let content = load_commit_file().unwrap();
    let lines: Vec<&str> = content.split('\n').collect();
    let random_line = lines.choose(&mut rand::thread_rng()).unwrap();

    println!("{}", random_line);
}
