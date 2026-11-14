//! Preprocessor mdBook pour remplacer les balises {{paint:NOM}} par des pastilles CSS.
//!
//! Utilise le catalogue Two Thin Coats depuis data/ttc_paints.toml

extern crate docopt;
extern crate mdbook_preprocessor;
extern crate regex;
extern crate toml;
extern crate serde_derive;
extern crate serde;

use docopt::Docopt;
use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::{Error, Result as MdResult},
    Preprocessor, PreprocessorContext,
};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process;

static NAME: &str = "paint-preprocessor";
static USAGE: &str = "
Usage:
    paint-preprocessor
    paint-preprocessor supports <renderer>
";

#[derive(Deserialize)]
struct Args {
    pub cmd_supports: bool,
    pub arg_renderer: Option<String>,
}

/// Représente une peinture Two Thin Coats
#[derive(Debug, Deserialize)]
struct PaintEntry {
    brand: String,
    name: String,
    #[allow(dead_code)]
    color: Option<String>,
    hex: Option<String>,
    #[allow(dead_code)]
    layer: Option<String>,
    #[allow(dead_code)]
    citadel_equivalent: Option<String>,
}

/// Structure pour parser le TOML ttc_paints.toml
#[derive(Debug, Deserialize)]
struct TtcPaints {
    paints: Vec<PaintEntry>,
}

/// Catalogue des peintures avec identifiant normalisé -> données
struct PaintCatalog {
    map: HashMap<String, PaintEntry>,
}

impl PaintCatalog {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, entry: PaintEntry) {
        let ident = normalize_paint_name(&entry.name);
        self.map.insert(ident, entry);
    }

    fn get(&self, ident: &str) -> Option<&PaintEntry> {
        self.map.get(ident)
    }
}

/// Normalise un nom de peinture en identifiant
fn normalize_paint_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace("'", "")
        .replace("(", "")
        .replace(")", "")
}

/// Charge le catalogue depuis data/ttc_paints.toml
fn load_paint_catalog(book_root: &PathBuf) -> MdResult<PaintCatalog> {
    let ttc_path = book_root.join("data").join("ttc_paints.toml");
    
    if !ttc_path.exists() {
        return Err(Error::msg(format!(
            "Fichier de catalogue introuvable: {}",
            ttc_path.display()
        )));
    }

    let content = fs::read_to_string(&ttc_path)
        .map_err(|e| Error::msg(format!("Erreur de lecture de {}: {}", ttc_path.display(), e)))?;

    let ttc_paints: TtcPaints = toml::from_str(&content)
        .map_err(|e| Error::msg(format!("Erreur de parsing TOML de {}: {}", ttc_path.display(), e)))?;

    let mut catalog = PaintCatalog::new();
    for entry in ttc_paints.paints {
        catalog.add(entry);
    }

    Ok(catalog)
}

/// Remplace les balises {{paint:NOM}} dans un contenu texte
fn replace_paint_tags(content: &str, catalog: &PaintCatalog) -> MdResult<String> {
    let re = Regex::new(r"\{\{\s*paint:\s*([a-zA-Z0-9\-_]+)\s*\}\}").unwrap();
    
    let mut result = String::with_capacity(content.len());
    let mut last_end = 0;
    
    for cap in re.captures_iter(content) {
        let full_match = cap.get(0).unwrap();
        let paint_name = cap.get(1).unwrap().as_str();
        
        result.push_str(&content[last_end..full_match.start()]);
        
        match catalog.get(paint_name) {
            Some(paint) => {
                let hex = paint.hex.as_deref().unwrap_or("#000000");
                let html = format!(
                    r#"<span class="paint-chip" style="background-color: {};" title="{} - {}">{}</span>"#,
                    hex,
                    paint.name,
                    paint.brand,
                    paint.name
                );
                result.push_str(&html);
            }
            None => {
                return Err(Error::msg(format!(
                    "Peinture inconnue: '{}'. Ajoutez-la dans data/ttc_paints.toml",
                    paint_name
                )));
            }
        }
        
        last_end = full_match.end();
    }
    
    result.push_str(&content[last_end..]);
    
    Ok(result)
}

/// Traite récursivement les items d'un livre
fn process_chapters<'a, I>(items: I, catalog: &PaintCatalog, excluded_files: &[&str]) -> MdResult<()>
where
    I: IntoIterator<Item = &'a mut BookItem> + 'a,
{
    for item in items {
        if let BookItem::Chapter(ref mut ch) = *item {
            // Vérifier si le fichier doit être exclu
            if let Some(path) = &ch.path {
                if let Some(path_str) = path.to_str() {
                    if excluded_files.contains(&path_str) {
                        // Traiter récursivement les sous-items mais pas ce chapitre
                        process_chapters(&mut ch.sub_items, catalog, excluded_files)?;
                        continue;
                    }
                }
            }
            
            // Traiter le contenu du chapitre
            let new_content = replace_paint_tags(&ch.content, catalog)?;
            ch.content = new_content;
            
            // Traiter récursivement les sous-items
            process_chapters(&mut ch.sub_items, catalog, excluded_files)?;
        }
    }
    Ok(())
}

/// Le preprocessor
struct PaintPreprocessor;

impl Preprocessor for PaintPreprocessor {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> MdResult<Book> {
        // Charger le catalogue de peintures
        let catalog = load_paint_catalog(&ctx.root)?;
        
        // Fichiers à exclure du preprocessing (ex: documentation technique)
        let excluded_files = vec!["plan.md"];
        
        // Traiter toutes les sections du livre
        process_chapters(&mut book.items, &catalog, &excluded_files)?;
        
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> MdResult<bool> {
        // Ce preprocessor fonctionne avec le renderer HTML
        match renderer {
            "html" => Ok(true),
            _ => Ok(false),
        }
    }
}

fn main() {
    // Parser les arguments
    let args: Args = Docopt::new(USAGE)
        .and_then(|a| a.deserialize())
        .unwrap_or_else(|e| e.exit());
    
    let pre = PaintPreprocessor;
    
    if args.cmd_supports {
        if let Some(renderer) = args.arg_renderer {
            if pre.supports_renderer(&renderer).unwrap_or(false) {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
    }
    
    // Mode normal : traiter le livre
    let (ctx, book) = match mdbook_preprocessor::parse_input(::std::io::stdin()) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            process::exit(1);
        }
    };
    
    let processed_book = pre.run(&ctx, book).unwrap_or_else(|e| {
        eprintln!("Error in preprocessor: {}", e);
        process::exit(1);
    });
    
    serde_json::to_writer(::std::io::stdout(), &processed_book).unwrap_or_else(|e| {
        eprintln!("Error writing output: {}", e);
        process::exit(1);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_paint_name() {
        assert_eq!(normalize_paint_name("Scorched Earth"), "scorched-earth");
        assert_eq!(normalize_paint_name("Berserker Red"), "berserker-red");
        assert_eq!(normalize_paint_name("Doom Death Black"), "doom-death-black");
        assert_eq!(normalize_paint_name("White Star"), "white-star");
    }

    #[test]
    fn test_replace_paint_tags() {
        let mut catalog = PaintCatalog::new();
        catalog.add(PaintEntry {
            brand: "Two Thin Coats".to_string(),
            name: "Test Red".to_string(),
            color: None,
            hex: Some("#FF0000".to_string()),
            layer: None,
            citadel_equivalent: None,
        });

        let content = "Armure: {{paint:test-red}}";
        let result = replace_paint_tags(content, &catalog).unwrap();
        assert!(result.contains("background-color: #FF0000"));
        assert!(result.contains("Test Red"));
    }
}
