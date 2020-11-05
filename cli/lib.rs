// #![deny(warnings)]	

#[macro_use]	
extern crate lazy_static;	
#[macro_use]	
extern crate log;	
extern crate clap;	
extern crate deno_core;	
extern crate indexmap;	
#[cfg(unix)]	
extern crate nix;	
extern crate rand;	
extern crate regex;	
extern crate serde;	
extern crate tokio;	

mod ast;
mod checksum;
mod colors;
mod coverage;
mod deno_dir;
mod diagnostics;
mod diff;
mod disk_cache;
mod errors;
mod file_fetcher;
mod file_watcher;
mod flags;
mod flags_allow_net;
mod fmt;
mod fmt_errors;
mod global_timer;
mod http_cache;
mod http_util;
mod import_map;
mod info;
mod inspector;
mod installer;
mod lint;
mod lockfile;
mod media_type;
mod metrics;
mod module_graph;
mod ops;
mod repl;
mod resolve_addr;
mod signal;
mod source_maps;
mod specifier_handler;
mod test_runner;
mod text_encoding;
mod tsc;
mod tsc_config;
mod upgrade;
mod version;

pub mod js;
pub mod worker;
pub mod tokio_util;
pub mod fs;
pub mod permissions;
pub mod module_loader;
pub mod program_state;

use crate::media_type::MediaType;
use deno_core::error::AnyError;
use deno_core::ModuleSpecifier;
use import_map::ImportMap;