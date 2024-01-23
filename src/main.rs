use clap::Parser;
use clap_num::number_range;
use std::env;
use anyhow::Result;
use std::path::PathBuf;
use std::fs::File;
use std::io;
use std::io::Write;
use libvorpal::*;

mod test;

const DEFAULT_COUNT: u8 = 15;
const REPORT_FORMAT: &str = ".txt";
const ENV_MODEL_DIR: &str = "MODEL_DIRECTORY";
const ERR_COUNT_TOO_BIG: &str = "Vorpal: Maximum query count allowed by API is 100";
const ERR_MUTUALLY_EXCLUSIVE: &str = "Vorpal: These arguments are mutually exclusive. The -m argument is meant for only downloading metadata, and the -o argument is for only downloading models.";
const MSG_DRY_RUN: &str = "Vorpal: Performing dry run (no download)";
const MSG_WRITE_SUCCESS: &str = "Vorpal: Wrote metadata file";
const ERR_WRITE_FAIL: &str = "Vorpal: An error occured when writing the metadata file.\nDo you have write permission?";
const MSG_DOWNLOAD_START: &str = "Vorpal: Starting download...";
const MSG_DOWNLOAD_SUCCESS: &str = "Vorpal: Download successful! Enjoy your model!";
const MSG_DOWNLOAD_FAIL: &str = "Vorpal: Download failed";
const STDIN_FAILED: &str = "Failed to get input";
const STDIN_INVALID: &str = "Vorpal: Only input integers";
const STDIN_OUT_OF_RANGE: &str = "Vorpal: The number you entered is not in the query";
const STDIN_GETTING: &str = "Getting item: ";

fn check_limit(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// The name of the model to download. First result will be downloaded.
    model_name: Option<String>,

    /// Run in get-first mode (download first model from query).
    #[arg(short, long, default_value_t = false)]
    get_first: bool,

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
    #[arg(short, long, default_value_t = DEFAULT_COUNT, value_name = "COUNT", value_parser=check_limit)]
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
    let test = model.get_download_url();
    let filename = model.get_model_filename();
    let size_mb = model.get_model_filesize() * 0.001;
    let file_path = format!("{}/{}", dir.display(), filename);
    println!("{} {:.2}MB", MSG_DOWNLOAD_START, size_mb);
    match download_file_by_url(test, file_path).await {
        Ok(_) => println!("{}", MSG_DOWNLOAD_SUCCESS),
        Err(e) => println!("{}\n{}", e, MSG_DOWNLOAD_FAIL),
    };
}

fn write_report(model: QueryItem, dir: PathBuf) -> () {
    let filename = model.get_model_filename();
    let report = model.generate_model_report().join("\n");
    let file_path = format!("{}/{}{}", dir.display(), filename, REPORT_FORMAT);
    let file = File::create(file_path);
    let written = file.expect(ERR_WRITE_FAIL).write_all(&report.as_bytes());
    match written {
        Ok(()) => println!("{}", MSG_WRITE_SUCCESS),
        Err(e) => println!("{}\n{}", e, ERR_WRITE_FAIL),
    }
}

fn run(args: Args) -> Result<()> {
    dbg!{&args};
    let count = args.count;
    if count > 100 { panic!("{}", ERR_COUNT_TOO_BIG )}
    let safe = args.safe;
    let full = args.full;
    let only_model = args.only_model;
    let only_meta = args.meta;
    let get_first = args.get_first;
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
        let query = get_query_items(q, count, safe);
        print_query(query, full)
    }

    if args.model_name.is_some() {
        let model_name = args.model_name.unwrap();
        if !get_first {
            let query = get_query_items(model_name, count, safe);
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