use std::{ffi::CStr, os::raw::c_char, usize};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Params {
    radius: usize,
    iterations: usize,
}

/// SAFETY
/// rgba_data must be a valid pointer
/// params must be a valid null-terminated C string
/// pointers must not be null
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    println!("Blur plugin called: {width}x{height}");
    let parsed = (|| {
        if params.is_null() {
            return None;
        }
        // SAFETY,
        // params should be not null, can be empty string
        let c_str = unsafe { CStr::from_ptr(params) };
        let json = c_str.to_str().ok()?;
        serde_json::from_str(json).ok()
    })();

    let params = parsed.unwrap_or(Params {
        radius: 1,
        iterations: 1,
    });

    println!("Parsed params: {:?}", params);

    // SAFETY, length should be calculated in right way
    let len = match (width as usize)
        .checked_mul(height as usize)
        .and_then(|res| res.checked_mul(4))
    {
        Some(v) => v,
        None => {
            println!("Image too large");
            return;
        }
    };

    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };
    let radius: isize = match params.radius.try_into() {
        Ok(v) => v,
        Err(_) => {
            println!("Radius too large");
            return;
        }
    };

    for _ in 0..params.iterations {
        blur(data, height as usize, width as usize, radius);
    }
}

fn blur(data: &mut [u8], height: usize, width: usize, radius: isize) {
    let mut new_data = data.to_vec();

    for row in 0..height {
        for column in 0..width {
            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut weight_sum = 0.0;

            for dy in -radius..radius {
                for dx in -radius..radius {
                    let neighbour_x = column as isize + dx;
                    let neighbour_y = row as isize + dy;

                    if neighbour_x < 0
                        || neighbour_y < 0
                        || neighbour_x >= width as isize
                        || neighbour_y >= height as isize
                    {
                        continue;
                    }

                    let dist =
                        ((neighbour_x * neighbour_x + neighbour_y * neighbour_y) as f32).sqrt();
                    let weight = 1.0 / (1.0 + dist);

                    let idx = (neighbour_y as usize * width + neighbour_x as usize) * 4;
                    r_sum += data[idx] as f32 * weight;
                    g_sum += data[idx + 1] as f32 * weight;
                    b_sum += data[idx + 2] as f32 * weight;
                    weight_sum += weight;
                }
            }

            let idx = (row * width + column) * 4;
            new_data[idx] = (r_sum / weight_sum) as u8;
            new_data[idx + 1] = (g_sum / weight_sum) as u8;
            new_data[idx + 2] = (b_sum / weight_sum) as u8;
        }
    }
    data.copy_from_slice(&new_data);
}
