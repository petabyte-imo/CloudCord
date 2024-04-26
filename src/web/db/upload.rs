use sqlx::Row;

pub struct UploadDatabase {
    pool: sqlx::PgPool,
}
impl UploadDatabase {
    pub async fn new() -> Result<UploadDatabase, sqlx::Error> {
        let url = "postgresql://postgres:7522@localhost:5432/learning_axum";
        let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
        Ok(Self { pool })
    }
    pub async fn create_table(&self) -> Result<(), sqlx::Error> {
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
        let first_filename = file_info[0].1.clone();
        Ok(file_info)
    }
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
}
