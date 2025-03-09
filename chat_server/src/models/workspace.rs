use crate::{AppError, User, Workspace};

impl Workspace {
    pub async fn find_by_name(
        pool: &sqlx::PgPool,
        workspace_name: &str,
    ) -> Result<Option<Self>, AppError> {
        let workspace = sqlx::query_as(
            r#"
            select id,name,owner_id,created_at from workspaces where name = $1
            "#,
        )
        .bind(&workspace_name)
        .fetch_one(pool)
        .await?;

        Ok(Some(workspace))
    }

    pub async fn create(pool: &sqlx::PgPool, name: &str, user_id: i32) -> Result<Self, AppError> {
        let workspace = sqlx::query_as(
            r#"
            INSERT INTO workspaces (name,owner_id)
            VALUES ($1, $2)
            RETURNING id, name, owner_id, created_at
            "#,
        )
        .bind(&name)
        .bind(&user_id)
        .fetch_one(pool)
        .await?;
        Ok(workspace)
    }

    pub async fn fetch_all_users(
        pool: &sqlx::PgPool,
        workspace_id: i32,
    ) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as(
            r#"
            select id,username,email,ws_id,created_at from users where ws_id = $1
            "#,
        )
        .bind(&workspace_id)
        .fetch_all(pool)
        .await?;
        Ok(users)
    }
}

#[test]
fn test() {

    use sqlx_db_tester::TestPg;
    #[tokio::test]
    async fn test_create_workspace() {
        let tdb = TestPg::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            std::path::Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let workspace = Workspace::create(&pool, "test", 1).await.unwrap();
        assert_eq!(workspace.name, "test");
        assert_eq!(workspace.owner_id, 1);
    }

    #[tokio::test]
    async fn test_fetch_all_users() {
        let tdb = TestPg::new(
            "postgres://postgres:postgres@localhost:5432".to_string(),
            std::path::Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let workspace = Workspace::create(&pool, "test", 1).await.unwrap();
        let users = Workspace::fetch_all_users(&pool, workspace.id)
            .await
            .unwrap();
        assert_eq!(users.len(), 0);
    }
}
