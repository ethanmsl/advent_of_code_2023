use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
        // let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
        let re_id = Regex::new(r"Game (\d*):").unwrap();
        let re_r = Regex::new(r"(?<red>\d*) blue").unwrap();
        let re_g = Regex::new(r"(\d*) green").unwrap();
        let re_b = Regex::new(r"(\d*) red").unwrap();
        let re_rgb = Regex::new(r"(?<red>\d*) red|(?<green>\d*) green|(?<blue>\d*) blue").unwrap();

        let hay = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 202: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        hay.lines().for_each(|line| {
                println!("\n\n-----{}-----", line);
                for c in re_r.captures_iter(line) {
                        dbg!(&c["red"]);
                }
        });

        let mut ids = vec![];
        for (_, [id]) in re_id.captures_iter(hay).map(|c| c.extract()) {
                ids.push(id.parse::<u64>()?);
        }
        dbg!(ids);

        // let mut reds = vec![];
        // for (full, [capture]) in re_r.captures_iter(hay).map(|c| c.extract()) {
        //         reds.push((full.to_string(), capture));
        // }
        //
        // let mut results = vec![];
        // for (full, [capture]) in re_rgb.captures_iter(hay).map(|c| c.extract()) {
        //         results.push((full.to_string(), capture));
        // }
        // dbg!(reds);
        //
        // dbg!(results);
        Ok(())
}
