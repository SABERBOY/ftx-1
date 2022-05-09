/* use serde::{Deserialize, Serialize};
use calamine::{RangeDeserializerBuilder, Reader, Xlsx}; */

fn main() {
    /* println!("Hello, world!");
    use calamine::{open_workbook, Reader, Xlsx};

    let mut excel: Xlsx<_> = open_workbook("C:\\Users\\11322\\Documents\\GitHub\\Ftx-Language-Platform\\rust\\src\\@SABERLOVEWH_user_tweets.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("users") {
        for row in r.rows() {
            println!("row={:?}", row);
        }
    } */
    /* if let Some(Ok(r)) = excel.worksheet_range("tweets") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    } */
}
// User Id	Name	Screen Name	Created At	Followers	Following	Favorites	Tweets	Lists	Bio	Location	URL	Verified	Default Profile




/* #[derive(Serialize, Deserialize, Debug)]
struct RawExcelRow {
    metric: String,
    #[serde(deserialize_with = "de_opt_f64")]
    value: Option<f64>,
} */


// Convert value cell to Some(f64) if float or int, else None
/* fn de_opt_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::Error(_)) => Ok(None),
        Ok(calamine::DataType::Float(f)) => Ok(Some(f)),
        Ok(calamine::DataType::Int(i)) => Ok(Some(i as f64)),
        _ => Ok(None),
    }
} */

/* fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let path = format!("{}/tests/excel.xlsx", env!("CARGO_MANIFEST_DIR"));
    let mut excel: Xlsx<_> = open_workbook(path)?;

    let range = excel
      .worksheet_range("Sheet1")
      .ok_or(calamine::Error::Msg("Cannot find Sheet1"))??;

    let iter_result =
        RangeDeserializerBuilder::with_headers(&COLUMNS).from_range::<_, RawExcelRow>(&range)?;
  } */