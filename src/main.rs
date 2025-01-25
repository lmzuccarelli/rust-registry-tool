use crate::api::schema::*;
use clap::Parser;
use custom_logger::*;
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
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    // setup logging
    let log = &Logging {
        log_level: res_log_level,
    };

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
                log,
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
                log.info(&format!("{}", image));
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
            let re = Regex::new(r"v4\.[0-9]{2}\.0").unwrap();
            if !re.is_match(version) {
                log.error("version must respect the regex expression 'v4.[0-9]{2}.0'");
                process::exit(1);
            }
            let t_impl = ImplTokenInterface {};
            let token = get_token(
                t_impl,
                log,
                registry.to_string(),
                format!("{}/{}", namespace, name),
                !no_tls_verify,
            )
            .await;
            let i_query = ImplQueryImageInterface {};
            let mut url = format!(
                "https://{}/v2/{}/{}/tags/list?n=100&last={}",
                registry.clone(),
                namespace.clone(),
                name.clone(),
                version.clone(),
            );
            if token.is_err() {
                log.error(&format!(
                    "token {}",
                    token.as_ref().err().unwrap().to_string().to_lowercase()
                ));
                process::exit(1);
            }
            let mut vec_tags: Vec<Tags> = Vec::new();
            let mut query = "query".to_string();
            let mut query_dump = "".to_string();
            log.info(&format!("querying {} ", registry.clone()));
            while query.len() > 0 && !query.contains(&version[..5]) {
                let res = i_query
                    .get_details(url.clone(), token.as_ref().unwrap().to_string(), false)
                    .await;
                if res.is_err() {
                    log.error(&format!(
                        "response {}",
                        res.as_ref().err().unwrap().to_string().to_lowercase()
                    ));
                    process::exit(1);
                }
                let rd = res.unwrap();
                query = rd.link;
                log.trace(&format!("query link {}", query.clone()));
                url = format!("https://{}{}", registry.clone(), query);
                query_dump.push_str(&format!("{}\n", query));
                let res_json = serde_json::from_str(&rd.data);
                let root: Tags = res_json.unwrap();
                vec_tags.insert(0, root.clone());
            }

            let json_res = serde_json::to_string_pretty(&vec_tags);
            if json_res.is_err() {
                log.error(&format!(
                    "parsing json {}",
                    json_res.as_ref().err().unwrap().to_string().to_lowercase()
                ));
            }
            if *persist {
                fs::write(format!("results/{}.json", name.clone()), json_res.unwrap())
                    .expect("should write json formatted results");
                fs::write("results/links.txt".to_string(), query_dump)
                    .expect("should write links list");
                log.info(&format!(
                    "file results/{}.json created successfully",
                    name.clone()
                ));
            } else {
                log.info(&format!("results {}", json_res.unwrap()));
            }
        }
        Some(Commands::ListTagsByUrl {
            registry,
            url,
            no_tls_verify,
        }) => {
            let t_impl = ImplTokenInterface {};
            let img_ref: Vec<&str> = url.split("/").collect::<Vec<&str>>();
            log.debug(&format!("{:?}", img_ref));
            let token = get_token(
                t_impl,
                log,
                registry.to_string(),
                format!("{}/{}", img_ref[2].to_string(), img_ref[3].to_string()),
                !no_tls_verify,
            )
            .await;
            let i_query = ImplQueryImageInterface {};
            if token.is_err() {
                log.error(&format!(
                    "token {}",
                    token.as_ref().err().unwrap().to_string().to_lowercase()
                ));
                process::exit(1);
            }
            let get_url = format!("https://{}{}", registry, url);
            let res = i_query
                .get_details(get_url, token.as_ref().unwrap().to_string(), false)
                .await;
            if res.is_err() {
                log.error(&format!(
                    "response {}",
                    res.as_ref().err().unwrap().to_string().to_lowercase()
                ));
                process::exit(1);
            }
            log.info(&format!("{}", res.unwrap().data));
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
                log,
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
            log.info(&format!("etag digest {}", rd.data));
        }
        Some(Commands::Copy {
            from,
            to,
            no_tls_verify,
        }) => {
            log.info(&format!("{} {} {}", from, to, no_tls_verify));
            todo!()
        }

        None => {
            log.error("sub command not recognized");
            process::exit(1);
        }
    }
}
