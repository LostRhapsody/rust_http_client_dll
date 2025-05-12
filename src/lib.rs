use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use reqwest::blocking::Client;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perform_http_request(
    url: *const c_char,
    method: *const c_char,
    headers: *const c_char,
    body: *const c_char,
    status_code: *mut c_int,
    error_code: *mut c_int,
) -> *mut c_char {
    let start_total = Instant::now();

    // Validate input pointers
    if url.is_null() || method.is_null() || headers.is_null() || body.is_null() {
        if !error_code.is_null() {
            *error_code = 2; // Invalid input
        }
        return std::ptr::null_mut();
    }
    if status_code.is_null() || error_code.is_null() {
        return std::ptr::null_mut();
    }

    // Initialize outputs
    *status_code = 0;
    *error_code = 0;

    // Convert C strings to Rust strings
    let url = match CStr::from_ptr(url).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_code = 3; // Invalid UTF-8
            return std::ptr::null_mut();
        }
    };
    let method = CStr::from_ptr(method).to_str().unwrap_or("GET");
    let headers = CStr::from_ptr(headers).to_str().unwrap_or("");
    let body = CStr::from_ptr(body).to_str().unwrap_or("");

    // Create HTTP client
    let client = Client::new();
    let mut request = match method.to_uppercase().as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        _ => client.get(url),
    };

    // Add headers
    for header in headers.split(';').filter(|h| !h.is_empty()) {
        let parts: Vec<&str> = header.split(':').collect();
        if parts.len() == 2 {
            request = request.header(parts[0].trim(), parts[1].trim());
        }
    }

    // Add body if present
    if !body.is_empty() {
        request = request.body(body.to_string());
    }

    let start_http = Instant::now();
    let response = request.send();
    let http_duration = start_http.elapsed();
    
    // Execute request
    match response {
        Ok(resp) => {
            *status_code = resp.status().as_u16() as c_int;
            let text = resp.text().unwrap_or_default();
            // Write response to file
            if let Ok(mut file) = File::create("response.txt") {
                let _ = file.write_all(text.as_bytes());
            }
            // Log profiling data
            let total_duration = start_total.elapsed();
            if let Ok(mut log) = File::create("rust_profile.log") {
                let _ = writeln!(
                    log,
                    "Total: {}ms, HTTP Request: {}ms",
                    total_duration.as_millis(),
                    http_duration.as_millis()
                );
            }
            match CString::new(text) {
                Ok(c_text) => c_text.into_raw(),
                Err(_) => {
                    *error_code = 4; // Null byte in response
                    std::ptr::null_mut()
                }
            }
        }
        Err(_) => {
            *error_code = 1; // HTTP request failed
            std::ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_response(response: *mut c_char) {
    if !response.is_null() {
        let _ = CString::from_raw(response);
    }
}
