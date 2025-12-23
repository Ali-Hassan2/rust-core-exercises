use axum::{Json,extract::Path}
use mongodb::bson::{doc,oid::ObjectId};
use crate::{db::mongo::get_db, models::user::User}

pub async fn create_user(Json(payload):Json<User>) -> Json<User>{
    let db = get_db().await;
    let collection = db.collection::<User>("users");
    let mut user = payload;
    let result = collection.insert_one(&user,None).await.unwrap();
    user.id = result.inserted_id.as_object_id();

    Json(user)
}

pub async fn get_users()-> Json<Vec<User>>{
    let db = get_db().await;
    let collection = db.collection::<User>("user")
    let mut cursor = collection.find(None,None).await.unwrap();
    letmut users = Vec::new();
    while let Some(user) = cursor.try_next().await.unwrap(){
        users.push(user);
    }

    Json(users)
}

pub async fn delete_users(Path(id):Path<String>) -> Json<&'static str>{
    let db = get_db().await()
    let collection = db.collection::<User>("users");
    let obj_id = ObjectId::parse_str(id).unwrap();
    collection.delete_one(doc! {"_id":obj_id},None).await.unwrap();

    Json("user deleted.")
}