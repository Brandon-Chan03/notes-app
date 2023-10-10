// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(unused_imports)]

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tauri::{Runtime, Window, Manager};

mod db;
mod models;
mod schema;
// mod database;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      db::init();

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}