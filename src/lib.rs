//! Easily search and download models from Civitai.
//! 
//! This is the library used in Vorpal, a package manager-esque CLI 
//! downloading and querying tool for Civitai.
//! 
//! Also included is a general-purpose downnloading function that
//! simply requires a URL and filepath, because the alternatives had
//! cryptic documentation or no library target.
//! 
//! There is also a function for shortening unicode strings, because
//! certain errors can come about due to Chinese characters or emojis
//! skewing character indices.

use std::fs::File;
use std::io::Write;
use std::fs::remove_file;
use futures_util::StreamExt;
use serde::Deserialize;
use scraper;
use serde_json;
use reqwest::Error;

const ERR_CONNECTION: &str = "Vorpal: Error in getting JSON. Ensure you have a stable internet connection. Try connecting to 1.1.1.1, 0.0.0.0, or google.com in a browser to ensure DNS connectivity.\n";
const ERR_GET_JSON: &str = "Vorpal: Error in getting JSON. This is likely due to trying to parse an invalid query.\n";
const ERR_FETCH: &str = "Vorpal: Failed to fetch download. This could be the result of an unstable connection.\n";
const ERR_FILE_CREATE: &str = "Vorpal: Failed to create file. Is the file path clear?\n";
const ERR_FILE_DOWNLOAD: &str = "Vorpal: Something went wrong while downloading the file. Is your connection stable?\n";
//const ERR_FILE_WRITE: &str = "Vorpal: Something went wrong when writing to the file.\n"; //TODO
const ERR_FILE_DELETE: &str = "Vorpal: Something went wrong when deleting the file.\nThe model file is likely corrupted, and vorpal is unable to delete it.";
const QUERY_INDENT: &str = "    ";
const SHORT_SIZE: usize = 100;
const DESC_CUTOFF: &str = "...";
const MSG_NO_RESULTS: &str = "Vorpal: No results were found.\n";
const BASE_DL_URL: &str = "https://civitai.com/api/download/models/"; 
const NO_DESC: &str = "<No description given>";

#[derive(Deserialize, Debug)]
/// A vector of QueryItems sent from Civitai
pub struct QueryResponse {
    pub items: Vec<QueryItem>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A struct that contains important metadata of a Civitai "model".
/// Take this model, for example:
/// https://civitai.com/models/235002/sdxl-red-glitter
/// 
/// The QueryItem will contain the important metadata used to describe, catalog, and donwload
/// the red glitter model.
/// 
/// It should be noted that a QueryItem is not a model file or metadata for one model.
/// The model_version struct within the QueryItem contains the versions of the model, which then 
/// contains a Vec of the ModelFiles struct. Also note that QueryItems, model versions, and model 
/// files all have their own, separate Ids. The Id used in download links belongs to the model 
/// version, and can be accessed with QueryItem's get_download_id().
pub struct QueryItem {
    name: String,
    id: u32,
    description: Option<String>,
    creator: Creator,
    tags: Vec<String>,
    stats: Stats,
    model_versions: Vec<ModelVersion>,
}

#[derive(Deserialize, Debug, Clone)] // Could be expanded later
struct Creator {
    username: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// A struct of some basic stats from a QueryItem
pub struct Stats {
    download_count: u32,
    favorite_count: u32,
    comment_count: u32,
    rating_count: u32,
    rating: f32,
    tipped_amount_count: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// The version of a Civitai model, which contains files, trained words, and
/// other metadata. For most use cases, it would likely be much easier to
/// utilize the QueryItem built-in methods for getting things like urls.
pub struct ModelVersion {
    id: u32,
    model_id: u32,
    name: String,
    trained_words: Vec<String>,
    base_model: Option<String>,
    base_model_type: Option<String>,
    files: Vec<ModelFile>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// File metadata of a Civitai model file. Note that the Id here is separate from
/// the (useful) Id of the model version. The file Id is likely not going to see 
/// use, but is included here just in case.
pub struct ModelFile {
    id: u32,
    #[serde(rename = "sizeKB")]
    size_kb: f64,
    name: String,
    download_url: String,
}

type JsonResult = Result<String, Error>;
type QueryResult = Result<QueryResponse, Error>;

#[tokio::main]
async fn get_raw_civitai_json(query: String, limit: u8, safe: bool) -> JsonResult {
    let request_url = format!("https://civitai.com/api/v1/models?limit={}&query={}&nsfw={}",
                                        limit.to_string(),
                                        query,
                                        safe,
);
    let res = reqwest::get(&request_url).await?;
    let body = res.text().await;
    body
}

fn parse_civitai_json(raw: JsonResult) -> QueryResult {
    let raw_unwrapped = match raw {
        Ok(_) => raw.unwrap(),
        Err(e) => panic!("{}\n{}", e, ERR_CONNECTION),
    };
    //let raw_unwrapped = raw.unwrap();
    let test: QueryResponse = serde_json::from_str(&raw_unwrapped).expect(ERR_GET_JSON);
    Ok(test)
}

/// Query Civitai for models. Returns a Vector of QueryItems
/// Args:
///     search - the keyword to query
///     count - the amount of results to display
///     safe - enter query as 'safe'
pub fn get_query_items(search: String, count: u8, safe: bool) -> Vec<QueryItem> {
    let raw = get_raw_civitai_json(search, count, safe);
    let parsed = parse_civitai_json(raw);
    let items: Vec<QueryItem>;
    match &parsed {
        Ok(_) => items = parsed.unwrap().get_items(),
        Err(e) => panic!("{}", e),
    };
    if items.len() == 0 { panic!("{}", MSG_NO_RESULTS) }
    items
}

/// Find only the url of the first model from a Civitai query
/// The most recent model version and file will be used
pub fn get_model_file_url(search: String) -> String {
    let count = 1;
    let safe = false;
    let queryitem = &get_query_items(search, count, safe)[0];
    queryitem.get_download_url()
}

/// Return the first query item from a Civitai query. Intended for quickly downloading
/// a model from a simple search.
/// Args:
///     search - The search term to get models from Civitai
///     safe - Enter search as 'safe' (no NSFW)
///            Note that this is done on a 'best effort' basis,
///            as it is very common for users to not properly
///            label their items.
pub fn get_first_query_item(search: String, safe: bool) -> QueryItem {
    let count = 1;
    let query = get_query_items(search, count, safe);
    let first = query[0].clone();
    first
}

impl QueryResponse {
    pub fn get_items(self) -> Vec<QueryItem> {
        self.items
    }
}

fn handle_remove(path: &str, e: &str) -> () {
    remove_file(path).expect(ERR_FILE_DELETE);
    panic!("{}", e) //TODO add err msg
}

async fn perform_validated_download(mut file: File, path: String, res: reqwest::Response) -> () {
    let stream = &mut res.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(ERR_FILE_DOWNLOAD));
        match &chunk {
            Ok(c) => file.write_all(c).expect(ERR_CONNECTION),
            Err(e) => handle_remove( &path, e)
        }
    }
}

/// Download a Civitai model given the Id (of the model version).
/// This is the same Id that will appear on the Civitai page for
/// that model. The get_download_url() of QueryItem can be used to
/// find this.
/// 
/// Panics:
///     - If reqwest cannot establish connection
///     - If file cannot be created (file will be removed)
///     - If corrupted file cannot be deleted
pub async fn download_civitai_model_by_id(id: String, path: String) -> Result<(), Error> {
    let url = format!("{BASE_DL_URL}{id}");
    download_file_by_url(url, path).await
}


/// Download a file given a url and path.
/// 
/// Panics:
///     - If reqwest cannot establish connection
///     - If file cannot be created (file will be removed)
///     - If corrupted file cannot be deleted
pub async fn download_file_by_url(url: String, path: String) -> Result<(), Error> {
    let res = reqwest::get(url)
        .await
        .or(Err(ERR_FETCH));

    let validated_res = match res {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    let file = File::create(&path);
    match file {
        Ok(f) => {
            perform_validated_download(f, path, validated_res).await;
            return Ok(())
        },
        Err(e) => {
            panic!("{}\n{}", e, ERR_FILE_CREATE);
        },
    }
}

impl QueryItem {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
    pub fn get_tags(&self) -> String {
        let tags = self.tags.join(", ");
        tags
    }

    pub fn get_creator_name(&self) -> String {
        self.creator.username.clone()
    }

    #[allow(dead_code)]
    pub fn get_model_versions(&self) -> Vec<ModelVersion> {
        self.model_versions.clone()
    }

    pub fn get_first(&self) -> ModelVersion {
        let model_versions = self.model_versions.clone();
        model_versions[0].clone()
    }

    /// Get model description without HTML artifacts
    pub fn get_description(&self) -> String {
        let desc = match self.description.clone(){
            Some(desc) => desc,
            None => NO_DESC.to_string(),
        };
        let frag = scraper::Html::parse_fragment(&desc);
        let mut trimmed = String::from("");
        for node in frag.tree{
            if let scraper::node::Node::Text(text) = node {
                trimmed += &text.text
            }
        }
        trimmed
    }

    /// Attempt to get a short description for the model. 
    /// If the description does not need to be shortened, it will remain untouched.
    /// Args:
    ///     len - The length to shorten the description to
    ///     trail - What to add at the end of the description (ex. ...)
    pub fn get_short_description(&self, len: usize, trail: &str) -> String {
        let desc = self.get_description();
        if desc.len() > len {
            shorten_unicode(self.get_description(), len, trail)
        } else {
            self.get_description()
        }
    }


    /// Make a list of metadata that can be used in a txt file
    pub fn generate_model_report(&self) -> Vec<String> {
        let mut report_fields: Vec<String> = Vec::new();
        let version_metadata = self.get_first().get_version_metadata();
        let file_metadata = self.get_first().get_latest_file().get_file_metadata();
        report_fields.extend(version_metadata);
        report_fields.extend(file_metadata);
        report_fields
    }


    pub fn get_download_url(&self) -> String {
        let model_version = self.get_first();
        let model_file = model_version.get_latest_file();
        model_file.download_url
    }

    pub fn get_download_id(&self) -> String {
        let model_version = self.get_first();
        model_version.get_model_id()
    }

    pub fn get_model_filename(&self) -> String {
        let model_version = self.get_first();
        let model_file = model_version.get_latest_file();
        model_file.name
    }

    pub fn get_model_filesize(&self) -> f64 {
        let model_version = self.get_first();
        let model_file = model_version.get_latest_file();
        model_file.size_kb
    }

    /// Generate CLI-oriented output of QueryItem
    /// Args:
    ///     full - true for full description, false for short description
    pub fn make_cli_query_display(&self, full: bool) -> String {
        let mut display_vec: Vec<String> = Vec::new();
        display_vec.push(format!("{}Model: {}", QUERY_INDENT, self.get_model_filename()));
        display_vec.push(format!("{}Id: {}", QUERY_INDENT, self.get_id()));
        display_vec.push(format!("{}Size (KB): {}", QUERY_INDENT, self.get_model_filesize()));
        display_vec.push(format!("{}Creator: {}", QUERY_INDENT, self.get_creator_name()));
        display_vec.push(format!("{}Tags: {}", QUERY_INDENT, self.get_tags()));
        match full {
            true => display_vec.push(format!("{}Desc: {}", QUERY_INDENT, self.get_description())),
            false => display_vec.push(format!("{}Desc: {}", QUERY_INDENT, self.get_short_description(SHORT_SIZE, DESC_CUTOFF))),
        }
        display_vec.push("\n".to_string());
        display_vec.join("\n")
    }
}

impl ModelVersion {
    fn get_model_id(&self) -> String {
        self.id.to_string()
    }

    #[allow(dead_code)]
    fn get_version_id(&self) -> String {
        self.model_id.to_string()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_trained_words(&self) -> String {
        self.trained_words.join(", ")
    }

    fn get_latest_file(&self) -> ModelFile {
        self.files[0].clone()
    }

    fn get_version_metadata(&self) -> Vec<String> {
        let mut version_metadata: Vec<String> = Vec::new();
        version_metadata.push(format!("Model Name/Version: {}", self.get_name()));
        version_metadata.push(format!("Trained Words: {}", self.get_trained_words()));
        version_metadata.push(format!("Id: {}", self.get_name()));
        version_metadata.push(format!("Model Name/Version: {}", self.get_name()));
        version_metadata
    }
}

impl ModelFile {
    fn get_id(&self) -> String {
        self.id.to_string()
    }

    fn get_size(&self) -> String {
        self.size_kb.to_string()
    }
    
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_url(&self) -> String {
        self.download_url.clone()
    }
    fn get_file_metadata(&self) -> Vec<String> {
        let mut file_metadata: Vec<String> = Vec::new();
        file_metadata.push(format!("Filename: {}", self.get_name()));
        file_metadata.push(format!("Url: {}", self.get_url()));
        file_metadata.push(format!("File Id: {}", self.get_id()));
        file_metadata.push(format!("File Size (KB): {}", self.get_size()));
        file_metadata
    }
}

use unicode_segmentation::UnicodeSegmentation;

/// Shorten a unicode string and add a trail at the end (ex. ...).
/// Can handle Chinese script, emojis, and other scripts.
/// Args:
///     string - the string to shorten
///     len - The length to shorten the description to
///     trail - What to add at the end of the description (ex. ...)
pub fn shorten_unicode(string: String, length: usize, trail: &str) -> String {
    let graphemes = string.grapheme_indices(true);
    let graph_vec: Vec<_> = graphemes.take(length).into_iter().collect();
    let mut unpacked: Vec<&str> = vec![];
    for grapheme in graph_vec {
        unpacked.push(grapheme.1)
    }
    let joined = unpacked.join("");
    
    //dbg!{&graphemes};
    let shortened = format!("{}{}", joined, trail);
    shortened
}
