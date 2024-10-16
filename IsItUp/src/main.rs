use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use reqwest::Client;
use colored::{Color, Colorize};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    /*

        Hello code lurker,

        As you will see ahead, this probably isn't
        peak rust code, however it does work for what I
        wanted it to work and therefore this is good code.

    */

    let args: Vec<String> = env::args().collect();

    // use option to handle possible absence of value
    let file_path: &str;

    if args.len() < 2 { // check for at least one argument

        eprintln!("[-] Error: No file path provided.");
        return; // exit if no argument is provided

    } else {

        file_path = &args[1]; // Assign the value if provided
        eprintln!("[+] Provided path: {}", file_path);

    }

    // check if the file exists
    if !Path::new(file_path).exists() {

        eprintln!("[-] Error: File not found.");
        return; // exit if the file is not found
    }

    // open the file and read lines
    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let client = Client::new();

    let mut tasks = vec![];

    for line in reader.lines() {

        match line {

            Ok(subdomain) => {

                let http_url = format!("http://{}", subdomain);
                let https_url = format!("https://{}", subdomain);
                let client = client.clone();

                // create a task for the requests
                tasks.push(async move {

                    check_url(&client, &http_url).await;
                    check_url(&client, &https_url).await;
                });

            }
            Err(e) => eprintln!("[-] Error reading line: {}", e),
        }
    }

    // limit to 5 requests per second
    for (i, task) in tasks.into_iter().enumerate() {

        if i > 0 && i % 5 == 0 {

            sleep(Duration::from_secs(1)).await; // wait for 1 second every 5 requests
            
        }

        tokio::spawn(task); // spawn each task

    }

    // wait for all tasks to finish
    sleep(Duration::from_secs(2)).await; 
}

// prints the url according to what i considered intuitive
async fn check_url(client: &Client, url: &str) {

    match client.get(url).send().await {

        Ok(response) => {

            let code = response.status().as_u16();
            let color = match code {
                200 => Color::Green,
                301 | 302 => Color::Yellow,
                400..=499 => Color::Red,
                500..=599 => Color::Red,
                _ => Color::Black,

            };

            println!(

                "[+] Request to [{}] -> [{}]",
                url.color(Color::Blue),
                code.to_string().color(color)

            );
        }

        Err(_) => {

            println!("[+] Request to [{}] -> [{}]", url.color(Color::Blue), "UNREACHABLE".color(Color::Black));

        }
    }
}
