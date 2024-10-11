use std::{fmt::Display, str::FromStr};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
/// Entity type
pub enum Entity {
    /// Categories
    Categories,
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Entity::Categories => "categories",
            }
        )
    }
}

impl FromStr for Entity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "categories" => Ok(Self::Categories),
            _ => Err(()),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
/// Events
pub enum Event {
    /// Sets a single item in cache and search index
    SetSingle(Entity),
    /// Sets a batch of items in cache and search index
    SetBatch(Entity),
    /// Updates a single item in cache and search index
    UpdateSingle(Entity),
    /// Updates a batch of items in cache and search index
    UpdateBatch(Entity),
    /// Deletes a single item from cache and search index
    DeleteSingle(Entity),
    /// Deletes a batch of items from cache and search index
    DeleteBatch(Entity),
    /// Updates a single item in cache only
    CacheUpdateSingle(Entity),
    /// Updates a batch of items in cache only
    CacheUpdateBatch(Entity),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Event::SetSingle(entity) => {
                    format!("{entity}.update.index.set.single")
                }
                Event::SetBatch(entity) => {
                    format!("{entity}.update.index.set.batch")
                }
                Event::UpdateSingle(entity) => {
                    format!("{entity}.update.index.update.single")
                }
                Event::UpdateBatch(entity) => {
                    format!("{entity}.update.index.update.batch")
                }
                Event::CacheUpdateSingle(entity) => {
                    format!("{entity}.update.set.single")
                }
                Event::DeleteSingle(entity) => {
                    format!("{entity}.update.index.delete.single")
                }
                Event::DeleteBatch(entity) => {
                    format!("{entity}.update.index.delete.batch")
                }
                Event::CacheUpdateBatch(entity) => {
                    format!("{entity}.update.set.single")
                }
            }
        )
    }
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('.');
        let entity = tokens.next().ok_or(())?;

        let entity = Entity::from_str(entity)?;

        let action = tokens.next().ok_or(())?;
        let scope = tokens.next().ok_or(())?;
        let operation = tokens.next().ok_or(())?;
        let item = tokens.next();

        match (action, scope, operation, item) {
            ("update", "index", "set", Some("single")) => Ok(Event::SetSingle(entity)),
            ("update", "index", "set", Some("batch")) => Ok(Event::SetBatch(entity)),
            ("update", "index", "update", Some("single")) => Ok(Event::UpdateSingle(entity)),
            ("update", "index", "update", Some("batch")) => Ok(Event::UpdateBatch(entity)),
            ("update", "set", "single", None) => Ok(Event::CacheUpdateSingle(entity)),
            ("update", "index", "delete", Some("single")) => Ok(Event::DeleteSingle(entity)),
            ("update", "index", "delete", Some("batch")) => Ok(Event::DeleteBatch(entity)),
            ("update", "set", "batch", None) => Ok(Event::CacheUpdateBatch(entity)),
            _ => Err(()),
        }
    }
}
