
use crate::AppState;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse, Result};
use futures::StreamExt;
use std::io::Write;
use std::path::Path;
use tokio::fs;

#[post("/upload")]
async fn upload_file(mut payload: Multipart, data: web::Data<AppState>) -> Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map(|f| sanitize_filename::sanitize(f))
            .ok_or_else(|| actix_web::error::ErrorBadRequest("No filename"))?;

        let filepath = format!("{}/{}", data.upload_dir, filename);
        let mut f = web::block(move || std::fs::File::create(&filepath)).await??;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
    }
    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[get("/download/{filename}")]
async fn download_file(filename: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let filename = filename.into_inner();
    let filepath = format!("{}/{}", data.upload_dir, sanitize_filename::sanitize(&filename));

    if Path::new(&filepath).exists() {
        let file: Vec<u8> = fs::read(&filepath).await?;
        Ok(HttpResponse::Ok()
            .content_type("application/octet-stream")
            .append_header((
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", filename),
            ))
            .body(file))
    } else {
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}

#[delete("/delete/{filename}")]
async fn delete_file(filename: web::Path<String>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let filename = filename.into_inner();
    let filepath = format!("{}/{}", data.upload_dir, sanitize_filename::sanitize(&filename));
    if Path::new(&filepath).exists() {
        fs::remove_file(filepath).await?;
        Ok(HttpResponse::Ok().body("File deleted successfully"))
    } else {
        Ok(HttpResponse::NotFound().body("File not found"))
    }
}

#[get("/list")]
async fn list_files(data: web::Data<AppState>) -> Result<HttpResponse> {
    let mut entries = fs::read_dir(&data.upload_dir).await?;
    let mut files: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        if let Ok(file_type) = entry.file_type().await {
            if file_type.is_file() {
                files.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    Ok(HttpResponse::Ok().json(files))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(upload_file)
            .service(download_file)
            .service(delete_file)
            .service(list_files)
    );
}

