use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use tera::{Context, Tera};

use crate::data::{Data, Metadata, Wrapper};

pub fn create_html(wrapper: Wrapper) -> Result<String> {
    let tera = Tera::new("templates/**/*.*").unwrap();
    let mut context = Context::from_serialize(wrapper)?;
    context.insert("styles", include_str!("static/styles.css"));
    Ok(tera.render("resume.html", &context)?)
}
