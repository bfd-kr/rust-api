use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use simple_server_timing_header::Timer;

use crate::{
    fibonacci::{calculate_fibonacci, MAX_FIBONACCI_NUMBER},
    models::{ApiResponse, Error},
    random_kbytes::{generate_random_kbytes, MAX_KBYTES},
};

#[derive(Serialize)]
struct FibonacciKBytesResponse {
    f: u32,
    fibonacci: u32,
    k: usize,
    kbytes: String,
}

pub async fn fibonacci_kbytes(req: HttpRequest) -> impl Responder {
    let f: u32 = req.match_info().query("f").parse().unwrap_or(0);
    let k: usize = req.match_info().query("k").parse().unwrap_or(0);

    // validate the number is not negative and less than max
    if f > MAX_FIBONACCI_NUMBER {
        let error_response: ApiResponse<Error> = ApiResponse::Error {
            error: Error {
                code: "BAD_REQUEST".to_string(),
                message: format!(
                    "Invalid value of f: {}. Please provide a number between 0 and {}",
                    f, MAX_FIBONACCI_NUMBER
                ),
            },
        };

        return HttpResponse::BadRequest().json(error_response);
    }

    if k > MAX_KBYTES {
        let error_response: ApiResponse<Error> = ApiResponse::Error {
            error: Error {
                code: "BAD_REQUEST".to_string(),
                message: format!(
                    "Invalid value of k: {}. Please provide a number between 0 and {}",
                    k, MAX_KBYTES
                ),
            },
        };

        return HttpResponse::BadRequest().json(error_response);
    }

    let mut timer = Timer::new();
    let fibonacci = calculate_fibonacci(f);

    timer.add("calculate_fibonacci");

    let kbytes = generate_random_kbytes(k);

    timer.add("calculate_kbytes");

    let response = FibonacciKBytesResponse {
        f,
        fibonacci,
        k,
        kbytes,
    };

    let wrapped_response = ApiResponse::Success { data: response };

    HttpResponse::Ok()
        .insert_header((Timer::header_key(), timer.header_value()))
        .json(wrapped_response)
}
