use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Order {
    ASC,
    DESC,
}
#[derive(Serialize, Deserialize)]
pub struct PageOptions {
    pub page: i32,
    pub take: i32,
    pub order: Order,
}
#[derive(Serialize, Deserialize)]
pub struct PageMetaData {
    pub page: i32,
    pub take: i32,
    pub item_count: i32,
    pub page_count: i32,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}
#[derive(Serialize, Deserialize)]
pub struct PageDto<T> {
    pub meta_data: PageMetaData,
    pub data: Vec<T>,
}

impl PageOptions {
    pub fn new(page: i32, take: i32, order: Order) -> Self {
        Self { page, take, order }
    }
    pub fn default() -> Self {
        Self {
            page: 1,
            take: 10,
            order: Order::ASC,
        }
    }
    pub fn skip(&self) -> i32 {
        return (self.page -1) * self.take;
    }
}
impl PageMetaData {
    pub fn new(page_options: PageOptions, item_count: i32) -> Self {
        let tmp = (item_count / page_options.take) as f64;
        let page_count = tmp.ceil() as i32;
        Self {
            page: page_options.page,
            take: page_options.take,
            item_count,
            page_count,
            has_next_page: page_options.page < page_count,
            has_previous_page: page_options.page > 1,
        }
    }
    pub fn default() -> Self {
        Self {
            page: 1,
            take: 10,
            item_count: 0,
            page_count: 0,
            has_next_page: false,
            has_previous_page: false,
        }
    }
}
impl<T> PageDto<T> {
    pub fn new(metadata: PageMetaData, data: Vec<T>) -> Self {
        Self { meta_data: metadata, data }
    }
    pub fn default() -> Self {
        Self {
            meta_data: PageMetaData::default(),
            data: Vec::new(),
        }
    }
}
#[derive(Debug)]
pub enum PageOptionsError {
    Missing,
    Invalid,
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for PageOptions {
    type Error = PageOptionsError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let query = request.uri().query();
        match query {
            Some(query) => {
                let _split_query = query.split("&");
                Outcome::Success(PageOptions::default()

                    // page: query.get("page").unwrap_or("1").parse::<i32>().unwrap_or(1),
                    // take: query.get("take").unwrap_or("10").parse::<i32>().unwrap_or(10),
                    // order: match query.get("order").unwrap_or("ASC") {
                    //     "ASC" => Order::ASC,
                    //     "DESC" => Order::DESC,
                    //     _ => Order::ASC,
                    // },
                )
            },
            _ => Outcome::Success(PageOptions::default()),
        }
    }
}