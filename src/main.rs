#![forbid(unsafe_code)]
#![forbid(rust_2018_idioms)]

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use clap::Clap;
use iced::{Application, Settings};
use log::info;

mod explorer;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.dump_tree {
        dump_tree(args.file.as_ref().unwrap());
    } else {
        init_logging();
        explorer::Explorer::run(Settings {
            flags: args,
            ..Default::default()
        })?;
    }
    Ok(())
}

#[derive(Clap, Default)]
struct Args {
    file: Option<PathBuf>,
    #[clap(long, requires("file"))]
    dump_tree: bool,
}

fn dump_tree(path: &Path) {
    use std::fmt::Write;

    const UNNAMED_NODE: &str = "<unnamed node>";

    let (document, _, _) = gltf::import(path).unwrap();

    let mut out = String::new();
    let mut children_stack = Vec::new();

    for scene in document.scenes() {
        out.push_str(scene.name().unwrap_or("<unnamed scene>"));
        if document.default_scene().map(|s| s.index()) == Some(scene.index()) {
            out.push_str(" [default]");
        }
        out.push('\n');

        let mut root_nodes = scene.nodes().peekable();

        while let Some(root_node) = root_nodes.next() {
            if root_nodes.peek().is_some() {
                writeln!(out, "├── {}", root_node.name().unwrap_or(UNNAMED_NODE)).ok();
            } else {
                writeln!(out, "└── {}", root_node.name().unwrap_or(UNNAMED_NODE)).ok();
            }

            children_stack.push(root_node.children().peekable());

            while let Some(mut children) = children_stack.pop() {
                if let Some(child) = children.next() {
                    write!(out, "    ").ok();
                    for node in children_stack.iter_mut() {
                        if node.peek().is_some() {
                            write!(out, "│   ").ok();
                        } else {
                            write!(out, "    ").ok();
                        }
                    }

                    if children.peek().is_some() {
                        writeln!(out, "├── {}", child.name().unwrap_or(UNNAMED_NODE)).ok();
                    } else {
                        writeln!(out, "└── {}", child.name().unwrap_or(UNNAMED_NODE)).ok();
                    }

                    children_stack.push(children);
                    children_stack.push(child.children().peekable());
                }
            }
        }
    }

    print!("{}", out);
}

fn init_logging() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let now = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]");
            if let (Some(module), Some(line)) = (record.module_path(), record.line()) {
                out.finish(format_args!(
                    "{}[{}:{:<4}][{}] {}",
                    now,
                    module,
                    line,
                    record.level(),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    now,
                    record.target(),
                    record.level(),
                    message
                ))
            }
        })
        .level(log::LevelFilter::Warn)
        .level_for("gltf_explorer", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .expect("Could not initialize logging");

    info!("Logging initalized");
}
