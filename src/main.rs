use crate::api::schema::*;
use clap::Parser;
use custom_logger::*;
use mirror_auth::{get_token, ImplTokenInterface};
use mirror_query::*;
use std::process;
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
            let res_json = serde_json::from_str(&res.unwrap());
            let root: Catalogs = res_json.unwrap();
            for image in root.repositories.iter() {
                log.info(&format!("{}", image));
            }
        }
        Some(Commands::ListTags {
            registry,
            namespace,
            name,
            no_tls_verify,
            query_params,
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
                "https://{}/v2/{}/{}/tags/list",
                registry,
                namespace.clone(),
                name.clone()
            );
            if query_params.is_some() {
                url = format!(
                    "https://{}/{}",
                    registry.clone(),
                    query_params.clone().unwrap()
                );
            }
            if token.is_err() {
                log.error(&format!(
                    "token {}",
                    token.as_ref().err().unwrap().to_string().to_lowercase()
                ));
                process::exit(1);
            }
            let res = i_query.get_details(url, token.unwrap(), false).await;
            if res.is_err() {
                log.error(&format!(
                    "response {}",
                    res.as_ref().err().unwrap().to_string().to_lowercase()
                ));
                process::exit(1);
            }
            let res_json = serde_json::from_str(&res.unwrap());
            let root: Tags = res_json.unwrap();
            log.info(&format!("image {}", root.name));
            for tag in root.tags.iter() {
                log.info(&format!("tag   {}", tag));
            }
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
            log.info(&format!("etag digest {}", res.unwrap()));
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
