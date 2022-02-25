use requests;
use regex::Regex;
use soup::{Soup, QueryBuilderExt, NodeExt};
fn main() {
    let res = requests::get("https://thebulletin.org/doomsday-clock/past-statements/")
        .expect("Failed to fetch website");
    let content=res.text().unwrap();
    
    let content=Soup::new(content);
    let h3=content.tag("h3").find().expect("Error tage <h3> not found!");
    let match_time=Regex::new(r#"IT IS(?: STILL)? (?P<time>\d+)(?: AND A (?P<half>HALF))? (?P<type>MINUTES|SECONDS) TO MIDNIGHT"#).unwrap();
    let match_sentence=h3.text();
    let res=match_time.captures(&match_sentence).unwrap();
    println!("{} {} to the End!", &res["time"],&res["type"]);
}
