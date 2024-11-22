use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use simple_server_timing_header::Timer;

use crate::models::{ApiResponse, Error};

const MAX_FIBONACCI_NUMBER: u32 = 45;

#[derive(Serialize)]
struct FibonacciResponse {
    number: u32,
    fibonacci: u32,
}

pub async fn fibonacci(req: HttpRequest) -> impl Responder {
    let n: u32 = req.match_info().query("n").parse().unwrap_or(0);

    // validate the number is not negative and less than 40
    if n > MAX_FIBONACCI_NUMBER {
        let error_response: ApiResponse<Error> = ApiResponse::Error {
            error: Error {
                code: "BAD_REQUEST".to_string(),
                message: format!(
                    "Invalid number: {}. Please provide a number between 0 and {}",
                    n, MAX_FIBONACCI_NUMBER
                ),
            },
        };

        return HttpResponse::BadRequest().json(error_response);
    }

    let mut timer = Timer::new();

    let response = FibonacciResponse {
        number: n,
        fibonacci: calculate_fibonacci(n),
    };

    timer.add("calculate_fibonacci");

    let wrapped_response = ApiResponse::Success {
        data: Some(response),
    };

    HttpResponse::Ok()
        .insert_header((Timer::header_key(), timer.header_value()))
        .json(wrapped_response)
}

fn calculate_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2),
    }
}
