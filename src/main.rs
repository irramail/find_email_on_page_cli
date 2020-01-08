extern crate reqwest;

use std::io::Read;
use std::fs;

fn main() {
    use linkify::{LinkFinder, LinkKind};
    let need = String::from("page_with_email.txt");

    let contents = fs::read_to_string(need)
        .expect("Something went wrong reading the file");

    let mut all_emails = String::new();
    all_emails.push('|');

    for line in contents.lines() {

        let res = reqwest::get(line);

        match res {
            Ok(mut a) => {
                    let mut body = String::new();
                    a.read_to_string(&mut body).unwrap();

                    let mut finder = LinkFinder::new();
                    finder.kinds(&[LinkKind::Email]);
                    let links: Vec<_> = finder.links(body.as_str()).collect();

                    for email in links {
                        let tmp = format!("|{}|", email.as_str().to_ascii_lowercase());
                        if !all_emails.contains(tmp.as_str())
                            && tmp.as_str()[1..].chars().next().unwrap().is_ascii_alphanumeric()
                            && !tmp.as_str().contains('/')
                        {
                            all_emails.push_str(&tmp.as_str()[1..]);
                            println!("{}", email.as_str());
                        }
                    }
                },
            Err(b) => (),
        }
    }
}
