use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use rocket::{fairing::AdHoc, routes, FromForm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Category {
    Cats,
    Dogs,
    Pigs,
    Lizards,
}

impl TryFrom<String> for Category {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Cats" => Ok(Self::Cats),
            "Dogs" => Ok(Self::Dogs),
            "Pigs" => Ok(Self::Pigs),
            "Lizards" => Ok(Self::Lizards),
            s @ _ => Err(format!("invalid category: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Status {
    Available,
    Pending,
    Sold,
}

impl TryFrom<String> for Status {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Available" => Ok(Self::Available),
            "Pending" => Ok(Self::Pending),
            "Sold" => Ok(Self::Sold),
            s @ _ => Err(format!("invalid status: {}", s)),
        }
    }
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::try_from(value.to_string())
    }
}
impl TryFrom<&str> for Category {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Category::try_from(value.to_string())
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PetDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub age: u32,
    pub category: Category,
    pub status: Status,
}

#[derive(FromForm, Debug, Clone, Deserialize)]
pub struct PetCreateForm {
    pub name: String,
    pub age: u32,
    pub category: String,
    pub status: String,
}
impl TryFrom<PetCreateForm> for PetDocument {
    type Error = String;
    fn try_from(value: PetCreateForm) -> Result<Self, Self::Error> {
        if (&value.name.trim()).is_empty() {
            return Err("name is required".to_string());
        }
        let category = Category::try_from(value.category.as_str())?;
        let status = Status::try_from(value.status.as_str())?;
        Ok(Self {
            id: ObjectId::new(),
            name: value.name,
            age: value.age,
            category,
            status,
        })
    }
}

mod dao {
    use super::{
        service::get_by_id, Category, PetCreateForm, PetDocument, PetQueryForm, PetUpdateForm,
        Status,
    };
    use crate::common::{AppError, AppResult, DbClient};
    use mongodb::{
        bson::{doc, oid::ObjectId, Document},
        options::FindOptions,
    };
    use rocket::{data::N, futures::TryStreamExt};
    use rocket_db_pools::Connection;
    pub async fn insert_one(
        client: &Connection<DbClient>,
        doc: &PetDocument,
    ) -> AppResult<ObjectId> {
        let col = client
            .default_database()
            .unwrap()
            .collection::<PetDocument>("pets");
        let ret = col.insert_one(doc, None).await?;
        Ok(ret.inserted_id.as_object_id().unwrap())
    }

    fn build_filter(form: &PetQueryForm) -> Option<Document> {
        let status = match &form.status {
            Some(s) => match Status::try_from(s.as_str()) {
                Ok(st) => Some(st),
                Err(_) => None,
            },
            None => None,
        };

        let name = form.name.clone();
        if status.is_none() && name.is_none() {
            return None;
        }
        let mut doc = doc! {};
        if status.is_some() {
            doc.insert("status", &form.status);
        }

        if name.is_some() {
            doc.insert(
                "name",
                doc! {
                    "$regex": name.unwrap(),
                },
            );
        }
        Some(doc)
    }
    fn build_update(form: &PetUpdateForm) -> Option<Document> {
        let status = match &form.status {
            Some(s) => match Status::try_from(s.as_str()) {
                Ok(_st) => Some(s.clone()),
                Err(_) => None,
            },
            None => None,
        };
        let category = match &form.category {
            Some(c) => match Category::try_from(c.as_str()) {
                Ok(cat) => Some(c),
                Err(_) => None,
            },
            None => None,
        };
        let name = form.name.clone();
        let age = match form.age {
            Some(a) => Some(a),
            None => None,
        };
        let mut doc = doc! {
            "$set": {}
        };

        if category.is_some() {
            doc.get_document_mut("$set")
                .unwrap()
                .insert("category", category.unwrap());
        }
        if status.is_some() {
            doc.get_document_mut("$set")
                .unwrap()
                .insert("status", status.unwrap());
        }
        if name.is_some() {
            doc.get_document_mut("$set")
                .unwrap()
                .insert("name", name.unwrap());
        }
        if form.age.is_some() {
            doc.get_document_mut("$set")
                .unwrap()
                .insert("age", age.unwrap());
        }

        Some(doc)
    }
    pub async fn count(client: &Connection<DbClient>, form: &PetQueryForm) -> AppResult<u64> {
        let col = client
            .default_database()
            .unwrap()
            .collection::<PetDocument>("pets");
        let filter = build_filter(form);
        Ok(col.count_documents(filter, None).await?)
    }
    pub async fn find(
        client: &Connection<DbClient>,
        form: &PetQueryForm,
    ) -> AppResult<Vec<PetDocument>> {
        let col = client
            .default_database()
            .unwrap()
            .collection::<PetDocument>("pets");
        let filter = build_filter(form);
        let options = FindOptions::builder()
            .sort(doc! {"name": 1})
            .skip((form.page - 1) * form.items_per_page)
            .limit(form.items_per_page as i64)
            .build();
        Ok(col
            .find(filter, options)
            .await?
            .try_collect::<Vec<_>>()
            .await?)
    }
    pub async fn find_by_id(
        client: &Connection<DbClient>,
        id: ObjectId,
    ) -> AppResult<Option<PetDocument>> {
        let collection = client
            .default_database()
            .unwrap()
            .collection::<PetDocument>("pets");
        let res = collection.find_one(doc! {"_id":id}, None).await?;
        Ok(res)
    }
    pub async fn update_by_id(
        client: &Connection<DbClient>,
        id: &str,
        data: PetUpdateForm,
    ) -> AppResult<bool> {
        let id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => return Err(AppError::bad_request("id is not valid")),
        };
        let cur = find_by_id(client, id).await?;
        if cur.is_none() {
            return Err(AppError::not_found("pet not found"));
        }
        let col = client
            .default_database()
            .unwrap()
            .collection::<PetDocument>("pets");
        let update_data = build_update(&data);
        if update_data.is_none() {
            return Ok(false);
        }
        let res = col
            .update_one(doc! {"_id": id}, update_data.unwrap(), None)
            .await?;
        Ok(res.modified_count > 0)
    }
}
mod service {
    use super::{dao, PetCreateForm, PetDocument, PetQueryForm, PetUpdateForm};
    use crate::common::{AppError, AppResult, DbClient, PagedResponse};
    use mongodb::bson::oid::ObjectId;
    use rocket::serde::json::Json;
    use rocket_db_pools::Connection;
    pub async fn create(client: &Connection<DbClient>, form: PetCreateForm) -> AppResult<ObjectId> {
        // let doc: PetDocument = form.into();
        let doc = match PetDocument::try_from(form) {
            Ok(doc) => doc,
            Err(s) => return Err(AppError::bad_request(s.as_str())),
        };
        dao::insert_one(client, &doc).await
    }

    pub async fn query(
        client: &Connection<DbClient>,
        form: PetQueryForm,
    ) -> AppResult<PagedResponse<PetDocument>> {
        let total = dao::count(client, &form).await?;
        let items = dao::find(client, &form).await?;
        Ok(PagedResponse { total, items })
    }
    pub async fn get_by_id(
        client: &Connection<DbClient>,
        id: &str,
    ) -> AppResult<Option<PetDocument>> {
        let id = ObjectId::parse_str(id);
        match id {
            Ok(_id) => {}
            Err(_) => return Err(AppError::bad_request("id is not valid")),
        }
        let res = dao::find_by_id(client, id.unwrap()).await?;
        Ok(res)
    }
    pub async fn update(
        client: &Connection<DbClient>,
        id: &str,
        data: Json<PetUpdateForm>,
    ) -> AppResult<bool> {
        dao::update_by_id(client, id, data.into_inner()).await
    }
}
mod controller {
    use super::{service, PetCreateForm, PetDocument, PetQueryForm, PetUpdateForm};
    use crate::common::{AppResult, DbClient, IdResponse, PagedResponse};
    use rocket::{form::Form, get, post, put, serde::json::Json};
    use rocket_db_pools::Connection;
    #[post("/", data = "<data>")]
    pub async fn create(
        client: Connection<DbClient>,
        data: Json<PetCreateForm>,
    ) -> Json<AppResult<IdResponse>> {
        match service::create(&client, data.into_inner()).await {
            Ok(id) => Json(Ok(IdResponse { id: id.to_hex() })),
            Err(e) => Json(Err(e)),
        }
        // oid.to_hex()
    }
    #[get("/?<form..>")]
    pub async fn query(
        client: Connection<DbClient>,
        form: PetQueryForm,
    ) -> Json<AppResult<PagedResponse<PetDocument>>> {
        Json(service::query(&client, form).await)
    }

    #[get("/<id>")]
    pub async fn detail(
        id: String,
        client: Connection<DbClient>,
    ) -> Json<AppResult<Option<PetDocument>>> {
        let res = service::get_by_id(&client, &id).await;
        Json(res)
    }
    #[put("/<id>", data = "<data>")]
    pub async fn update(
        id: String,
        client: Connection<DbClient>,
        data: Json<PetUpdateForm>,
    ) -> Json<AppResult<bool>> {
        Json(service::update(&client, &id, data).await)
    }
}

#[derive(FromForm)]
pub struct PetQueryForm {
    #[field(default = 1)]
    pub page: u64,

    #[field(name = "itemsPerPage", default = 20)]
    pub items_per_page: u64,
    pub status: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PetUpdateForm {
    pub name: Option<String>,
    pub age: Option<u32>,
    pub status: Option<String>,
    pub category: Option<String>,
}
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("pets", |rocket| async {
        rocket.mount(
            "/pets",
            routes![
                controller::create,
                controller::query,
                controller::detail,
                controller::update
            ],
        )
    })
}
