use crate::api::schema::*;
use clap::Parser;
use custom_logger as log;
use mirror_auth::{get_token, ImplTokenInterface};
use mirror_error::MirrorError;
use mirror_query::*;
use regex::Regex;
use std::{fs, str::FromStr};
use tokio;

mod api;

#[tokio::main]
async fn main() -> Result<(), MirrorError> {
    let args = Cli::parse();
    let level = args.loglevel.as_deref().unwrap_or("info");
    let res_log_level = log::LevelFilter::from_str(level)
        .map_err(|_| MirrorError::new(&format!("invalid log level \"{level}\"")))?;

    // setup logging
    log::Logging::new()
        .with_level(res_log_level)
        .init()
        .expect("should initialize");
    fs::create_dir_all("results").expect("should create results directory");

    match args.command {
        Some(Commands::ListCatalog {
            registry,
            namespace,
            no_tls_verify,
        }) => {
            let url = format!(
                "http{}://{registry}/v2/_catalog",
                if no_tls_verify { "" } else { "s" },
            );
            log::trace!("url={url}");
            let t_impl = ImplTokenInterface {};
            let token = get_token(t_impl, registry, namespace, !no_tls_verify).await?;
            let i_query = ImplQueryImageInterface {};
            let rd = i_query.get_details(url, token, false).await?;
            let root: Catalogs = serde_json::from_str(&rd.data)
                .map_err(|e| MirrorError::new(&format!("should unmarshall response data: {e}")))?;
            for image in root.repositories.iter() {
                log::info!("{image}");
            }
        }
        Some(Commands::ListTags {
            registry,
            namespace,
            name,
            version,
            no_tls_verify,
            persist,
        }) => {
            let re = Regex::new(r"4\.[0-9]{2}\.0").expect("regex must compile");
            if !re.is_match(&version) {
                return Err(MirrorError::new(
                    "format must respect the pattern -> '4.XX.0' where XX > 10",
                ));
            }
            let cleaned_version = match name.as_str() {
                "ocp-release" => version[..4].to_string(),
                _ => format!("v{}", &version[..4]),
            };
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                registry.clone(),
                format!("{}/{}", namespace, name),
                !no_tls_verify,
            )
            .await?;
            let i_query = ImplQueryImageInterface {};
            let mut url = format!(
                "http{}://{registry}/v2/{namespace}/{name}/tags/list?n=200&last={cleaned_version}",
                if no_tls_verify { "" } else { "s" },
            );
            log::trace!("url {url}");
            let mut vec_tags: Vec<Tags> = Vec::new();
            let mut query = cleaned_version.to_string();
            let mut query_dump = "".to_string();
            log::info!("querying {registry}");

            while !query.is_empty() && query.contains(&cleaned_version) {
                let rd = i_query.get_details(url, token.clone(), false).await?;
                query = rd.link;
                log::trace!("query link {query}");
                if name == "ocp-release" && !query.contains(&cleaned_version) {
                    break;
                }
                url = format!(
                    "http{}://{registry}{query}",
                    if no_tls_verify { "" } else { "s" }
                );
                query_dump.push_str(&format!("{query}\n"));
                let root: Tags = serde_json::from_str(&rd.data).map_err(|e| {
                    MirrorError::new(&format!("could not parse response data: {e}"))
                })?;
                vec_tags.insert(0, root);
            }

            let json = serde_json::to_string_pretty(&vec_tags)
                .map_err(|e| MirrorError::new(&format!("failed to marshal json: {e}")))?;
            if persist {
                fs::write(format!("results/{name}.json"), json)
                    .expect("should write json formatted results");
                fs::write("results/links.txt", query_dump).expect("should write links list");
                log::info!("file results/{name}.json created successfully");
            } else {
                log::info!("results {json}");
            }
        }
        Some(Commands::ListTagsByUrl {
            registry,
            url,
            no_tls_verify,
        }) => {
            let img_ref: Vec<&str> = url.split('/').collect();
            log::debug!("{:?}", img_ref);
            let get_url = format!(
                "http{}://{registry}{url}",
                if no_tls_verify { "" } else { "s" }
            );
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                registry,
                format!("{}/{}", img_ref[2], img_ref[3]),
                !no_tls_verify,
            )
            .await?;
            let i_query = ImplQueryImageInterface {};
            let res = i_query.get_details(get_url, token, false).await?;
            log::info!("{}", res.data);
        }
        Some(Commands::Digest {
            registry,
            namespace,
            name,
            tag,
            no_tls_verify,
        }) => {
            let url = format!(
                "http{}://{registry}/v2/{namespace}/{name}/manifests/{tag}",
                if no_tls_verify { "" } else { "s" },
            );
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                registry,
                format!("{}/{}", namespace, name),
                !no_tls_verify,
            )
            .await?;
            let i_query = ImplQueryImageInterface {};
            let rd = i_query.get_details(url, token, true).await?;
            log::info!("etag digest {}", rd.data);
        }
        Some(Commands::Copy {
            from,
            to,
            no_tls_verify,
        }) => {
            log::info!("{from} {to} {no_tls_verify}");
            todo!()
        }
        None => return Err(MirrorError::new("sub command not recognized")),
    }
    Ok(())
}
