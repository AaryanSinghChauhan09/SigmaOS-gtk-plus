// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-GTK-plus: GTK plus component model.
//!
//! Real implementation: pluss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plus {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GtkPlusError {
    EmptyPlusName,
    DuplicatePlus(String),
    EmptyEventSubject,
}

impl fmt::Display for GtkPlusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPlusName => write!(f, "plus name must not be empty"),
            Self::DuplicatePlus(id) => write!(f, "duplicate plus {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GtkPlusError {}

pub struct GtkPlus {
    pub name: String,
    pluss: BTreeMap<String, Plus>,
    events: BTreeMap<String, Event>,
}

impl GtkPlus {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pluss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_plus(&mut self, item: Plus) -> Result<(), GtkPlusError> {
        if item.name.trim().is_empty() {
            return Err(GtkPlusError::EmptyPlusName);
        }
        let id = item.id.clone();
        if self.pluss.contains_key(&id) {
            return Err(GtkPlusError::DuplicatePlus(id.clone()));
        }
        self.pluss.insert(id, item);
        Ok(())
    }

    pub fn plus_count(&self) -> usize {
        self.pluss.len()
    }

    pub fn get_plus(&self, id: &str) -> Option<&Plus> {
        self.pluss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GtkPlusError> {
        if event.subject.trim().is_empty() {
            return Err(GtkPlusError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_plus(id: &str, name: &str) -> Plus {
        Plus {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GtkPlus::new("GtkPlus");
        assert_eq!(m.name, "GtkPlus");
        assert_eq!(m.plus_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_plus() {
        let mut m = GtkPlus::new("GtkPlus");
        m.add_plus(sample_plus("a1", "Alpha")).unwrap();
        assert_eq!(m.plus_count(), 1);
        assert!(m.get_plus("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GtkPlus::new("GtkPlus");
        assert!(m.add_plus(sample_plus("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GtkPlus::new("GtkPlus");
        m.add_plus(sample_plus("x", "One")).unwrap();
        assert!(m.add_plus(sample_plus("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GtkPlus::new("GtkPlus");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
