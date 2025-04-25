use serde::{Deserialize, Serialize};
use skim::SkimItem;
use std::{borrow::Cow, error::Error, fs::{File, OpenOptions}, path::PathBuf};
use std::os::unix::fs::OpenOptionsExt;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ItemRaw {
    title: String,
    #[serde(default)]
    tags: Vec<String>,
    fields: Vec<Field>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Vault {
    name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Field {
    reference: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    section: Option<SectionRaw>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename = "Section")]
pub struct SectionRaw {
    pub label: String,
}

// Simplified representation of an op item for sanity
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub title: String,
    pub tags: Vec<String>,
    pub sections: Vec<Section>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Section {
    pub title: String,
    reference: String,
}

impl SkimItem for Section {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.title)
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.reference)
    }
}


#[allow(dead_code)]
impl Item {
    fn from_raw(raw: ItemRaw) -> Self {
        let mut reference = None;

        let sections = raw.fields
            .into_iter()
            .filter(|field| field.reference.contains("password"))
            .map(|field| {
                if field.section.is_none() && reference.is_none() {
                    reference = Some(field.reference.clone());
                }

                let section_title = match &field.section {
                    Some(section) => section.label.clone(),
                    None => raw.title.clone(),
                };

                Section {
                    title: section_title,
                    reference: field.reference,
                }
            })
            .collect();

        Item {
            title: raw.title,
            tags: raw.tags,
            sections,
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct OpItemSummary {
    pub id: String,
    pub title: String,
}

#[allow(dead_code)]
pub fn parse_item_from_json(json: &str) -> Result<Item, Box<dyn Error>>{
    let raw: ItemRaw = serde_json::from_str(json)?;
    Ok(Item::from_raw(raw))
}

#[allow(dead_code)]
/// Writes item to cache, creates it with 0600 if it does not already exist
pub fn write_items_to_cache(items: &Vec<Item>, filename: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(filename)?;
    serde_json::to_writer(file, items)?;
    Ok(())
}

#[allow(dead_code)]
/// Loads a vec of Item from file
pub fn load_items_from_cache(filename: PathBuf) -> Result<Vec<Item>, Box<dyn Error>> {
    let file = File::open(&filename)?;
    let items: Vec<Item> = serde_json::from_reader(file)?;
    Ok(items)
}
