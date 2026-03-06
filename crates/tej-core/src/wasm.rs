// WASM-compatible speed test implementation using web-sys fetch
// This replaces tokio/reqwest with browser-native APIs

use js_sys::{Function, Uint8Array};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};


// Speed test result structure
#[derive(serde::Serialize)]
pub struct WasmSpeedTestResult {
    pub server_location: Option<String>,
    pub latency: Option<WasmLatencyResult>,
    pub download: Option<WasmThroughputResult>,
    pub upload: Option<WasmThroughputResult>,
    pub packet_loss: Option<f64>,
}

#[derive(serde::Serialize)]
pub struct WasmLatencyResult {
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub jitter_ms: f64,
}

#[derive(serde::Serialize)]
pub struct WasmThroughputResult {
    pub mbps: f64,
    pub bytes: u64,
    pub duration_secs: f64,
}

#[derive(serde::Serialize)]
pub struct ProgressUpdate {
    pub phase: String,
    pub speed_mbps: Option<f64>,
    pub progress: f64,
    pub latency_ms: Option<f64>,
}

// Generate random data for upload test
fn generate_random_data(size: usize) -> Vec<u8> {
    use rand::RngCore;
    let mut data = vec![0u8; size];
    rand::thread_rng().fill_bytes(&mut data);
    data
}

// Measure latency using fetch
async fn measure_latency_js() -> Result<(WasmLatencyResult, Option<String>), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let mut latencies = Vec::new();
    let mut server_location = None;

    // Warmup requests (discarded)
    for _ in 0..3 {
        let start = js_sys::Date::now();
        let request = Request::new_with_str("https://speed.cloudflare.com/__down?bytes=0")?;
        let _ = JsFuture::from(window.fetch_with_request(&request)).await;
        let _ = js_sys::Date::now() - start;
    }
    
    // Actual measurements
    for _ in 0..20 {
        let start = js_sys::Date::now();
        let request = Request::new_with_str("https://speed.cloudflare.com/__down?bytes=0")?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;
        
        // Get server location from cf-ray header
        if server_location.is_none() {
            let headers = resp.headers();
            if let Ok(cf_ray) = headers.get("cf-ray") {
                if let Some(ref ray) = cf_ray {
                    if let Some(dash) = ray.rfind('-') {
                        server_location = Some(ray[dash + 1..].to_string());
                    }
                }
            }
        }
        
        let end = js_sys::Date::now();
        latencies.push(end - start);
    }

    // Calculate statistics
    let avg = latencies.iter().sum::<f64>() / latencies.len() as f64;
    let min = *latencies
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);
    let max = *latencies
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);
    
    // Calculate jitter (mean absolute difference between consecutive samples)
    let jitter = if latencies.len() > 1 {
        let diffs: Vec<f64> = latencies.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
        diffs.iter().sum::<f64>() / diffs.len() as f64
    } else {
        0.0
    };
    
    Ok((
        WasmLatencyResult {
            avg_ms: avg,
            min_ms: min,
            max_ms: max,
            jitter_ms: jitter,
        },
        server_location,
    ))
}

// Measure download speed
async fn measure_download_js(
    on_progress: &Option<Function>,
) -> Result<WasmThroughputResult, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let test_size = 25_000_000u64; // 25MB
    let url = format!("https://speed.cloudflare.com/__down?bytes={}", test_size);

    let start = js_sys::Date::now();
    let mut total_bytes: u64 = 0;
    
    let request = Request::new_with_str(&url)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    // Get reader from response body
    if let Some(body) = resp.body() {
        let reader = body
            .get_reader()
            .dyn_into::<web_sys::ReadableStreamDefaultReader>()?;
        
        loop {
            let promise = reader.read();
            let chunk = JsFuture::from(promise).await?;
            
            let done = js_sys::Reflect::get(&chunk, &"done".into())?
                .as_bool()
                .unwrap_or(true);
            
            if done {
                break;
            }

            if let Ok(value) = js_sys::Reflect::get(&chunk, &"value".into()) {
                if let Ok(arr) = value.dyn_into::<Uint8Array>() {
                    total_bytes += arr.length() as u64;
                }
            }

            // Progress callback
            if let Some(cb) = on_progress {
                let elapsed = (js_sys::Date::now() - start) / 1000.0;
                let speed_bps = (total_bytes as f64 * 8.0) / elapsed.max(0.1);
                let speed_mbps = speed_bps / 1_000_000.0;
                let progress = (total_bytes as f64 / test_size as f64).min(1.0);

                let _ = cb.call3(
                    &JsValue::NULL,
                    &JsValue::from_str("download"),
                    &JsValue::from_f64(speed_mbps),
                    &JsValue::from_f64(progress),
                );
            }
        }
    }
    
    let end = js_sys::Date::now();
    let duration_secs = (end - start) / 1000.0;
    let mbps = (total_bytes as f64 * 8.0) / (duration_secs * 1_000_000.0);

    Ok(WasmThroughputResult {
        mbps,
        bytes: total_bytes,
        duration_secs,
    })
}

// Measure upload speed
async fn measure_upload_js(
    on_progress: &Option<Function>,
) -> Result<WasmThroughputResult, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let upload_size = 10_000_000usize; // 10MB

    // Generate random data
    let data = generate_random_data(upload_size);
    let array = Uint8Array::from(&data[..]);

    let start = js_sys::Date::now();

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    // Convert Uint8Array to JsValue for body
    let body_value: JsValue = array.into();
    opts.set_body(&body_value);

    let request = Request::new_with_str_and_init("https://speed.cloudflare.com/__up", &opts)?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let _: Response = resp_value.dyn_into()?;

    let end = js_sys::Date::now();
    let duration_secs = (end - start) / 1000.0;
    let mbps = (upload_size as f64 * 8.0) / (duration_secs * 1_000_000.0);

    // Simulate progress for upload (we don't get streaming progress on upload)
    if let Some(cb) = on_progress {
        let _ = cb.call3(
            &JsValue::NULL,
            &JsValue::from_str("upload"),
            &JsValue::from_f64(mbps),
            &JsValue::from_f64(1.0),
        );
    }

    Ok(WasmThroughputResult {
        mbps,
        bytes: upload_size as u64,
        duration_secs,
    })
}

// Measure packet loss
async fn measure_packet_loss_js() -> Result<f64, JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let timeout_ms = 2000.0;
    let mut lost = 0;
    let total = 20;

    for _ in 0..total {
        let controller = web_sys::AbortController::new()?;
        let signal = controller.signal();

        let opts = RequestInit::new();
        opts.set_signal(Some(&signal));

        let request =
            Request::new_with_str_and_init("https://speed.cloudflare.com/__down?bytes=0", &opts)?;
        
        // Set timeout - keep closure alive
        let timeout_callback = Closure::once_into_js(move || {
            controller.abort();
        });

        let timeout_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_callback.as_ref().unchecked_ref(),
                timeout_ms as i32,
            )
            .map_err(|_| JsValue::from_str("Failed to set timeout"))?;
        
        let result = JsFuture::from(window.fetch_with_request(&request)).await;
        let _ = window.clear_timeout_with_handle(timeout_id);

        if result.is_err() {
            lost += 1;
        }
    }

    Ok((lost as f64 / total as f64) * 100.0)
}

// Main speed test function
#[wasm_bindgen(js_name = runSpeedTest)]
pub async fn run_speed_test_wasm(js_callback: JsValue) -> Result<JsValue, JsValue> {
    let callback = if js_callback.is_null() || js_callback.is_undefined() {
        None
    } else {
        js_callback.dyn_into::<Function>().ok()
    };

    // Phase 1: Latency
    if let Some(cb) = &callback {
        let _ = cb.call4(
            &JsValue::NULL,
            &JsValue::from_str("latency"),
            &JsValue::NULL,
            &JsValue::from_f64(0.0),
            &JsValue::NULL,
        );
    }

    let (latency, server) = measure_latency_js().await?;
    
    if let Some(cb) = &callback {
        let _ = cb.call4(
            &JsValue::NULL,
            &JsValue::from_str("latency"),
            &JsValue::NULL,
            &JsValue::from_f64(1.0),
            &JsValue::from_f64(latency.avg_ms),
        );
    }

    // Phase 2: Download
    let download = measure_download_js(&callback).await?;

    // Phase 3: Upload
    if let Some(cb) = &callback {
        let _ = cb.call4(
            &JsValue::NULL,
            &JsValue::from_str("upload"),
            &JsValue::NULL,
            &JsValue::from_f64(0.0),
            &JsValue::NULL,
        );
    }

    let upload = measure_upload_js(&callback).await?;
    
    // Phase 4: Packet Loss
    if let Some(cb) = &callback {
        let _ = cb.call4(
            &JsValue::NULL,
            &JsValue::from_str("packet_loss"),
            &JsValue::NULL,
            &JsValue::from_f64(0.0),
            &JsValue::NULL,
        );
    }

    let packet_loss = measure_packet_loss_js().await?;
    
    // Complete
    if let Some(cb) = &callback {
        let _ = cb.call4(
            &JsValue::NULL,
            &JsValue::from_str("done"),
            &JsValue::NULL,
            &JsValue::from_f64(1.0),
            &JsValue::NULL,
        );
    }

    let result = WasmSpeedTestResult {
        server_location: server,
        latency: Some(latency),
        download: Some(download),
        upload: Some(upload),
        packet_loss: Some(packet_loss),
    };

    to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}
