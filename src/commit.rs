//! Conventional commit

use std::{io::BufRead, str::FromStr, string::ToString};

use indoc::formatdoc;
use regex::Regex;

use crate::error::{Error, Result};

/// Conventional commit
pub struct Commit<T: ToString + FromStr + Sized> {
    /// Commit type
    pub r#type: T,
    /// Commit scope
    pub scope: Option<String>,
    /// Commit description
    pub description: String,
    /// Commit body
    pub body: Option<String>,
    /// Breaking change
    pub breaking_change: Option<String>,
}

impl<T: ToString + FromStr + Sized> ToString for Commit<T> {
    fn to_string(&self) -> String {
        formatdoc!(
            "
            {}{}: {}
            {}
            {}
            ",
            self.r#type.to_string(),
            self.scope
                .clone()
                .map(|s| format!("({})", s))
                .unwrap_or_default(),
            self.description,
            self.body
                .clone()
                .map(|s| format!("\n{}", s))
                .unwrap_or_default(),
            self.breaking_change
                .clone()
                .map(|s| format!("\nBREAKING CHANGE: {}", s))
                .unwrap_or_default(),
        )
    }
}

impl<T: ToString + FromStr<Err = Error> + Sized> FromStr for Commit<T> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let x: Vec<_> = s.chars().collect();
        let r#type = T::from_str("")?;
        let mut scope = None;
        let mut description = String::new();
        let mut body = None;
        let mut breaking_change = None;

        let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let mut is_footer = false;

        for (i, line_res) in s.as_bytes().lines().enumerate() {
            let line = line_res?;
            if i == 0 {

                // regex.find(text)
                // let parts: Vec<_> = line.splitn(2, ":").collect();
                // if parts.len() != 2 {
                //     return Err(Error::InvalidCommit("Missing colon".to_string()));
                // }
                // let type_scope = parts[0].trim().to_string();
                // description = parts[1].trim().to_string();
                // let scope_parts: Vec<_> = type_scope.splitn(2, "(").collect();
                // if scope_parts.len() != 2 {
                //     r#type = type_scope;
                // } else {
                //     let scope_parts_end: Vec<_> = scope_parts[1].splitn(2, ')').collect();
                //     if scope_parts_end.len() != 2 {
                //         r#type = scope_parts[0].trim().to_string();
                //     } else {
                //         if  {
                //             //
                //         }
                //         //
                //     }
                // }
            } else {
                // other lines
                if line.starts_with("BREAKING_CHANGE:") {
                    //
                }
            }
        }

        Ok(Commit {
            r#type,
            scope,
            description,
            body,
            breaking_change,
        })
    }
}
