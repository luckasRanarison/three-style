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
            raw,
        }) = self.command
        {
            let allowed_moves = gen
                .chars()
                .map(|c| MoveKind::from_str(&c.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            let start = Instant::now();
            let commutators = match (corners, edges) {
                (Some(corners), None) => search_corner_commutators(corners, allowed_moves, depth)?,
                (None, Some(edges)) => search_edge_commutators(edges, allowed_moves, depth)?,
                _ => unreachable!(),
            };
            let end = Instant::now();

            print_commutators(commutators, end - start, raw);
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
        #[arg(long, short, num_args(3), help = "Corner cycle")]
        corners: Option<Vec<String>>,

        #[arg(long, short, num_args(3), help = "Edge cycle")]
        edges: Option<Vec<String>>,

        #[arg(long, short, help = "Allwed movesets")]
        gen: String,

        #[arg(long, short, help = "Maximum search depth")]
        depth: u8,

        #[arg(long, short, help = "Display the non-reduced algorithm")]
        raw: bool,
    },
}

fn search_corner_commutators(
    corners: Vec<String>,
    allowed_moves: Vec<MoveKind>,
    depth: u8,
) -> Result<Vec<Commutator>, Error> {
    let corners = corners
        .into_iter()
        .map(|c| Corner::from_str(&c))
        .collect::<Result<Vec<_>, _>>()?;
    let cycle = Cycle::new(corners[0], corners[1], corners[2]);
    let results = find_corner_commutators(cycle, &allowed_moves, depth);

    Ok(results)
}

fn search_edge_commutators(
    edges: Vec<String>,
    allowed_moves: Vec<MoveKind>,
    depth: u8,
) -> Result<Vec<Commutator>, Error> {
    let edges = edges
        .into_iter()
        .map(|c| Edge::from_str(&c))
        .collect::<Result<Vec<_>, _>>()?;
    let cycle = Cycle::new(edges[0], edges[1], edges[2]);
    let results = find_edge_commutators(cycle, &allowed_moves, depth);

    Ok(results)
}

fn print_commutators(commutators: Vec<Commutator>, duration: Duration, raw: bool) {
    let count = commutators.len();
    let duration = duration.as_secs_f32();
    let green = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));

    for comm in commutators {
        let bold = Style::new().bold();
        let cyan = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
        let alg = comm.expand();
        let alg = if raw { alg } else { alg.reduce() };

        println!("{bold}{comm}{bold:#}: {alg} {cyan}({}){cyan:#}", alg.len());
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

fn print_error(error: Error) {
    let style = Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)));

    eprintln!("{style}error{style:#}: {error}");
}

fn main() {
    let cli = Cli::parse();
    let result = cli.exec();

    if let Err(error) = result {
        print_error(error);
        process::exit(1);
    }
}
