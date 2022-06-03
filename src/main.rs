use ureq::Error;

fn main() {
    match ureq::get("http://whatthecommit.com/index.txt").call() {
        Ok(response) => {
            println!("{}", response.into_string().unwrap().trim());
        }
        Err(Error::Status(code, _)) => {
            println!("ERROR: HTTP {}", code);
            std::process::exit(exitcode::DATAERR);
        }
        Err(_) => {
            /* some kind of io/transport error */
            println!("IO transport error!");
            std::process::exit(exitcode::DATAERR);
        }
    }
}
