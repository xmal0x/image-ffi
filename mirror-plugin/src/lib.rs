use std::{ffi::CStr, os::raw::c_char};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Params {
    mode: String,
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
    println!("Mirror plugin called: {width}x{height}");
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
        mode: "vertical".to_string(),
    });

    println!("Parsed params: {:?}", params);

    // SAFETY, length should be calculated in right way
    let len = match (width as usize)
        .checked_mul(height as usize)
        .and_then(|res| res.checked_mul(4))
    {
        Some(v) => v,
        None => {
            println!("Imgage too large");
            return;
        }
    };

    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    let row_length = (width * 4) as usize;
    let rows_count = height as usize;

    if params.mode == "vertical" {
        vertical(data, row_length, rows_count);
    } else {
        horizontal(data, row_length, rows_count, width as usize);
    }
}

fn vertical(data: &mut [u8], row_length: usize, rows_count: usize) {
    for r in 0..(rows_count as usize / 2) {
        let up_row_start = r * row_length;
        let down_row_start = (rows_count as usize - 1 - r) * row_length;

        let (top, bottom) = data.split_at_mut(down_row_start);

        let up_row = &mut top[up_row_start..up_row_start + row_length];
        let down_row = &mut bottom[..row_length];

        up_row.swap_with_slice(down_row);
    }
}

fn horizontal(data: &mut [u8], row_length: usize, rows_count: usize, width: usize) {
    for row_num in 0..rows_count {
        let row_start = row_num * row_length;
        let row = &mut data[row_start..row_start + row_length];

        for p in 0..(width / 2) {
            let left = p * 4;
            let right = (width - 1 - p) * 4;

            for i in 0..4 {
                row.swap(left + i, right + i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn test_horizontal() {
        let width = 3;
        let height = 1;

        let mut data: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let expected: [u8; 12] = [9, 10, 11, 12, 5, 6, 7, 8, 1, 2, 3, 4];

        let params = CString::new(r#"{"mode":"horizontal"}"#).unwrap();

        unsafe {
            process_image(width, height, data.as_mut_ptr(), params.as_ptr());
        }

        assert_eq!(data, expected);
    }

    #[test]
    fn test_vertical() {
        let width = 2;
        let height = 2;

        let mut data: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let expected: [u8; 16] = [9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8];

        let params = CString::new(r#"{"mode":"vertical"}"#).unwrap();

        unsafe {
            process_image(width, height, data.as_mut_ptr(), params.as_ptr());
        }
        assert_eq!(data, expected);
    }
}
