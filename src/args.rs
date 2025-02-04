use clap::{value_parser, Arg, Command};

#[derive(Debug)]
pub struct Args {
    pub node: String,
    pub host: String,
    pub port: usize
}

#[derive(Default)]
struct ArgsBuilder {
    node: String,
    host: String,
    port: usize
}

impl Args {
    fn builder() -> ArgsBuilder {
        ArgsBuilder::default()
    }
}

impl ArgsBuilder {
    pub fn node(&mut self, node: String) -> &mut Self {
        self.node = node;
        self
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        self.port = port;
        self
    }

    pub fn build(self) -> Args {
        let node = self.node;
        let host = self.host;
        let port = self.port;

        Args { node, host, port }
    }
}

pub fn get_args() -> Args {
    let matches = build_command().get_matches();
    let mut builder = Args::builder();

    if let Some(val) = matches.get_one::<String>("node") {
        builder.node((*val).clone());
    } else {
        panic!("Missing node name.");
    }

    if let Some(val) = matches.get_one::<String>("host") {
        builder.host((*val).clone());
    } else {
        panic!("Missing host.");
    }

    if let Some(val) = matches.get_one("port") {
        builder.port(*val);
    } else {
        panic!("Missing port.");
    }

    builder.build()
}

fn build_command() -> Command {
    Command::new("stress")
        .author("Bayram, bkulyev@gmail.com")
        .version("0.0.1")
        .about("runs cluster node.")
        .arg(
            Arg::new("node")
                .required(true)
                .value_parser(value_parser!(String))
                .long("node")
                .help("Node name)"))
        .arg(
            Arg::new("host")
                .required(true)
                .value_parser(value_parser!(String))
                .long("host")
                .help("Etcd server IP)"))
        .arg(
            Arg::new("port")
                .required(true)
                .value_parser(value_parser!(usize))
                .long("port")
                .help("Etcd server port)")
        )
}
