// SPDX-License-Identifier: AGPL-3.0-only
//! Wutong Knowledge engine.
//! Local vault index: objects keyed by stable id, never by path; exact-bytes digests.

use std::collections::BTreeMap;
use wutong_core::wire::sha256_digest;

/// An indexed native file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Workspace-relative path (validated portable path).
    pub relative_path: String,
    /// `sha256:` of exact bytes.
    pub content_digest: String,
    /// Byte length.
    pub byte_length: usize,
    /// Lower-cased text for naive search (FTS lands with SQLite storage).
    text: String,
}

/// In-memory vault index.
#[derive(Debug, Default)]
pub struct Vault {
    entries: BTreeMap<String, Entry>,
    relations: Vec<(String, String, String)>,
}

impl Vault {
    /// Add or update a file; rejects non-portable paths.
    pub fn upsert(&mut self, relative_path: &str, bytes: &[u8]) -> Result<&Entry, String> {
        wutong_core::document::validate_relative_path(relative_path).map_err(|e| e.to_string())?;
        let e = Entry {
            relative_path: relative_path.to_owned(),
            content_digest: sha256_digest(bytes).to_string(),
            byte_length: bytes.len(),
            text: String::from_utf8_lossy(bytes).to_lowercase(),
        };
        self.entries.insert(relative_path.to_owned(), e);
        Ok(&self.entries[relative_path])
    }
    /// Record a typed relation (author-attributed in the protocol layer).
    pub fn relate(&mut self, source: &str, predicate: &str, target: &str) {
        self.relations
            .push((source.into(), predicate.into(), target.into()));
    }
    /// Outgoing relations of a path.
    pub fn relations_of(&self, source: &str) -> Vec<(&str, &str)> {
        self.relations
            .iter()
            .filter(|r| r.0 == source)
            .map(|r| (r.1.as_str(), r.2.as_str()))
            .collect()
    }
    /// Case-insensitive substring search over paths and text.
    pub fn search(&self, q: &str) -> Vec<&str> {
        let q = q.to_lowercase();
        self.entries
            .values()
            .filter(|e| e.text.contains(&q) || e.relative_path.to_lowercase().contains(&q))
            .map(|e| e.relative_path.as_str())
            .collect()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Empty?
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// One-line description used by the GUI.
pub fn describe(bytes: Vec<u8>) -> String {
    let words = String::from_utf8_lossy(&bytes).split_whitespace().count();
    format!("{} words · {}", words, sha256_digest(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_search_relations() {
        let mut v = Vault::default();
        v.upsert("notes/梧桐.md", "梧桐 Protocol notes".as_bytes())
            .unwrap();
        v.upsert("notes/b.md", b"other").unwrap();
        assert!(v.upsert("../escape.md", b"x").is_err());
        assert_eq!(v.search("protocol"), vec!["notes/梧桐.md"]);
        v.relate("notes/b.md", "references", "notes/梧桐.md");
        assert_eq!(
            v.relations_of("notes/b.md"),
            vec![("references", "notes/梧桐.md")]
        );
        assert_eq!(v.len(), 2);
    }
}
