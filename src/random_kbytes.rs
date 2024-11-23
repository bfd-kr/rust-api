use actix_web::{HttpRequest, HttpResponse, Responder};
use hex::encode;
use rand::Rng;
use serde::Serialize;
use simple_server_timing_header::Timer;

use crate::models::{ApiResponse, Error};

pub(crate) const MAX_KBYTES: usize = 1024;

#[derive(Serialize)]
struct KbytesResponse {
    number: usize,
    kbytes: String,
}

pub async fn random_kbytes(req: HttpRequest) -> impl Responder {
    let n: usize = req.match_info().query("n").parse().unwrap_or(0);

    if n > MAX_KBYTES {
        let error_response: ApiResponse<Error> = ApiResponse::Error {
            error: Error {
                code: "BAD_REQUEST".to_string(),
                message: format!(
                    "Invalid number: {}. Please provide a number between 0 and {}",
                    n, MAX_KBYTES
                ),
            },
        };

        return HttpResponse::BadRequest().json(error_response);
    }

    let mut timer = Timer::new();
    let response = KbytesResponse {
        number: n,
        kbytes: generate_random_kbytes(n),
    };

    timer.add("calculate_kbytes");

    let wrapped_response = ApiResponse::Success { data: response };

    HttpResponse::Ok()
        .insert_header((Timer::header_key(), timer.header_value()))
        .json(wrapped_response)
}

pub(crate) fn generate_random_kbytes(kb: usize) -> String {
    let n = kb * 1024; // Convert kilobytes to bytes
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..n).map(|_| rng.gen::<u8>()).collect();
    let encoded_data = encode(data);

    // hex encoding will be double the length of the original data
    encoded_data[..encoded_data.len() / 2].to_string()
}
