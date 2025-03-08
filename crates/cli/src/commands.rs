use {
    clap::{Parser, Subcommand, ValueEnum},
    std::{fmt, str::FromStr},
};

#[derive(Parser)]
#[command(name = "IDL Parser CLI")]
#[command(about = "Generate Program Parsers for Indexer.")]
#[command(version = "0.1.4", author = "aimbot-labs")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "parse")]
    #[command(about = "Generate code for Indexer around the submitted IDL.")]
    #[command(aliases = &["create", "generate"])]
    Parse(ParseOptions),
}

#[derive(Parser)]
pub struct ParseOptions {
    #[arg(short, long, required = true)]
    #[arg(help = "Path to an IDL json file or a Solana program address.")]
    pub idl: IdlSource,

    #[arg(short, long, required = true)]
    #[arg(help = "Path to the desired output directory.")]
    pub output: String,

    #[arg(short = 'c', long = "as-crate", default_value_t = false)]
    #[arg(help = "Generate a directory or a crate.")]
    pub as_crate: bool,

    #[arg(short, long = "standard", default_value = "anchor")]
    #[arg(help = "Specify the IDL standard to parse.")]
    pub standard: IdlStandard,

    #[arg(short, long)]
    #[arg(help = "Comma-separated names of defined types to parse as CPI Events.")]
    pub event_hints: Option<String>,

    #[arg(short, long, required_if_eq("idl", "ProgramAddress"))]
    #[arg(help = "Network URL to fetch the IDL from. Required if input is a program address.")]
    pub url: Option<Url>,
}

#[derive(Clone, Debug)]
pub enum IdlSource {
    FilePath(String),
    ProgramAddress(String),
}

impl std::str::FromStr for IdlSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with(".json") {
            Ok(IdlSource::FilePath(s.to_string()))
        } else if s.len() <= 44 && s.len() >= 32 && s.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(IdlSource::ProgramAddress(s.to_string()))
        } else {
            Err("Invalid input: Must be either a valid file path (ending in .json) or a valid Solana program address.".to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub enum Url {
    Mainnet,
    Devnet,
    CustomRpc(String),
}

impl FromStr for Url {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet-beta" => Ok(Url::Mainnet),
            "devnet" => Ok(Url::Devnet),
            _ if s.starts_with("http") => Ok(Url::CustomRpc(s.to_string())),
            _ => {
                Err("Invalid network: Must be 'mainnet', 'devnet', or a valid RPC URL.".to_string())
            }
        }
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Url::Mainnet => write!(f, "mainnet-beta"),
            Url::Devnet => write!(f, "devnet"),
            Url::CustomRpc(ref url) => write!(f, "{url}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum IdlStandard {
    Anchor,
    Codama,
}

impl fmt::Display for IdlStandard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IdlStandard::Anchor => write!(f, "anchor"),
            IdlStandard::Codama => write!(f, "codama"),
        }
    }
}

impl std::str::FromStr for IdlStandard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "anchor" => Ok(IdlStandard::Anchor),
            "codama" => Ok(IdlStandard::Codama),
            _ => Err("Invalid Idl Standard: Must be 'anchor' or 'codama'.".to_string()),
        }
    }
}
