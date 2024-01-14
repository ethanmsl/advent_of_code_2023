use anyhow::Result;

/// Example helpfully provided by the `once_cell` crate's documentation.
///
/// Simply assigns a string literal pattern to a `Regex` generator
/// wrapped in a lazy `once_cell` wrapper.
/// Besides avoiding perf issues with accidental-recompilation
/// I just find using a `static` for these patterns clear and readable for this code.
///
/// # PERF: `regex-automata` crate allows creating a DFA, writing it, and rolling it into binary.
///  That woudl avoid any initialization cost when running binary, and allow speed optimization
///  for what are very simple regexes.
macro_rules! regex_lazyonce {
        ($re:expr $(,)?) => {{
                static RE: once_cell::sync::OnceCell<regex::Regex> =
                        once_cell::sync::OnceCell::new();
                RE.get_or_init(|| regex::Regex::new($re).unwrap())
        }};
}

const ID_PAT: &str = r"Game (\d+):";
const RED_PAT: &str = r"(\d+) red";
const GREEN_PAT: &str = r"(\d+) green";
const BLUE_PAT: &str = r"(\d+) blue";

/// REFACTOR:
/// yuuuch at repeated code.
/// Not sure of  a better way to do this if usign regex_macro
/// ... I should just have defined the regex as static with lazy
/// and then iterated over them.
fn extract_data(hay: &str) -> Vec<u32> {
        let id = regex_lazyonce!(ID_PAT).captures(hay)
                                        .expect("captures iter failure")
                                        .get(1)
                                        .map(|v| {
                                                v.as_str()
                                                 .parse::<u32>()
                                                 .expect("id parse failure")
                                        })
                                        .expect("iteration failure");
        let r_sum = regex_lazyonce!(RED_PAT).captures_iter(hay)
                                            .map(|c| {
                                                    let (_, [val]) = c.extract();
                                                    val.parse::<u32>()
                                                       .expect("red parse failure")
                                            })
                                            .sum();
        let g_sum = regex_lazyonce!(GREEN_PAT).captures_iter(hay)
                                              .map(|c| {
                                                      let (_, [val]) = c.extract();
                                                      val.parse::<u32>()
                                                         .expect("green parse failure")
                                              })
                                              .sum();
        let b_sum = regex_lazyonce!(BLUE_PAT).captures_iter(hay)
                                             .map(|c| {
                                                     let (_, [val]) = c.extract();
                                                     val.parse::<u32>()
                                                        .expect("blue parse failure")
                                             })
                                             .sum();
        vec![id, r_sum, g_sum, b_sum]
}

fn main() -> Result<()> {
        let hay = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 202: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
        ";

        for line in hay.lines() {
                let data = extract_data(line);
                println!("{:?}", data);
        }
        Ok(())
}
