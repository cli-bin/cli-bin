use anyhow::{self, Context};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::{fs::File, path::PathBuf, process::Command};
use structopt::{self, StructOpt};

fn main() -> anyhow::Result<()> {
    let opts = CliOpts::from_args();

    let data_file = File::open(&opts.data).context(format!("Reading {}", opts.data.display()))?;

    let env_maps: Vec<EnvMap> = serde_json::from_reader(&data_file)
        .context(format!("Parsing {} as JSON", opts.data.display()))?;

    for env_map in env_maps {
        Command::new(&opts.script)
            .args(&opts.args)
            .env_clear()
            .envs(env_map.vars())
            .spawn()?;
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct CliOpts {
    data: PathBuf,
    script: PathBuf,
    args: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EnvMap {
    #[serde(flatten)]
    map: HashMap<String, String>,
}

impl EnvMap {
    fn vars(&self) -> impl Iterator<Item = (&str, &str)> {
        self.map.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}
