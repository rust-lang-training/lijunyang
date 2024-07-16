use async_std::task;
use futures::future::{join, join_all};
use std::sync::{Arc, Mutex};
use std::time;
async fn hello() {
    println!("hello");
}

async fn connect_db() -> String {
    task::sleep(time::Duration::from_secs(1)).await;
    String::from("connect_db successfully")
}

async fn open_file() -> String {
    task::sleep(time::Duration::from_secs(1)).await;
    String::from("open_file successfully")
}

async fn back_with_result() -> Result<String, ()> {
    Ok(String::from("Result"))
}

pub async fn main_exe() {
    hello().await;
    let (db, filename) = join(connect_db(), open_file()).await;
    println!("{}, {}", db, filename);
    println!("{}", back_with_result().await.unwrap());
}

pub async fn select_db(input: &str) -> String {
    task::sleep(time::Duration::from_secs(1)).await;
    format!("select :{}", input)
}

pub async fn get_cities() -> Vec<String> {
    let cities = vec![
        String::from("shanghai"),
        String::from("beijing"),
        String::from("guangzhou"),
        String::from("shenzhen"),
    ];

    let city_vec: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    join_all(
        cities
            .into_iter()
            .map(|city| build_city(city_vec.clone(), city)),
    )
    .await;

    return city_vec.lock().unwrap().clone();
}

async fn build_city(city_vec: Arc<Mutex<Vec<String>>>, city: String) {
    task::sleep(time::Duration::from_secs(1)).await;
    println!("super city build");
    city_vec
        .lock()
        .unwrap()
        .push(format!("china super city {}", city));
}

pub async fn test02_main() {
    let now = time::Instant::now();
    main_exe().await;
    println!("main");

    let users = vec!["ma", "hua", "teng"];
    let user_info = join_all(users.into_iter().map(|user| select_db(user))).await;
    println!("users info: {:?}", user_info);
    println!("executed in {:?}!", now.elapsed());
    println!("{:?}", get_cities().await);
}
