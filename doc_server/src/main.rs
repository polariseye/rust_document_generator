mod api_doc;
mod project;

use once_cell::sync::Lazy;
use tera::Tera;
use warp::http::StatusCode;
use warp::{Filter, Rejection};
use std::collections::HashMap;
use std::error::Error;

/*
pub fn do_nothing_filter(value: &tera::Value, _: &HashMap<String, tera::Value>) ->tera::Result<tera::Value> {
    let s = tera::try_get_value!("do_nothing_filter", "value", String, value);
    Ok(tera::to_value(&s).unwrap())
}
*/

static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera_obj = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(err) => {
            println!("Parsing error:{}", err);
            std::process::exit(1);
        }
    };

    tera_obj.autoescape_on(vec!["html"]);
    //tera_obj.register_filter("do_nothing",do_nothing_filter);

    if tera_obj.get_template_names().count()<=0{
        println!("have no template");
        std::process::exit(1);
    }

    for item in tera_obj.get_template_names(){
        println!("templeta item {}",item);
    }

    match tera_obj.full_reload() {
        Ok(_) => {}
        Err(err) => {
            println!("full_reload error:{}", err);
            std::process::exit(2);
        }
    }

    tera_obj
});

/// API文档生成服务
/// ref-->https://github.com/polariseye/rust_document_generator
#[doc_macro::doc_header]
#[tokio::main]
async fn main() {
    println!("work dir:{}",std::env::current_dir().unwrap().to_str().unwrap());
    //test_tera();
    TERA.check_macro_files();

    let api_filter = warp::any();
    let api_filter = api_filter
        .and(
            api_filter
                .and(warp::path::end())
                .and(warp::get())
                .and_then(get_project_list),
        )
        .or(api_filter
            .and(warp::path!("api" / String))
            .and(warp::path::end())
            .and(warp::get())
            .and_then(get_api_list));

    warp::serve(api_filter).run(([127, 0, 0, 1], 3032)).await;
}

/// module Main
/// fn GetProjectList / get 获取项目列表
/// return
///     一个html文档
#[doc_macro::api]
pub async fn get_project_list() -> Result<Box<dyn warp::Reply>, Rejection> {
    let result = project::get_project_list();
    return match result {
        Ok(val) => {
            let mut ctx = tera::Context::new();
            ctx.insert("project_list", &val);

            match TERA.render("index.html", &ctx) {
                Ok(val) => Ok(Box::new(warp::reply::html(val))),
                Err(err) => Ok(Box::new(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))),
            }
        }
        Err(err) => Ok(Box::new(warp::reply::with_status(
            err.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))),
    };
}

/// module Main
/// fn GetApiList /api/{project_id} get 获取指定项目的API列表
/// param
///     ProjectId    string required     项目Id ，此参数在路径上面
/// return
///     一个html文档
#[doc_macro::api]
pub async fn get_api_list(project_id: String) -> Result<Box<dyn warp::Reply>, Rejection> {
    let project_item;
    match project::get_project_item(project_id) {
        Ok(val) => {
            project_item = val;
        }
        Err(err) => {
            return Ok(Box::new(warp::reply::with_status(
                err,
                StatusCode::INTERNAL_SERVER_ERROR,
            )));
        }
    }

    return match api_doc::get_api_list(&project_item) {
        Ok(val) => {
            let mut ctx = tera::Context::new();
            ctx.insert("api_list", &val);
            ctx.insert("project_item", &project_item);

            match TERA.render("api.html", &ctx) {
                Ok(val) => Ok(Box::new(warp::reply::html(val))),
                Err(err) => Ok(Box::new(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))),
            }
        }
        Err(err) => Ok(Box::new(warp::reply::with_status(
            err,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))),
    };
}
