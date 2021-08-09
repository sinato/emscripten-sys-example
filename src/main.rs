use emscripten_sys::{
    emscripten_fetch, emscripten_fetch_attr_init, emscripten_fetch_attr_t, emscripten_fetch_close,
    emscripten_fetch_t,
};

fn body_string(fetch: &emscripten_fetch_t) -> String {
    let data = unsafe { std::mem::transmute::<*const i8, *mut u8>((*fetch).data) };
    let len = (*fetch).totalBytes as usize;
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    let mut v = Vec::with_capacity(len);
    v.resize(len, 0);
    v.copy_from_slice(slice);
    String::from_utf8(v).ok().unwrap()
}

fn print_json(fetch: &emscripten_fetch_t) {
    let body = body_string(fetch);
    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(obj) => {
            println!("{}", obj["a"]);
        }
        Err(e) => {
            println!("error: line: {}, column: {}", e.line(), e.column());
        }
    }
}

extern "C" fn handle_success(fetch: *mut emscripten_fetch_t) {
    unsafe {
        print_json(&*fetch);
        emscripten_fetch_close(fetch);
    }
}

extern "C" fn handle_error(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("error: status code {}", (*fetch).status);
        emscripten_fetch_close(fetch);
    }
}

fn main() {
    unsafe {
        let mut fetch_arg: emscripten_fetch_attr_t = std::mem::uninitialized();
        emscripten_fetch_attr_init(&mut fetch_arg);
        fetch_arg.attributes = 1;
        fetch_arg.onsuccess = Some(handle_success);
        fetch_arg.onerror = Some(handle_error);
        let url = std::ffi::CString::new("data.json").unwrap();
        emscripten_fetch(&mut fetch_arg, url.as_ptr());
    }
}
