

pub async fn fetch_json_or_yaml<T: serde::de::DeserializeOwned + Send>(
    registry: &url::Url,
    on_unexpected_response: &str,
) -> anyhow::Result<T> {
    match fetch_json_yaml_or_ok_response(registry, on_unexpected_response).await {
        Ok(v) => Ok(v),
        Err(e) => match e {
            either::Either::Left(err) => Err(err),
            either::Either::Right(_) => {
                anyhow::bail!("{on_unexpected_response}")
            }
        },
    }
}

pub async fn fetch_json_yaml_or_ok_response<T: serde::de::DeserializeOwned + Send>(
    registry: &url::Url,
    on_unexpected_response: &str,
) -> Result<T, either::Either<anyhow::Error, reqwest::Response>> {
    let client = reqwest::Client::new();
    let res = client
        .get(registry.clone())
        .header(
            "User-Agent",
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await
        .map_err(|e| either::Either::Left(e.into()))?
        .error_for_status()
        .map_err(|e| either::Either::Left(e.into()))?;
    let content_type = res
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if content_type.starts_with("application/json") || content_type.starts_with("text/json") {
        let text = res
            .text()
            .await
            .map_err(|e| either::Either::Left(e.into()))?;
        let parsed: T = serde_json::from_str(&text).map_err(|e| either::Either::Left(e.into()))?;
        return Ok(parsed);
    }
    if content_type.starts_with("application/yaml") || content_type.starts_with("text/yaml") {
        let text = res
            .text()
            .await
            .map_err(|e| either::Either::Left(e.into()))?;
        let parsed: T = serde_yml::from_str(&text).map_err(|e| either::Either::Left(e.into()))?;
        return Ok(parsed);
    }
    if content_type.starts_with("text/plain") {
        let text = res
            .text()
            .await
            .map_err(|e| either::Either::Left(e.into()))?;
        let parsed = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .or_else(|| serde_yml::from_str::<serde_json::Value>(&text).ok());
        if let Some(v) = parsed {
            return serde_json::from_value(v).map_err(|e| either::Either::Left(e.into()));
        }
        return Err(either::Either::Left(anyhow::anyhow!(
            "{on_unexpected_response}"
        )));
    }
    Err(either::Either::Right(res))
}

pub async fn fetch_may_follow_url<T: serde::de::DeserializeOwned + Send>(
    registry: &url::Url,
    on_unexpected_response: &str,
) -> anyhow::Result<T> {
    let response = match fetch_json_yaml_or_ok_response::<T>(registry, on_unexpected_response).await
    {
        Ok(v) => return Ok(v),
        Err(e) => match e {
            either::Either::Left(err) => return Err(err),
            either::Either::Right(res) => res,
        },
    };
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !content_type.starts_with("text/html") {
        anyhow::bail!("{on_unexpected_response}");
    }

    let html = response.text().await?;
    let url = 'url: {
        let document = scraper::Html::parse_document(&html);

        let link_alternate = document
            .select(&scraper::Selector::parse(r#"html > head > link[rel="alternate"][type="application/yaml+aviutl2-extension-composer"]"#).unwrap())
            .filter_map(|el| {
                el.value()
                    .attr("href")
                    .and_then(|href| registry.join(href).ok())
            })
            .next();
        if link_alternate.is_some() {
            break 'url link_alternate;
        }

        let selector = scraper::Selector::parse("pre, code").unwrap();
        let pre_code = document.select(&selector);

        let pattern = lazy_regex::lazy_regex!(
            r"\s*aviutl2-extension-composer:alternate:(?<url>https?:\/\/[^\s]+)\s*"
        );
        for el in pre_code {
            let text = el.text().collect::<Vec<_>>().join("");
            if let Some(url) = pattern
                .captures(&text)
                .and_then(|caps| caps.name("url"))
                .and_then(|m| m.as_str().parse::<url::Url>().ok())
            {
                break 'url Some(url);
            }
        }

        None
    };
    if let Some(url) = url {
        return fetch_json_or_yaml(&url, on_unexpected_response).await;
    }

    anyhow::bail!("{on_unexpected_response}");
}
