use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use scraper::{Html, Selector};

use crate::models::domain::coingecko::RawTransaction;
use crate::{models::dto::coingecko::CoinDataResponse, utils::error::AppError};

const BASE_URL: &str = "https://api.coingecko.com/api/v3";
const API_HEADER: &str = "x-cg-demo-api-key";

#[derive(Clone)]
pub struct CoinGeckoClient {
    base_url: String,
    res_client: Client,
    headers: HeaderMap,
}

impl CoinGeckoClient {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(API_HEADER, HeaderValue::from_str(&api_key).unwrap());
        Self {
            base_url: BASE_URL.to_string(),
            res_client: Client::new(),
            headers,
        }
    }

    pub async fn get_coin_data(&self, coin_id: &str) -> Result<CoinDataResponse, AppError> {
        let response = self
            .res_client
            .get(format!("{}/coins/{}", self.base_url, coin_id))
            .headers(self.headers.clone())
            .send()
            .await?;
        let data = response.json::<CoinDataResponse>().await?;
        Ok(data)
    }

    pub fn parse_html_contents(&self, contents: &String) -> Result<Vec<RawTransaction>, AppError> {
        let doc = Html::parse_document(contents);
        let table_sel = Selector::parse(r"body > div.container > main > div:nth-child(3) > div:nth-child(3) > div.tw-overflow-x-auto.\32 lg\:tw-overflow-x-visible.\32 lg\:tw-flex.\32 lg\:tw-justify-center > table > tbody").unwrap();
        let table = doc.select(&table_sel).next().expect("Table not found");

        let row_sel = Selector::parse("tr").unwrap();
        let rows = table.select(&row_sel);

        let mut transactions: Vec<RawTransaction> = Vec::new();
        let edit_sel = Selector::parse(r"td.tw-text-center.\!tw-pr-0.tw-px-1.tw-py-2\.5.\32 lg\:tw-p-2\.5.tw-bg-inherit.tw-text-gray-900.dark\:tw-text-moon-50 > div > span:nth-child(1)").unwrap();
        for row in rows {
            if let Some(span) = row.select(&edit_sel).next() {
                if let Some(transaction_data) =
                    span.value().attr("data-portfolio-coin-transaction-data")
                {
                    let transaction: RawTransaction = serde_json::from_str(transaction_data)?;
                    transactions.push(transaction);
                }
            }
        }
        println!("Found {} transactions", transactions.len());
        Ok(transactions)
    }
}
