use sqlx::Row;

use crate::secrets::get_secret;

pub struct UploadDatabase {
    pool: sqlx::PgPool,
}
impl UploadDatabase {
    pub async fn new() -> Result<UploadDatabase, sqlx::Error> {
        let url = &get_secret("DATABASE_URL");
        let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
        Ok(Self { pool })
    }
    /*   pub async fn create_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS files (
                file_name VARCHAR PRIMARY KEY,
                file_size VARCHAR NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn add_file(&self, file_name: &str, file_size: &str) -> Result<(), sqlx::Error> {
        self.create_table().await?;
        let mut transaction = self.pool.begin().await?;
        let q = "SELECT EXISTS(SELECT 1 FROM files WHERE file_name = $1)";
        let row = sqlx::query(q)
            .bind(file_name)
            .bind(file_size)
            .fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;

        let exists = row.get::<bool, &str>("exists");

        if !exists {
            let mut transaction = self.pool.begin().await?;

            let sql = "INSERT INTO files (file_name, file_size) VALUES ($1, $2)";

            sqlx::query(sql)
                .bind(file_name) // Bind the actual 'file_name' variable
                .bind(file_size) // Bind the 'file_size' converted to i64 (if applicable)
                .execute(&mut *transaction)
                .await?;

            transaction.commit().await?;
        }
        Ok(())
    }
    pub async fn get_files(&self) -> Result<Vec<(String, String)>, sqlx::Error> {
        self.create_table().await?;
        let mut transaction = self.pool.begin().await?;
        let q = "SELECT * FROM files";
        let rows = sqlx::query(q).fetch_all(&mut *transaction).await?;
        transaction.commit().await?;
        let mut file_info = Vec::new();

        for row in rows.iter() {
            let hello = row.get::<&str, &str>("file_size").to_string();
            let grrr = row.get::<&str, &str>("file_name").to_string();
            println!("File name: {} File Size:{}\n\n", hello, grrr);
            file_info.push((hello, grrr));
        }
        Ok(file_info)
    } */
    pub async fn create_urls_table(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS urls (
                url VARCHAR PRIMARY KEY,
                file_name VARCHAR NOT NULL,
                chunk_filename VARCHAR NOT NULL,
                chunk_size VARCHAR NOT NULL
            )",
        )
        .execute(&self.pool)
        .await
        .unwrap();
        Ok(())
    }
    pub async fn add_url(
        &self,
        url: &str,
        file_name: &str,
        chunk_filename: &str,
        size: &str,
    ) -> Result<(), sqlx::Error> {
        self.create_urls_table().await?;
        let mut transaction = self.pool.begin().await?;
        let q = "SELECT EXISTS(SELECT 1 FROM urls WHERE url = $1 OR chunk_filename = $2)";
        let row = sqlx::query(q)
            .bind(url)
            .bind(file_name)
            .fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;
        let exists = row.get::<bool, &str>("exists");

        if !exists {
            let mut transaction = self.pool.begin().await?;
            sqlx::query(
                "INSERT INTO urls (url, file_name, chunk_filename, chunk_size) VALUES ($1, $2, $3, $4)",
            )
            .bind(url)
            .bind(file_name)
            .bind(chunk_filename)
            .bind(size)
            .execute(&mut *transaction)
            .await
            .unwrap();
            transaction.commit().await.unwrap();
        }

        Ok(())
    }
    pub async fn get_urls_by_filename(&self, file_name: &str) -> Result<Vec<UrlInfo>, sqlx::Error> {
        self.create_urls_table().await?;
        let mut transaction = self.pool.begin().await?;
        let q = "SELECT * FROM urls WHERE file_name = $1";
        let rows = sqlx::query(q)
            .bind(file_name)
            .fetch_all(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await?;

        let mut file_info = Vec::new();

        for row in rows.iter() {
            let url = row.get::<&str, &str>("url").to_string();
            println!("Url : {} ", url);
            let filename = row.get::<&str, &str>("file_name").to_string();
            println!("File name: {} ", filename);
            let chunk_filename = row.get::<&str, &str>("chunk_filename").to_string();
            println!("Chunk File name: {} ", chunk_filename);
            file_info.push(UrlInfo {
                file_name: file_name.to_string(),
                url: url.clone(),
                chunk_filename,
            });

            println!("File name: {} File Size:{}\n\n", url, filename);
        }
        Ok(file_info)
    }
    pub async fn get_urls(&self) -> Result<Vec<String>, sqlx::Error> {
        self.create_urls_table().await?;
        let mut transaction = self.pool.begin().await?;
        let q = "SELECT DISTINCT file_name FROM urls";
        let rows = sqlx::query(q).fetch_all(&mut *transaction).await.unwrap();
        transaction.commit().await?;
        let mut file_info = Vec::new();

        for row in rows.iter() {
            let filename = row.get::<&str, &str>("file_name").to_string();
            file_info.push(filename);
        }
        Ok(file_info)
    }
}
#[derive(Debug, Clone)]
pub struct UrlInfo {
    pub file_name: String,
    pub url: String,
    pub chunk_filename: String,
}
