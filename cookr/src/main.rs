use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut ingredient_map: HashMap<String, usize> = HashMap::new();

    if let Ok(lines) = read_lines("./src/hellofresh.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("Starting: {}", ip);
                let response = reqwest::blocking::get(
                    ip,
                )
                .unwrap()
                .text()
                .unwrap();
            
                let document = scraper::Html::parse_document(&response);
                let title_selector = scraper::Selector::parse("h1.sc-5b343ba0-0").unwrap();
                let titles = document.select(&title_selector).map(|x| x.inner_html());
            
                // Select ingredients from page.
                let ingredient_selector = scraper::Selector::parse("p.sc-5b343ba0-0").unwrap();
                let ingredients = document
                    .select(&ingredient_selector)
                    .map(|x| x.inner_html());
            
                titles
                    .zip(1..100)
                    .for_each(|(item, number)| println!("{}. {}", number, item));
            
                ingredients.zip(1..100).for_each(|(item, number)| {
                    if !item.contains("(")
                        && !item.contains("<")
                        && !(b'0'..=b'9').contains(&item.as_bytes()[0])
                        && !item.contains("½")
                        && !item.contains("You also might be interested in:")
                        && !item.contains("Water")
                    {
                        let count = ingredient_map.entry(item.to_string()).or_insert(0);
                        *count += 1;
                        println!("{}. {}", number, item)
                    }
                });
            }
        }
    }

    for (key, value) in &ingredient_map {
        println!("{} ({})", key, value);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
