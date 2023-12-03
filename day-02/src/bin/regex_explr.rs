use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
        let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
        let hay = "\
path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
";

        let mut results = vec![];
        for (_, [path, lineno, line]) in re.captures_iter(hay).map(|c| c.extract()) {
                results.push((path, lineno.parse::<u64>()?, line));
        }
        dbg!(results);
        Ok(())
}
