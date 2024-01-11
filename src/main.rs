use clap::Parser;
use std::env;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::Write;

use crate::civitai_api::{get_model_file_url, download_civitai_model_by_id, get_first_query_item, QueryItem};

mod civitai_api;
mod format;
mod format_test;

const DEFAULT_COUNT: u8 = 5;
const SHORT_SIZE: usize = 100;
const DESC_CUTOFF: &str = "...";
const TAB_SPACE: &str = "    ";
const REPORT_FORMAT: &str = ".txt";
const ENV_MODEL_DIR: &str = "MODEL_DIRECTORY";
const ERR_FILE_WRITE: &str = "Vorpal: Something went wrong when writing to the file.\n";
const ERR_COUNT_TOO_BIG: &str = "Vorpal: Maximum query count allowed by API is 100";
const ERR_MUTUALLY_EXCLUSIVE: &str = "Vorpal: These arguments are mutually exclusive. The -m argument is meant for only downloading metadata, and the -o argument is for only downloading models.";
const MSG_DRY_RUN: &str = "Vorpal: Performing dry run (no download)";
const MSG_DOWNLOAD_START: &str = "Vorpal: Starting download...";
const MSG_DOWNLOAD_SUCCESS: &str = "Vorpal: Download successful! Enjoy your model!";
const MSG_DOWNLOAD_FAIL: &str = "Vorpal: Download failed";
const STDIN_FAILED: &str = "Failed to get input";
const STDIN_INVALID: &str = "Vorpal: Only input integers";
const STDIN_OUT_OF_RANGE: &str = "Vorpal: The number you entered is not in the query";
const STDIN_GETTING: &str = "Getting item: ";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// The name of the model to download. First result will be downloaded.
    model_name: Option<String>,

    /// Run in interactive mode (Search models, then select one to download).
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Specify a directory to download to. Overrides MODEL_DIRECTORY environment variable.
    /// Currnet directory will be used if both are empty.
    #[arg(short, long, value_name = "DIRECTORY")]
    directory: Option<PathBuf>,

    /// Only download model (don't save metadata).
    #[arg(short, long, default_value_t = false)]
    only_model: bool,

    /// Only get metadata of model.
    #[arg(short, long, value_name = "MODEL_NAME")]
    meta: bool,

    /// Search Civitai for available models and LoRAs.
    #[arg(short, long, value_name = "QUERY")]
    query: Option<String>,

    /// How many models to search 
    #[arg(short, long, default_value_t = DEFAULT_COUNT, value_name = "COUNT", value_parser=format::check_limit)]
    count: u8,
    
    /// Enter query as 'safe' (no NSFW).
    #[arg(short, long, default_value_t = false, value_name = "QUERY")]
    safe: bool,

    /// Show full descriptions of query.
    #[arg(short, long, default_value_t = false)]
    full: bool,

    /// Return the download url of a model only.
    #[arg(short, long, value_name = "MODEL_NAME")]
    url: Option<String>,

}

impl QueryItem {
    /// Method for CLI-specific output of queries
    fn make_cli_query_display(&self, full: bool) -> String {
        let mut display_vec: Vec<String> = Vec::new();
        display_vec.push(format!("{}Model: {}", TAB_SPACE, self.get_model_filename()));
        display_vec.push(format!("{}Id: {}", TAB_SPACE, self.get_id()));
        display_vec.push(format!("{}Creator: {}", TAB_SPACE, self.get_creator_name()));
        display_vec.push(format!("{}Tags: {}", TAB_SPACE, self.get_tags()));
        match full {
            true => display_vec.push(format!("{}{}", TAB_SPACE, self.get_description())),
            false => display_vec.push(format!("{}{}", TAB_SPACE, self.get_short_description(SHORT_SIZE, DESC_CUTOFF))),
        }
        display_vec.push("\n".to_string());
        display_vec.join("\n")
    }
}

fn print_query(mut query: Vec<QueryItem>, full: bool) -> () {
    query.reverse();
    dbg!{&query};
    let output = concatenate_query_items(query, full);
    println!("{}", output);
}

fn download_first(model_name: String, safe: bool, only_meta: bool, only_model: bool, dir: PathBuf) -> () {
    let model = get_first_query_item(model_name, safe);
    if !only_meta { download(model.clone(), dir.clone()) }
    if !only_model { write_report(model, dir) }
}

fn concatenate_query_items(queries: Vec<QueryItem>, full: bool) -> String {
    let mut cli_output = String::new();
    let mut i = queries.len() as u8;
    queries.iter().for_each(|q| -> () {
            let item_header = format!("\n[{}]=========\n", i);
            cli_output.push_str(&item_header);
            cli_output.push_str(q.make_cli_query_display(full).as_str());
            i -= 1;
        });
    cli_output
}

#[tokio::main]
async fn download(model: QueryItem, dir: PathBuf) {
    let id = model.get_download_id();
    let filename = model.get_model_filename();
    let size_mb = model.get_model_filesize() * 0.001;
    let file_path = format!("{}/{}", dir.display(), filename);
    println!("{} {}MB", MSG_DOWNLOAD_START, size_mb);
    match download_civitai_model_by_id(id, file_path).await {
        Ok(_) => println!("{}", MSG_DOWNLOAD_SUCCESS),
        Err(e) => println!("{}\n{}", e, MSG_DOWNLOAD_FAIL),
    };
}

fn write_report(model: QueryItem, dir: PathBuf) {
    let filename = model.get_model_filename();
    let report = model.generate_model_report().join("\n");
    let file_path = format!("{}/{}{}", dir.display(), filename, REPORT_FORMAT);
    let file = File::create(file_path);
    let _ = file.expect(ERR_FILE_WRITE).write_all(&report.as_bytes());
}

fn run(args: Args) -> Result<()> {
    dbg!{&args};
    let count = args.count;
    if count > 100 { panic!("{}", ERR_COUNT_TOO_BIG )}
    let safe = args.safe;
    let full = args.full;
    let only_model = args.only_model;
    let only_meta = args.meta;
    let interactive = args.interactive;
    //let model_name = args.model_name;

    let env_directory = match env::var(ENV_MODEL_DIR).is_ok() {
        true => PathBuf::from(env::var(ENV_MODEL_DIR).unwrap()),
        false => env::current_dir().unwrap(),
    };
    let dir = match args.directory {
        Some(directory) => directory,
        None => env_directory,
    };

    if only_model && only_meta { println!("{}\n{}", ERR_MUTUALLY_EXCLUSIVE, MSG_DRY_RUN) }

    if args.url.is_some() {
        let u = args.url.unwrap();
        let url = get_model_file_url(u);
        println!("{}", url);
    }

    if args.query.is_some() {
        let q = args.query.unwrap();
        let query = civitai_api::get_query_items(q, count, safe);
        print_query(query, full)
    }

    if args.model_name.is_some() {
        let model_name = args.model_name.unwrap();
        if interactive {
            let query = civitai_api::get_query_items(model_name, count, safe);
            let len = query.len() + 1;
            print_query(query.clone(), full);
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect(STDIN_FAILED);
            let trimmed = user_input.trim();
            let user_selection: usize;
            match trimmed.parse::<usize>() {
                Ok(i) => {
                    println!("{}{}", STDIN_GETTING, i); 
                    user_selection = i; }
                Err(..) => panic!("{}", STDIN_INVALID),
            };
            if user_selection >= len { panic!("{}", STDIN_OUT_OF_RANGE) }
            else {
                let desired_model = query[user_selection - 1].clone();
                if !only_meta { download(desired_model.clone(), dir.clone()) }
                if !only_model { write_report(desired_model, dir) }
            }
        } else {
            download_first(model_name, safe, only_meta, only_model, dir)
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    run(args)
}
