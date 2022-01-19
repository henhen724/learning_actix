use futures::stream::StreamExt;
use mongodb::{error::Result, Cursor};
use serde::de::DeserializeOwned;

pub async fn convert_cursor_to_vec<T: DeserializeOwned + Unpin + Send + Sync>(
    cursor_res: Result<Cursor<T>>,
) -> Result<Vec<T>> {
    let mut cursor = cursor_res?;
    let mut doc_list = Vec::new();
    while let Some(maybe_doc) = cursor.next().await {
        match maybe_doc {
            Ok(doc) => doc_list.push(doc),
            Err(e) => {
                println!("Error while getting, {:?}", e);
            }
        }
    }
    Ok(doc_list)
}
