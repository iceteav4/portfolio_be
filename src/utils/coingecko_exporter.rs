use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs;
use std::num::ParseFloatError;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug)]
pub enum TransactionError {
    ParseFloatError(ParseFloatError),
    TimeParseError(time::error::Parse),
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::ParseFloatError(e) => write!(f, "Failed to parse number: {}", e),
            TransactionError::TimeParseError(e) => write!(f, "Failed to parse timestamp: {}", e),
        }
    }
}

impl std::error::Error for TransactionError {}

pub fn read_html_contents(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Can not read html file");
    contents
}

#[derive(Serialize, Deserialize, Debug)]
struct RawTransaction {
    id: u32,
    transaction_type: String,
    currency: String,
    quantity: String,
    price: String,
    transaction_timestamp: String,
    fees: String,
    notes: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: u32,
    pub transaction_type: String,
    pub currency: String,
    pub quantity: f64,
    pub price: f64,
    pub transaction_dt: OffsetDateTime,
    pub fees: f64,
    pub notes: String,
}

impl Transaction {
    fn from_raw(raw: RawTransaction) -> std::result::Result<Self, TransactionError> {
        let timestamp = OffsetDateTime::parse(&raw.transaction_timestamp, &Rfc3339)
            .map_err(TransactionError::TimeParseError)?;

        Ok(Transaction {
            id: raw.id,
            transaction_type: raw.transaction_type,
            currency: raw.currency,
            quantity: raw
                .quantity
                .parse()
                .map_err(TransactionError::ParseFloatError)?,
            price: raw
                .price
                .parse()
                .map_err(TransactionError::ParseFloatError)?,
            transaction_dt: timestamp,
            fees: raw
                .fees
                .parse()
                .map_err(TransactionError::ParseFloatError)?,
            notes: raw.notes,
        })
    }
}

fn parse_html_contents(contents: &String) -> serde_json::Result<Vec<Transaction>> {
    let doc = Html::parse_document(contents);
    let table_sel = Selector::parse(r"body > div.container > main > div:nth-child(3) > div:nth-child(3) > div.tw-overflow-x-auto.\32 lg\:tw-overflow-x-visible.\32 lg\:tw-flex.\32 lg\:tw-justify-center > table > tbody").unwrap();
    let table = doc.select(&table_sel).next().expect("Table not found");

    let row_sel = Selector::parse("tr").unwrap();
    let rows = table.select(&row_sel);

    let mut transactions: Vec<Transaction> = Vec::new();
    let edit_sel = Selector::parse(r"td.tw-text-center.\!tw-pr-0.tw-px-1.tw-py-2\.5.\32 lg\:tw-p-2\.5.tw-bg-inherit.tw-text-gray-900.dark\:tw-text-moon-50 > div > span:nth-child(1)").unwrap();
    for row in rows {
        if let Some(span) = row.select(&edit_sel).next() {
            if let Some(transaction_data) =
                span.value().attr("data-portfolio-coin-transaction-data")
            {
                let transaction: RawTransaction = serde_json::from_str(transaction_data)?;
                transactions.push(Transaction::from_raw(transaction).unwrap());
            }
        }
    }
    println!("Found {} transactions", transactions.len());
    Ok(transactions)
}

fn export_to_json_file(
    transactions: &Vec<Transaction>,
    json_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize transactions to JSON and write to file
    let transactions_with_string_dt: Vec<serde_json::Value> = transactions
        .iter()
        .map(|t| {
            let transaction = serde_json::json!({
                "id": t.id,
                "transaction_type": t.transaction_type,
                "currency": t.currency,
                "quantity": t.quantity,
                "price": t.price,
                "transaction_dt": t.transaction_dt.format(&Rfc3339).unwrap(),
                "fees": t.fees,
                "notes": t.notes
            });
            transaction
        })
        .collect();

    let json = serde_json::to_string_pretty(&transactions_with_string_dt)?;
    println!("Writing to {}", json_file_path);
    std::fs::write(&json_file_path, json)?;

    Ok(())
}

pub fn read_and_export_coingecko_portfolio(
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let html_contents = read_html_contents(file_path);
    let transactions = parse_html_contents(&html_contents)?;

    // Generate output filename from input path, changing extension to json
    let path = std::path::Path::new(file_path);
    let parent_dir = path.parent().unwrap_or_else(|| std::path::Path::new(""));
    let file_stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let json_file_path = parent_dir
        .join(format!("{}.json", file_stem))
        .to_string_lossy()
        .to_string();
    println!("Exporting to {}", json_file_path);

    export_to_json_file(&transactions, &json_file_path)?;
    Ok(())
}
