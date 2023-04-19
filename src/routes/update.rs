use crate::config_app::Config;
use crate::database::connection;
use crate::database::record_bank::RecordBank;
use rocket::State;

#[get("/")]
pub fn index() -> &'static str {
    "Обновление справочника"
}

#[get("/<key>")]
pub fn update(config: &State<Config>, key: &str) -> String {
    // ключи не совпадают, ошибка
    if &config.key_update[..] != key {
        return "Error key".to_string();
    }

    let conn = match connection::open(&config.base_name) {
        Ok(i) => i,
        Err(e) => return e.to_string(),
    };

    let _ = connection::clear(&conn);

    let record = RecordBank {
        city: "Омск".to_string(),
        name: "Сбербанк".to_string(),
        bik: "2344343".to_string(),
        coor: "43243242432243432".to_string(),
    };

    match record.add(&conn) {
        Err(e) => return e.to_string(),
        _ => {}
    };

    let _ = conn.close();

    "conf_string".to_string()
}
