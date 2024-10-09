use scraper::{Html, Selector};
use std::{error::Error, path::Path};

use reqwest::{Client, Response, Url};
use tokio::{
    fs::{self},
    task::JoinSet,
};

struct WGet {
    _client: Client,
}
impl WGet {
    pub fn new() -> Self {
        Self {
            _client: Client::new(),
        }
    }

    async fn save_resource(
        client: Client,
        url: Url,
        folder: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let response = Self::fetch_resourse(client, url.clone()).await?;
        Self::save_file(url.path(), response, folder).await?;

        Ok(())
    }

    async fn save_file(
        path: &str,
        content: Response,
        folder: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let filename = format!("{}/{}", folder, path.trim_start_matches('/'));

        if let Some(parent) = Path::new(&filename).parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(&filename, &content.bytes().await?).await?;
        println!("Saved resource: {}", filename);

        Ok(())
    }

    async fn fetch_resourse(client: Client, url: Url) -> Result<Response, reqwest::Error> {
        let response = client.get(url).send().await?;
        Ok(response)
    }

    pub async fn save_page(
        &self,
        url: &str,
        folder: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let url = Url::parse(url).expect("Failed to parse url");
        let folder_string = folder.to_string();
        let site_page_resp = Self::fetch_resourse(self._client.clone(), url.clone())
            .await?
            .text()
            .await?;

        let html_site_content = Html::parse_fragment(&site_page_resp);
        let file_name = format!("{}/index.html", folder);
        let selector = Selector::parse("a, img, link, script").unwrap();

        fs::create_dir_all(folder).await?;
        fs::write(&file_name, &site_page_resp).await?;

        let mut tasks: JoinSet<Result<(), Box<dyn std::error::Error + Send + Sync>>> =
            JoinSet::new();
        for element in html_site_content.select(&selector) {
            if let Some(href) = element
                .value()
                .attr("href")
                .or_else(|| element.value().attr("src"))
            {
                if let Ok(resource_url) = url.clone().join(href) {
                    tasks.spawn(Self::save_resource(
                        self._client.clone(),
                        resource_url.clone(),
                        folder_string.clone(),
                    ));
                }
            }
        }
        while let Some(fetch_result) = tasks.join_next().await {
            match fetch_result {
                Ok(_) => {}
                Err(e) => eprintln!("Error fetching page: {}", e),
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://github.com/rust-lang/rust/issues/115188";
    let wget = WGet::new();

    let _ = wget.save_page(url, "site").await;

    Ok(())
}
