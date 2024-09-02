use std::path::PathBuf;

pub fn sanitize_path(req_path: &str, base_path: &str) -> Option<String> {
    let req_path = req_path.trim_start_matches('/');

    let mut path = PathBuf::from(base_path);

    let base_path = match PathBuf::from(base_path).canonicalize() {
        Ok(path) => path,
        Err(err) => {
            println!("Error canonicalizing base_path.\nError: {}", err);
            return None;
        }
    };

    path.push(req_path);

    if path.is_dir() {
        path.push("index.html");
    } else if path.extension().is_none() {
        path.set_extension("html");
    }

    return match path.canonicalize() {
        Ok(path) => {
            if path.starts_with(base_path) {
                Some(path.to_str()?.to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    };
}
