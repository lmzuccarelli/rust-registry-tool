use crate::api::schema::*;
use clap::Parser;
use custom_logger as log;
use mirror_auth::{get_token, ImplTokenInterface};
use mirror_query::*;
use regex::Regex;
use std::{fs, process};
use tokio;

mod api;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let level = args.loglevel.unwrap().to_string();

    // convert to enum
    let res_log_level = match level.as_str() {
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    // setup logging
    log::Logging::new()
        .with_level(res_log_level)
        .init()
        .expect("should initialize");
    fs::create_dir_all("results").expect("should create results directory");

    match &args.command {
        Some(Commands::ListCatalog {
            registry,
            namespace,
            no_tls_verify,
        }) => {
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                registry.to_string(),
                namespace.to_string(),
                !no_tls_verify,
            )
            .await;
            let i_query = ImplQueryImageInterface {};
            let mut url = format!("https://{}/v2/_catalog", registry);
            if *no_tls_verify {
                url = url.replace("https", "http");
            }
            let res = i_query.get_details(url, token.unwrap(), false).await;
            let rd = res.unwrap();
            let res_json = serde_json::from_str(&rd.data);
            let root: Catalogs = res_json.unwrap();
            for image in root.repositories.iter() {
                log::info!("{}", image);
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
            let re = Regex::new(r"4\.[0-9]{2}\.0").unwrap();
            if !re.is_match(version) {
                log::error!("format must respect the pattern -> '4.XX.0' where XX > 10");
                process::exit(1);
            }
            let cleaned_version = match name.as_str() {
                "ocp-release" => {
                    format!("{}", &version[..4])
                }
                _ => format!("v{}", &version[..4]),
            };
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                registry.to_string(),
                format!("{}/{}", namespace, name),
                !no_tls_verify,
            )
            .await;
            let i_query = ImplQueryImageInterface {};
            let mut url = format!(
                "https://{}/v2/{}/{}/tags/list?n=200&last={}",
                registry.clone(),
                namespace.clone(),
                name.clone(),
                cleaned_version.clone(),
            );
            log::trace!("url {}", url);
            if token.is_err() {
                log::error!(
                    "token {}",
                    token.as_ref().err().unwrap().to_string().to_lowercase()
                );
                process::exit(1);
            }
            let mut vec_tags: Vec<Tags> = Vec::new();
            let mut query = cleaned_version.to_string();
            let mut query_dump = "".to_string();
            log::info!("querying {} ", registry.clone());

            while query.len() > 0 && query.contains(&cleaned_version) {
                let res = i_query
                    .get_details(url.clone(), token.as_ref().unwrap().to_string(), false)
                    .await;
                if res.is_err() {
                    log::error!(
                        "response {}",
                        res.as_ref().err().unwrap().to_string().to_lowercase()
                    );
                    process::exit(1);
                }
                let rd = res.unwrap();
                query = rd.link;
                log::trace!("query link {}", query.clone());
                if name == "ocp-release" && !query.contains(&cleaned_version) {
                    break;
                }
                url = format!("https://{}{}", registry.clone(), query);
                query_dump.push_str(&format!("{}\n", query));
                let res_json = serde_json::from_str(&rd.data);
                let root: Tags = res_json.unwrap();
                vec_tags.insert(0, root.clone());
            }

            let json_res = serde_json::to_string_pretty(&vec_tags);
            if json_res.is_err() {
                log::error!(
                    "parsing json {}",
                    json_res.as_ref().err().unwrap().to_string().to_lowercase()
                );
            }
            if *persist {
                fs::write(format!("results/{}.json", name.clone()), json_res.unwrap())
                    .expect("should write json formatted results");
                fs::write("results/links.txt".to_string(), query_dump)
                    .expect("should write links list");
                log::info!("file results/{}.json created successfully", name.clone());
            } else {
                log::info!("results {}", json_res.unwrap());
            }
        }
        Some(Commands::ListTagsByUrl {
            registry,
            url,
            no_tls_verify,
        }) => {
            let t_impl = ImplTokenInterface {};
            let img_ref: Vec<&str> = url.split("/").collect::<Vec<&str>>();
            log::debug!("{:?}", img_ref);
            let token = get_token(
                t_impl,
                registry.to_string(),
                format!("{}/{}", img_ref[2].to_string(), img_ref[3].to_string()),
                !no_tls_verify,
            )
            .await;
            let i_query = ImplQueryImageInterface {};
            if token.is_err() {
                log::error!(
                    "token {}",
                    token.as_ref().err().unwrap().to_string().to_lowercase()
                );
                process::exit(1);
            }
            let get_url = format!("https://{}{}", registry, url);
            let res = i_query
                .get_details(get_url, token.as_ref().unwrap().to_string(), false)
                .await;
            if res.is_err() {
                log::error!(
                    "response {}",
                    res.as_ref().err().unwrap().to_string().to_lowercase()
                );
                process::exit(1);
            }
            log::info!("{}", res.unwrap().data);
        }
        Some(Commands::Digest {
            registry,
            namespace,
            name,
            tag,
            no_tls_verify,
        }) => {
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                registry.to_string(),
                format!("{}/{}", namespace, name),
                !no_tls_verify,
            )
            .await;
            let i_query = ImplQueryImageInterface {};
            let mut url = format!(
                "https://{}/v2/{}/{}/manifests/{}",
                registry,
                namespace.clone(),
                name.clone(),
                tag.clone(),
            );
            if *no_tls_verify {
                url = url.replace("https", "http");
            }
            let res = i_query.get_details(url, token.unwrap(), true).await;
            let rd = res.unwrap();
            log::info!("etag digest {}", rd.data);
        }
        Some(Commands::Copy {
            from,
            to,
            no_tls_verify,
        }) => {
            log::info!("{} {} {}", from, to, no_tls_verify);
            todo!()
        }

        None => {
            log::error!("sub command not recognized");
            process::exit(1);
        }
    }
}
