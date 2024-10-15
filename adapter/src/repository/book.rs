use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::{event::CreateBook, Book};
use kernel::repository::book::BookRepository;
use uuid::Uuid;

use crate::database::model::book::BookRow;
use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description)
                VALUES($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                ORDER BY created_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        let books = rows.into_iter().map(Book::from).collect();

        Ok(books)
    }

    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT
                    book_id,
                    title,
                    author,
                    isbn,
                    description
                FROM books
                WHERE book_id = $1
            "#,
            book_id
        )
        .fetch_optional(self.db.inner_ref())
        .await?;

        let book = row.map(Book::from);

        Ok(book)
    }
}
