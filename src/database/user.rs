use crate::database::db::get_connection;
use crate::entity::user::UserModel;

pub async fn create(user: &UserModel) -> Result<(), sqlx::Error> {
    let connection = get_connection().await;
    sqlx::query(
        r#"
        insert into userinfos (name,age,email,address) values (?,?,?,?)
    "#,
    )
    .bind(&user.name)
    .bind(&user.age)
    .bind(&user.email)
    .bind(&user.address)
    .execute(&*connection)
    .await?;
    Ok(())
}

pub async fn query_by_email(email: &str) -> Result<UserModel, sqlx::Error> {
    let connection = get_connection().await;

    let user_model = sqlx::query_as!(
        UserModel,
        "SELECT id, name, age, email, address from userinfos where email = ?",
        email
    )
    .fetch_one(&*connection)
    .await?;

    Ok(user_model)
}

pub async fn list_user(size: u32, offset: u32) -> Result<Vec<UserModel>, sqlx::Error> {
    let connection = get_connection().await;

    let users = sqlx::query_as!(
        UserModel,
        "SELECT id, name, age, email, address from userinfos limit ? offset ?",
        size,
        offset
    )
    .fetch_all(&*connection)
    .await?;

    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[tokio::test]
    async fn test_create() {
        let mut rng = rand::thread_rng();

        let rand: i32 = rng.gen();

        let age: u8 = rng.gen();

        let email = format!("wx.uv{}@gmail.com", rand);

        let user = UserModel {
            id: 0,
            name: "James Bond".to_string(),
            age: age as i32,
            email,
            address: "北京市海淀区".to_string(),
        };
        let create_status = create(&user).await;

        println!("create_status {:?}", create_status)
    }

    #[tokio::test]
    async fn test_query_by_email() {
        let rsp = query_by_email("uu.wx@gmail.com").await;
        println!("rsp {:?}", rsp)
    }

    #[tokio::test]
    async fn test_list_user() {
        let list_rsp = list_user(10, 0).await;
        println!("list_rsp {:?}", list_rsp)
    }
}
