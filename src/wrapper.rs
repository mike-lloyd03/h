use anyhow::Result;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Wrapper {
    pub cmd: String,
    pub replacement: String,
    #[serde(default)]
    pub use_pager: bool,
}

impl Wrapper {
    pub fn matches_args(self, args: &[String]) -> Result<bool> {
        let re = Regex::new(&self.cmd)?;
        let search_string = args.join(" ");

        Ok(re.is_match(&search_string))
    }

    pub fn parse_replacement(self, args: &[String]) -> Result<Vec<String>> {
        let re = Regex::new(&self.cmd)?;
        let search_string = args.join(" ");
        let caps = re.captures(&search_string);

        let mut result = self.replacement;

        if let Some(caps) = caps {
            for (i, c) in caps.iter().enumerate() {
                // First capture is the whole string
                if i == 0 {
                    continue;
                }

                if let Some(capture) = c {
                    result = result.replace(&format!("{{{i}}}"), capture.as_str());
                }
            }
        }

        Ok(result.split_whitespace().map(String::from).collect())
    }
}

#[test]
fn test_parse_replacement() -> Result<()> {
    let w = Wrapper {
        cmd: "cargo (.*)".into(),
        replacement: "cargo help {1}".into(),
        use_pager: true,
    };

    let args: Vec<String> = vec!["cargo", "run"].into_iter().map(String::from).collect();
    let expect: Vec<String> = vec!["cargo", "help", "run"]
        .into_iter()
        .map(String::from)
        .collect();

    assert_eq!(expect, w.parse_replacement(&args)?);

    Ok(())
}
