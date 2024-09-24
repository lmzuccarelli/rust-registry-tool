use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};

/// rust-registry-tool cli struct
#[derive(Parser)]
#[command(name = "rust-registry-tool")]
#[command(author = "Luigi Mario Zuccarelli <luzuccar@redhat.com>")]
#[command(version = "0.0.1")]
#[command(about = "A simple command line tool that can query and copy artifacts to an from an image registry", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// set the loglevel
    #[arg(
        value_enum,
        short,
        long,
        value_name = "loglevel",
        default_value = "info",
        help = "Set the log level [possible values: info, debug, trace]"
    )]
    pub loglevel: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// ListCatalog subcommand (lists all images in the registry)
    ListCatalog {
        #[arg(
            short,
            long,
            value_name = "registry",
            help = "The image registry to query (required)"
        )]
        registry: String,
        #[arg(
            short,
            long,
            value_name = "namespace/name",
            help = "The image namespace/name to query (required)"
        )]
        namespace: String,

        #[arg(short, long, value_name = "no-tls-verify", help = "disable tls-verify")]
        no_tls_verify: bool,
    },
    /// ListTags list all tags by image
    ListTags {
        #[arg(
            short,
            long,
            value_name = "registry",
            help = "The image registry to query (required)"
        )]
        registry: String,
        #[arg(
            short,
            long,
            value_name = "namespace",
            help = "The image namespace to query (required)"
        )]
        namespace: String,
        #[arg(
            short,
            long,
            value_name = "name",
            help = "The images name to query (required)"
        )]
        name: String,
        #[arg(
            short,
            long,
            value_name = "query-params",
            help = "The link query-param to append to url for pagination (required)"
        )]
        query_params: Option<String>,

        #[arg(short, long, value_name = "no-tls-verify", help = "disable tls-verify")]
        no_tls_verify: bool,
    },
    /// Digest - get a digest reference from an image manifest
    Digest {
        #[arg(
            short,
            long,
            value_name = "registry",
            help = "The image registry to query (required)"
        )]
        registry: String,
        #[arg(
            short,
            long,
            value_name = "namespace",
            help = "The image namespace to query (required)"
        )]
        namespace: String,
        #[arg(
            short,
            long,
            value_name = "name",
            help = "The images name to query (required)"
        )]
        name: String,
        #[arg(
            short,
            long,
            value_name = "tag",
            help = "The images tag  query (required)"
        )]
        tag: String,
        #[arg(
            short,
            long,
            value_name = "no-tls-verify",
            default_value = "false",
            help = "disable tls-verify"
        )]
        no_tls_verify: bool,
    },
    /// Copy
    Copy {
        #[arg(
            short,
            long,
            value_name = "from",
            help = "Copy from disk (oci or docker v2 format) or registry (required)"
        )]
        from: String,
        #[arg(
            short,
            long,
            value_name = "to",
            help = "Copy to disk (oci or docker v2 format) or registry (required)"
        )]
        to: String,
        #[arg(short, long, value_name = "no-tls-verify", help = "Disable tls-verify")]
        no_tls_verify: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Catalogs {
    pub repositories: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tags {
    pub name: String,
    pub tags: Vec<String>,
}
