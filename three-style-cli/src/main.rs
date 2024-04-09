use clap::{
    builder::styling::{AnsiColor, Color, Style},
    Parser, Subcommand,
};
use std::{
    process,
    str::FromStr,
    time::{Duration, Instant},
};
use three_style_lib::{
    commutator::{
        finder::{find_corner_commutators, find_edge_commutators},
        types::{Commutator, Cycle},
    },
    error::Error,
    moves::MoveKind,
    sticker::{Corner, Edge},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    fn exec(self) -> Result<(), Error> {
        if let Some(Command::Search {
            corners,
            edges,
            gen,
            depth,
        }) = self.command
        {
            let start = Instant::now();
            let commutators = match (corners, edges) {
                (Some(corners), None) => search_corner_commutators(corners, gen, depth)?,
                (None, Some(edges)) => search_edge_commutators(edges, gen, depth)?,
                _ => unreachable!(),
            };
            let end = Instant::now();

            print_commutators(commutators, end - start);
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Search commutators for the given three cycle")]
    #[clap(group(
    clap::ArgGroup::new("piece")
        .required(true)
        .args(&["corners", "edges"]),
    ))]
    Search {
        #[arg(long, short, num_args(3))]
        corners: Option<Vec<String>>,

        #[arg(long, short, num_args(3))]
        edges: Option<Vec<String>>,

        #[arg(long, short)]
        gen: String,

        #[arg(long, short)]
        depth: u8,
    },
}

fn search_corner_commutators(
    corners: Vec<String>,
    gen: String,
    depth: u8,
) -> Result<Vec<Commutator>, Error> {
    let corners = corners
        .into_iter()
        .map(|c| Corner::from_str(&c))
        .collect::<Result<Vec<_>, _>>()?;
    let cycle = Cycle::new(corners[0], corners[1], corners[2]);
    let allowed_moves = gen
        .chars()
        .map(|c| MoveKind::from_str(&c.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let results = find_corner_commutators(cycle, &allowed_moves, depth);

    Ok(results)
}

fn search_edge_commutators(
    edges: Vec<String>,
    gen: String,
    depth: u8,
) -> Result<Vec<Commutator>, Error> {
    let edges = edges
        .into_iter()
        .map(|c| Edge::from_str(&c))
        .collect::<Result<Vec<_>, _>>()?;
    let cycle = Cycle::new(edges[0], edges[1], edges[2]);
    let allowed_moves = gen
        .chars()
        .map(|c| MoveKind::from_str(&c.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let results = find_edge_commutators(cycle, &allowed_moves, depth);

    Ok(results)
}

fn print_commutators(commutators: Vec<Commutator>, duration: Duration) {
    let count = commutators.len();
    let duration = duration.as_secs_f32();
    let green = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));

    for comm in commutators {
        let bold = Style::new().bold();
        let cyan = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));

        println!(
            "{bold}{}{bold:#}: {} {cyan}({}){cyan:#}",
            comm,
            comm.expand(),
            comm.expand().len()
        );
    }

    if count > 0 {
        println!(
            "\nFound {green}{count}{green:#} result{} in {duration:.2}s.",
            if count > 1 { "s" } else { "" }
        );
    } else {
        println!("No result found.");
    }
}

fn print_error(error: String) {
    let style = Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)));

    eprintln!("{style}error{style:#}: {error}");
}

fn main() {
    let cli = Cli::parse();
    let result = cli.exec();

    if let Err(error) = result {
        print_error(error.to_string());
        process::exit(1);
    }
}
