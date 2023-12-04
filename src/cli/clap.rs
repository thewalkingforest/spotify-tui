use std::path::PathBuf;

use clap::{builder::ArgPredicate, Arg, ArgGroup, Args, Command, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
  /// Specify configuration file path
  #[arg(short, long, value_name = "config")]
  config: Option<PathBuf>,

  /// Generates completions for your preferred shell [possible values: bash, zsh, fish, power-shell, elvish]
  #[arg(long)]
  completions: String,

  /// Specify the tick rate in milliseconds: the lower the number the higher the FPS. It can be nicer to have a
  /// lower value when you want to use the audio analysis view of the app. Beware that this comes at a CPU cost!
  #[arg(short, long, name = "tick-rate", value_name = "tick-rate")]
  tick_rate: usize,

  #[command(subcommand)]
  commands: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
  List(List),
  Play(Play),
  Playback(Playback),
  Search(Search),
}

#[derive(Debug, Args)]
pub struct List {
  /// Lists devices
  #[arg(short, long)]
  devices: bool,

  /// Lists liked songs
  liked: bool,

  /// Lists playlists
  #[arg(short, long)]
  playlists: bool,

  /// There are multiple format specifiers you can use: %a: artist, %b: album, %p: playlist, %t: track, %h: show,
  /// %f: flags (shuffle, repeat, like), %s: playback status, %v: volume, %d: current device. Example: spt pb -s
  /// -f 'playing on %d at %v%'
  #[arg(short, long, value_name = "FORMAT")]
  format: String,

  /// Specifies the maximum number of results (1 - 50)
  limit: i32,
}

#[derive(Debug, Args)]
pub struct Play {
  #[arg(short, long, value_name = "URI")]
  uri: String,

  #[arg(short, long, value_name = "NAME")]
  name: String,

  #[arg(short, long)]
  queue: bool,

  #[arg(short, long)]
  random: bool,

  #[arg(short = 'b', long)]
  album: bool,

  #[arg(short = 'a', long)]
  artist: bool,

  #[arg(short = 't', long)]
  track: bool,

  #[arg(short = 'w', long)]
  show: bool,

  #[arg(short = 'p', long)]
  playlist: bool,

  contexts: String,

  actions: String,
}

#[derive(Debug, Args)]
pub struct Playback {}

#[derive(Debug, Args)]
pub struct Search {}

fn device_arg() -> Arg {
  Arg::new("device")
    .short('d')
    .long("device")
    // .takes_value(true)
    .value_name("DEVICE")
    .help("Specifies the spotify device to use")
}

fn format_arg() -> Arg {
  Arg::new("format")
    .short('f')
    .long("format")
    // .takes_value(true)
    .value_name("FORMAT")
    .help("Specifies the output format")
    .long_help(
      "There are multiple format specifiers you can use: %a: artist, %b: album, %p: playlist, \
%t: track, %h: show, %f: flags (shuffle, repeat, like), %s: playback status, %v: volume, %d: current device. \
Example: spt pb -s -f 'playing on %d at %v%'",
    )
}

pub fn playback_subcommand() -> Command {
  Command::new("playback")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Interacts with the playback of a device")
    .long_about(
      "Use `playback` to interact with the playback of the current or any other device. \
You can specify another device with `--device`. If no options were provided, spt \
will default to just displaying the current playback. Actually, after every action \
spt will display the updated playback. The output format is configurable with the \
`--format` flag. Some options can be used together, other options have to be alone.

Here's a list:

* `--next` and `--previous` cannot be used with other options
* `--status`, `--toggle`, `--transfer`, `--volume`, `--like`, `--repeat` and `--shuffle` \
can be used together
* `--share-track` and `--share-album` cannot be used with other options",
    )
    .visible_alias("pb")
    .arg(device_arg())
    .arg(
      format_arg()
        .default_value("%f %s %t - %a")
        .default_value_ifs([
          ("seek", ArgPredicate::IsPresent, Some("%f %s %t - %a %r")),
          ("volume", ArgPredicate::IsPresent, Some("%v% %f %s %t - %a")),
          (
            "transfer",
            ArgPredicate::IsPresent,
            Some("%f %s %t - %a on %d"),
          ),
        ]),
    )
    .arg(
      Arg::new("toggle")
        .short('t')
        .long("toggle")
        .num_args(0)
        .help("Pauses/resumes the playback of a device"),
    )
    .arg(
      Arg::new("status")
        .short('s')
        .long("status")
        .num_args(0)
        .help("Prints out the current status of a device (default)"),
    )
    .arg(
      Arg::new("share-track")
        .long("share-track")
        .num_args(0)
        .help("Returns the url to the current track"),
    )
    .arg(
      Arg::new("share-album")
        .long("share-album")
        .num_args(0)
        .help("Returns the url to the album of the current track"),
    )
    .arg(
      Arg::new("transfer")
        .long("transfer")
        .num_args(1)
        .value_name("DEVICE")
        .help("Transfers the playback to new DEVICE"),
    )
    .arg(
      Arg::new("like")
        .long("like")
        .num_args(0)
        .help("Likes the current song if possible"),
    )
    .arg(
      Arg::new("dislike")
        .long("dislike")
        .num_args(0)
        .help("Dislikes the current song if possible"),
    )
    .arg(
      Arg::new("shuffle")
        .long("shuffle")
        .num_args(0)
        .help("Toggles shuffle mode"),
    )
    .arg(
      Arg::new("repeat")
        .long("repeat")
        .num_args(0)
        .help("Switches between repeat modes"),
    )
    .arg(
      Arg::new("next")
        .short('n')
        .long("next")
        // .multiple(true)
        .num_args(0)
        .help("Jumps to the next song")
        .long_help(
          "This jumps to the next song if specied once. If you want to jump, let's say 3 songs \
forward, you can use `--next` 3 times: `spt pb -nnn`.",
        ),
    )
    .arg(
      Arg::new("previous")
        .short('p')
        .long("previous")
        // .multiple(true)
        .help("Jumps to the previous song")
        .long_help(
          "This jumps to the beginning of the current song if specied once. You probably want to \
jump to the previous song though, so you can use the previous flag twice: `spt pb -pp`. To jump \
two songs back, you can use `spt pb -ppp` and so on.",
        ),
    )
    .arg(
      Arg::new("seek")
        .long("seek")
        // .takes_value(true)
        .value_name("±SECONDS")
        .allow_hyphen_values(true)
        .help("Jumps SECONDS forwards (+) or backwards (-)")
        .long_help(
          "For example: `spt pb --seek +10` jumps ten second forwards, `spt pb --seek -10` ten \
seconds backwards and `spt pb --seek 10` to the tenth second of the track.",
        ),
    )
    .arg(
      Arg::new("volume")
        .short('v')
        .long("volume")
        // .takes_value(true)
        .value_name("VOLUME")
        .help("Sets the volume of a device to VOLUME (1 - 100)"),
    )
    .group(
      ArgGroup::new("jumps")
        .args(&["next", "previous"])
        .multiple(false)
        .conflicts_with_all(&["single", "flags", "actions"]),
    )
    .group(
      ArgGroup::new("likes")
        .args(&["like", "dislike"])
        .multiple(false),
    )
    .group(
      ArgGroup::new("flags")
        .args(&["like", "dislike", "shuffle", "repeat"])
        .multiple(true)
        .conflicts_with_all(&["single", "jumps"]),
    )
    .group(
      ArgGroup::new("actions")
        .args(&["toggle", "status", "transfer", "volume"])
        .multiple(true)
        .conflicts_with_all(&["single", "jumps"]),
    )
    .group(
      ArgGroup::new("single")
        .args(&["share-track", "share-album"])
        .multiple(false)
        .conflicts_with_all(&["actions", "flags", "jumps"]),
    )
}

pub fn play_subcommand() -> Command {
  Command::new("play")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Plays a uri or another spotify item by name")
    .long_about(
      "If you specify a uri, the type can be inferred. If you want to play something by \
name, you have to specify the type: `--track`, `--album`, `--artist`, `--playlist` \
or `--show`. The first item which was found will be played without confirmation. \
To add a track to the queue, use `--queue`. To play a random song from a playlist, \
use `--random`. Again, with `--format` you can specify how the output will look. \
The same function as found in `playback` will be called.",
    )
    .visible_alias("p")
    .arg(device_arg())
    .arg(format_arg().default_value("%f %s %t - %a"))
    .arg(
      Arg::new("uri")
        .short('u')
        .long("uri")
        // .takes_value(true)
        .num_args(1)
        .value_name("URI")
        .help("Plays the URI"),
    )
    .arg(
      Arg::new("name")
        .short('n')
        .long("name")
        // .takes_value(true)
        .num_args(1)
        .value_name("NAME")
        .requires("contexts")
        .help("Plays the first match with NAME from the specified category"),
    )
    .arg(
      Arg::new("queue")
        .short('q')
        .long("queue")
        // Only works with tracks
        .conflicts_with_all(["album", "artist", "playlist", "show"])
        .help("Adds track to queue instead of playing it directly"),
    )
    .arg(
      Arg::new("random")
        .short('r')
        .long("random")
        // Only works with playlists
        .conflicts_with_all(["track", "album", "artist", "show"])
        .help("Plays a random track (only works with playlists)"),
    )
    .arg(
      Arg::new("album")
        .short('b')
        .long("album")
        .help("Looks for an album"),
    )
    .arg(
      Arg::new("artist")
        .short('a')
        .long("artist")
        .help("Looks for an artist"),
    )
    .arg(
      Arg::new("track")
        .short('t')
        .long("track")
        .help("Looks for a track"),
    )
    .arg(
      Arg::new("show")
        .short('w')
        .long("show")
        .help("Looks for a show"),
    )
    .arg(
      Arg::new("playlist")
        .short('p')
        .long("playlist")
        .help("Looks for a playlist"),
    )
    .group(
      ArgGroup::new("contexts")
        .args(["track", "artist", "playlist", "album", "show"])
        .multiple(false),
    )
    .group(
      ArgGroup::new("actions")
        .args(&["uri", "name"])
        .multiple(false)
        .required(true),
    )
}

pub fn list_subcommand() -> Command {
  Command::new("list")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Lists devices, liked songs and playlists")
    .long_about(
      "This will list devices, liked songs or playlists. With the `--limit` flag you are \
able to specify the amount of results (between 1 and 50). Here, the `--format` is \
even more awesome, get your output exactly the way you want. The format option will \
be applied to every item found.",
    )
    .visible_alias("l")
    .arg(format_arg().default_value_ifs([
      ("devices", ArgPredicate::IsPresent, Some("%v% %d")),
      ("liked", ArgPredicate::IsPresent, Some("%t - %a (%u)")),
      ("playlists", ArgPredicate::IsPresent, Some("%p (%u)")),
    ]))
    .arg(
      Arg::new("devices")
        .short('d')
        .long("devices")
        .help("Lists devices"),
    )
    .arg(
      Arg::new("playlists")
        .short('p')
        .long("playlists")
        .help("Lists playlists"),
    )
    .arg(Arg::new("liked").long("liked").help("Lists liked songs"))
    .arg(
      Arg::new("limit")
        .long("limit")
        // .takes_value(true)
        .help("Specifies the maximum number of results (1 - 50)"),
    )
    .group(
      ArgGroup::new("listable")
        .args(&["devices", "playlists", "liked"])
        .required(true)
        .multiple(false),
    )
}

pub fn search_subcommand() -> Command {
  Command::new("search")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about("Searches for tracks, albums and more")
    .long_about(
      "This will search for something on spotify and displays you the items. The output \
format can be changed with the `--format` flag and the limit can be changed with \
the `--limit` flag (between 1 and 50). The type can't be inferred, so you have to \
specify it.",
    )
    .visible_alias("s")
    .arg(format_arg().default_value_ifs([
      ("tracks", ArgPredicate::IsPresent, Some("%t - %a (%u)")),
      ("playlists", ArgPredicate::IsPresent, Some("%p (%u)")),
      ("artists", ArgPredicate::IsPresent, Some("%a (%u)")),
      ("albums", ArgPredicate::IsPresent, Some("%b - %a (%u)")),
      ("shows", ArgPredicate::IsPresent, Some("%h - %a (%u)")),
    ]))
    .arg(
      Arg::new("search")
        .required(true)
        // .takes_value(true)
        .value_name("SEARCH")
        .help("Specifies the search query"),
    )
    .arg(
      Arg::new("albums")
        .short('b')
        .long("albums")
        .help("Looks for albums"),
    )
    .arg(
      Arg::new("artists")
        .short('a')
        .long("artists")
        .help("Looks for artists"),
    )
    .arg(
      Arg::new("playlists")
        .short('p')
        .long("playlists")
        .help("Looks for playlists"),
    )
    .arg(
      Arg::new("tracks")
        .short('t')
        .long("tracks")
        .help("Looks for tracks"),
    )
    .arg(
      Arg::new("shows")
        .short('w')
        .long("shows")
        .help("Looks for shows"),
    )
    .arg(
      Arg::new("limit")
        .long("limit")
        // .takes_value(true)
        .help("Specifies the maximum number of results (1 - 50)"),
    )
    .group(
      ArgGroup::new("searchable")
        .args(&["playlists", "tracks", "albums", "artists", "shows"])
        .required(true)
        .multiple(false),
    )
}
