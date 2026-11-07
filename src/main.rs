use std::io::Result;

fn main() -> Result<()> {
    let first_arg = std::env::args().skip(1).next();

    let mut path = first_arg.map(|x| std::path::PathBuf::from(x)).unwrap_or(std::env::current_dir()?);

    let mut segments = Vec::new();

    let mut is_home = false;
    while let Some(parent) = path.parent() {
        if Some(&path) == dirs::home_dir().as_ref() {
            segments.push(("~".to_string(), false));
            is_home = true;
            break;
        }

        let dir = path.file_name().unwrap().to_str().expect("Invalid utf-8 in path");

        let mut largest_shared_prefix_len = 0;
        for other_path in std::fs::read_dir(&parent)?.filter_map(|x| x.ok()).map(|x| x.file_name()) {
            let other: &str = match other_path.to_str() {
                Some(x) => x,
                None => continue,
            };

            if other == dir { continue; }

            let mut shared_prefix_len = 0;

            for (ch1, ch2) in dir.chars().zip(other.chars()) {
                if ch1 == ch2 {
                    shared_prefix_len += 1;
                } else {
                    break;
                }
            }

            largest_shared_prefix_len = largest_shared_prefix_len.max(shared_prefix_len);
        }

        if largest_shared_prefix_len >= dir.chars().count() - 2 {
            segments.push((dir.to_string(), false));
        } else {
            let segment = dir.chars().take(largest_shared_prefix_len + 1).collect::<String>();
            segments.push((segment, true));
        }
        path = parent.to_path_buf();
    }

    let mut res = std::path::PathBuf::new();
    if !is_home {
        res.push("/");
    }
    for (segment, is_prefix) in segments.into_iter().rev() {
        if is_prefix {
            res.push(segment + "…");
        } else {
            res.push(segment);
        }
    }

    if res.parent() == None {
        print!("/");
    } else {
        print!("{}/", res.to_str().unwrap());
    }

    Ok(())
}
