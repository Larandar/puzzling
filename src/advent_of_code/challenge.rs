use std::path::PathBuf;

use crate::config::Settings;
use crate::prelude::*;
use anyhow::Context;
use reqwest::{blocking::Client, StatusCode};

pub fn daily_challenge(year: usize, day: usize) -> Result<String> {
    if let Ok(challenge) = try_getting_cached_challenge(year, day) {
        return Ok(challenge);
    }
    info!("Cache miss, fetching fresh challenge");

    let challenge = fetch_daily_challenge(year, day)
        .with_context(|| format!("Fetching daily challenge of: {}/{}", year, day))?;
    info!("Challenge fetched: Content-Length: {}", challenge.len());

    info!("Writing challenge to cache");
    if let Err(e) = try_writing_challenge_to_cache(year, day, challenge.clone()) {
        warn!("Could not save challenge to cache: {:?}", e);
    }

    Ok(challenge)
}

fn challenge_cache_path(year: usize, day: usize) -> PathBuf {
    std::env::temp_dir()
        .canonicalize()
        .unwrap()
        .join("cache")
        .join(year.to_string())
        .join(day.to_string())
        .with_extension("txt")
}

fn try_getting_cached_challenge(year: usize, day: usize) -> Result<String> {
    let path = challenge_cache_path(year, day);
    info!("Looking for cache file: {:?}", path);

    // TODO(caching): check for cache invalidation
    if !path.exists() {
        return Err(anyhow!("Cache miss"));
    }

    let cache_age = std::fs::metadata(path.clone())
        .context("Error fetching cache metadata")?
        .modified()?
        .elapsed()?
        .as_secs();

    if cache_age < Settings::get().advent_of_code.cache_time {
        info!("Cache hit: {}s", cache_age);
        std::fs::read_to_string(path).context("Error reading cache contents")
    } else {
        warn!("Cache is cold");
        Err(anyhow!("Cache invalidated"))
    }
}

fn try_writing_challenge_to_cache(year: usize, day: usize, contents: String) -> Result<()> {
    let path = challenge_cache_path(year, day);

    // Create all sub dirs
    let parent = path
        .parent()
        .ok_or(anyhow!("could not traverse path"))
        .context("Looking for cache directory")?;
    std::fs::create_dir_all(parent).context("Creating all needed sub-directory for cache")?;

    // Remove previous cache value
    if path.exists() {
        std::fs::remove_file(path.clone()).context("Removing invalidated cache")?
    };

    // Write new contents
    std::fs::write(path, contents).context("Writing cache content")?;

    Ok(())
}

fn fetch_daily_challenge(year: usize, day: usize) -> Result<String> {
    let res = Client::new()
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header(
            reqwest::header::COOKIE,
            format!("session={}", Settings::get().advent_of_code.session_secret),
        )
        .send()
        .context("Failed to send request")?;

    match res.status() {
        StatusCode::OK => res.text().context("Failed to read response body"),
        StatusCode::NOT_FOUND => Err({
            let text = res.text();
            if text.is_ok() && text.unwrap().contains("Please log in") {
                error!("Invalid or expired session");
                anyhow!("Invalid or expired session")
            } else {
                warn!("Input not yet available");
                anyhow!("Input not yet available")
            }
        }),
        unhandled => match res.error_for_status() {
            Ok(_) => Err(anyhow!("Expected to fail for status: {:?}", unhandled)),
            Err(e) => Err(e).with_context(|| format!("Unhandled HTTP error: {:?}", unhandled)),
        },
    }
}
