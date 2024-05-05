use sqlx::Row;

use crate::secrets::get_secret;

pub struct UploadDatabase {
    pool: sqlx::PgPool,
}
impl UploadDatabase {
    //Initialize connection to the database pool
    pub async fn new() -> Result<UploadDatabase, sqlx::Error> {
        let url = &get_secret("DATABASE_URL");
        let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
        Ok(Self { pool })
    }
    //Create the urls table
    pub async fn create_urls_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS urls (
                url VARCHAR PRIMARY KEY,
                file_name VARCHAR NOT NULL,
                chunk_filename VARCHAR NOT NULL,
                chunk_size VARCHAR NOT NULL,
                encrypted VARCHAR NOT NULL
            )",
        )
        .execute(&self.pool)
        .await
        .unwrap();
        Ok(())
    }
    //Add a url to the database
    pub async fn add_url(
        &self,
        url: &str,
        file_name: &str,
        chunk_filename: &str,
        size: &str,
        encrypted: &str,
    ) -> Result<(), sqlx::Error> {
        self.create_urls_table().await?;
        //Initialize the transaction
        let mut transaction = self.pool.begin().await?;
        //See if the url already exists in the database, or not
        let q = "SELECT EXISTS(SELECT 1 FROM urls WHERE url = $1 OR chunk_filename = $2)";
        let row = sqlx::query(q)
            .bind(url)
            .bind(file_name)
            .fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;
        //Get the boolean value, true or false
        let exists = row.get::<bool, &str>("exists");
        //If it doesn't exist, add it
        if !exists {
            let mut transaction = self.pool.begin().await?;
            sqlx::query(
                "INSERT INTO urls (url, file_name, chunk_filename, chunk_size, encrypted) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(url)
            .bind(file_name)
            .bind(chunk_filename)
            .bind(size)
            .bind(encrypted)
            .execute(&mut *transaction)
            .await
            .unwrap();
            transaction.commit().await.unwrap();
        }

        Ok(())
    }
    pub async fn get_urls_by_filename(&self, file_name: &str) -> Result<Vec<UrlInfo>, sqlx::Error> {
        self.create_urls_table().await?;
        //Initialize the transaction
        let mut transaction = self.pool.begin().await?;
        //Get all the urls in the database
        let q = "SELECT * FROM urls WHERE file_name = $1";
        let rows = sqlx::query(q)
            .bind(file_name)
            .fetch_all(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await?;

        //Create a file info vector to store the urls that we got from the database
        let mut file_info = Vec::new();
        //We start an iteration, to iterate over the rows we got
        for row in rows.iter() {
            //Get the url from the row
            let url = row.get::<&str, &str>("url").to_string();
            //Get the filename from the row
            let filename = row.get::<&str, &str>("file_name").to_string();
            //Get the chunk filename from the row
            let chunk_filename = row.get::<&str, &str>("chunk_filename").to_string();
            //Push all the info to the file info vector
            file_info.push(UrlInfo {
                file_name: file_name.to_string(),
                url: url.clone(),
                chunk_filename,
            });
            //Debug print
            println!(
                "\nRetrieving File with url: {} And File Size:{}",
                url, filename
            );
        }
        Ok(file_info)
    }
    pub async fn get_names(&self) -> Result<Vec<String>, sqlx::Error> {
        self.create_urls_table().await?;
        //Initialize the transaction
        let mut transaction = self.pool.begin().await?;
        //Select the distinct file names from the urls table
        let q = "SELECT DISTINCT file_name FROM urls";
        let rows = sqlx::query(q).fetch_all(&mut *transaction).await.unwrap();
        transaction.commit().await?;
        //Create a file info vector to store the urls that we got from the database
        let mut file_info = Vec::new();
        //We start an iteration, to iterate over the rows we got
        for row in rows.iter() {
            //Get the filename from the row
            let filename = row.get::<&str, &str>("file_name").to_string();
            //Push it to the file info vector
            file_info.push(filename);
        }
        Ok(file_info)
    }
    pub async fn get_encrypted(&self) -> Result<Vec<String>, sqlx::Error> {
        self.create_urls_table().await?;
        //Initialize the transaction
        let mut transaction = self.pool.begin().await?;
        //Select the distinct file names from the urls table
        let q = r#"SELECT u.file_name, e.encrypted
        FROM (
            SELECT DISTINCT file_name
            FROM urls
        ) AS u
        LEFT JOIN urls AS e ON u.file_name = e.file_name;
        "#;
        let rows = sqlx::query(q).fetch_all(&mut *transaction).await.unwrap();
        transaction.commit().await?;
        //Create a file info vector to store the urls that we got from the database
        let mut file_info = Vec::new();
        //We start an iteration, to iterate over the rows we got
        for row in rows.iter() {
            //Get the filename from the row

            let filename = row.get::<&str, usize>(1).to_string();
            //Push it to the file info vector
            file_info.push(filename);
        }
        Ok(file_info)
    }

    pub async fn chunk_filename_exist(
        &self,
        chunk_filename: &str,
    ) -> Result<(bool, i64), sqlx::Error> {
        self.create_urls_table().await?;
        let mut transaction = self.pool.begin().await?;
        //See if it exists or not
        let q = "SELECT EXISTS(SELECT 1 FROM urls WHERE chunk_filename = $1)";
        //Select the count of urls with the same chunk_filename
        let q1 = "SELECT COUNT(*) FROM urls WHERE chunk_filename = $1";
        let row = sqlx::query(q)
            .bind(chunk_filename)
            .fetch_one(&mut *transaction)
            .await?;
        let row1 = sqlx::query(q1)
            .bind(chunk_filename)
            .fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;

        let count = row1.get::<i64, &str>("count");
        let exists = row.get::<bool, &str>("exists");

        Ok((exists, count))
    }
    pub async fn delete_from_filename(&self, file_name: &str) -> Result<(), sqlx::Error> {
        self.create_urls_table().await?;
        let mut transaction = self.pool.begin().await?;
        //Delete it from the database
        let q = "DELETE FROM urls WHERE file_name = $1";
        sqlx::query(q)
            .bind(file_name)
            .execute(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        Ok(())
    }
    pub async fn close(&self) {
        self.pool.close().await
    }
}

pub struct UrlInfo {
    pub file_name: String,
    pub url: String,
    pub chunk_filename: String,
}
