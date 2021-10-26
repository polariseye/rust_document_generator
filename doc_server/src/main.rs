mod api_doc;
mod project;

use once_cell::sync::Lazy;
use tera::Tera;
use warp::http::StatusCode;
use warp::{Filter, Rejection};
use std::str::FromStr;

static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera_obj = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(err) => {
            println!("Parsing error:{}", err);
            std::process::exit(1);
        }
    };

    tera_obj.autoescape_on(vec!["html"]);

    if tera_obj.get_template_names().count()<=0{
        println!("have no template");
        std::process::exit(1);
    }

    for item in tera_obj.get_template_names(){
        println!("templeta item {}",item);
    }

    /* no need reload
    match tera_obj.full_reload() {
        Ok(_) => {}
        Err(err) => {
            println!("full_reload error:{}", err);
            std::process::exit(2);
        }
    }*/

    tera_obj
});

/// API文档生成服务
/// ref-->https://github.com/polariseye/rust_document_generator
#[doc_macro::doc_header]
#[tokio::main]
async fn main() {
    let arg_list:Vec<String>= std::env::args().collect();
    let mut port=8011u16;
    if arg_list.len()!=0&& &arg_list[1].to_lowercase()=="-p"{
        port=match u16::from_str( &arg_list[2]){
            Ok(val)=>{
                val
            },
            Err(err)=>{
                println!("convert '{}' to listen port error:{}",&arg_list[2],err.to_string());
                std::process::exit(1);
            }
        }
    }

    let _result=TERA.check_macro_files();

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
            .and_then(get_api_list))
        .or(
            warp::path("static").and(warp::fs::dir("./static/"))
        );

    println!("doc server start listen at:{}",port);
    warp::serve(api_filter).run(([0, 0, 0, 0], port)).await;
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

    return match api_doc::get_grouped_api_list(&project_item) {
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
