
use clap::{ Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
    Put(Put),
    Delete(Delete),
}

#[derive(Parser, Debug)]
pub struct Get {
    url: String,
}

#[derive(Parser, Debug)]
pub struct Post {
    url: String,
    body: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Put {
    url: String,
    body: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct Delete {
    url: String,
}

// impl FromArgMatches for SubCommand {
//     fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
//         match matches.subcommand() {
//             Some(("get", args)) => Ok(Self::Get(Get::from_arg_matches(args)?)),
//             Some(("post", args)) => Ok(Self::Post(Post::from_arg_matches(args)?)),
//             Some(("put", args)) => Ok(Self::Put(Put::from_arg_matches(args)?)),
//             Some(("delete", args)) => Ok(Self::Delete(Delete::from_arg_matches(args)?)),
//             Some((subcommand, _)) => Err(Error::raw(
//                 ErrorKind::InvalidSubcommand,
//                 &format!("Invalid subcommand: `{}`. Valid subcommands are `get`, `post`, `put`, and `delete`.", subcommand),
//             )),
//             None => Err(Error::raw(
//                 ErrorKind::MissingSubcommand,
//                 "A subcommand is required. Valid subcommands are `get`, `post`, `put`, and `delete`.",
//             )),
//         }
//     }

//     fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
//         match matches.subcommand() {
//             Some(("get", args)) => *self = Self::Get(Get::from_arg_matches(args)?),
//             Some(("post", args)) => *self = Self::Post(Post::from_arg_matches(args)?),
//             Some(("put", args)) => *self = Self::Put(Put::from_arg_matches(args)?),
//             Some(("delete", args)) => *self = Self::Delete(Delete::from_arg_matches(args)?),
//             Some((_, _)) => {
//                 return Err(Error::raw(
//                     ErrorKind::InvalidSubcommand,
//                     "Valid subcommands are `add` and `remove`",
//                 ))
//             }
//             None => (),
//         };
//         Ok(())
//     }
// }

// impl Subcommand for SubCommand {
//     fn augment_subcommands(cmd: Command) -> Command {
//         cmd.subcommand(Get::augment_args(Command::new("get").about("get")))
//             .subcommand(Post::augment_args(Command::new("post").about("Make a POST request")))
//             .subcommand(Put::augment_args(Command::new("put").about("Make a PUT request")))
//             .subcommand(Delete::augment_args(Command::new("delete").about("Make a DELETE request")))
//             .subcommand_required(true)
//     }

//     fn augment_subcommands_for_update(cmd: Command) -> Command {
//         Self::augment_subcommands(cmd)
//     }

//     fn has_subcommand(name: &str) -> bool {
//         matches!(name, "get" | "post" | "put" | "delete")
//     }
// }
