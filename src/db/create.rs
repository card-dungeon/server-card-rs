// use mongodb::bson::{doc, Document};

// pub async fn create_card() {
//     let docs = vec![
//         doc! { "title": "1984", "author": "George Orwell" },
//         doc! { "title": "Animal Farm", "author": "George Orwell" },
//         doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
//     ];
//     let collection = database.collection::<Document>("books");
//     // Insert some documents into the "mydb.books" collection.
//     collection.insert_many(docs, None).await?;
// }