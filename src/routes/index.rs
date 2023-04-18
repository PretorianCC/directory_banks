#[get("/")]
pub fn index() -> &'static str {
    "Справочник информации по банкам"
}

#[get("/<template>")]
pub fn find(template: &str) -> &str {
    template
}
