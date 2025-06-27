use regex::Regex;
use std::path::PathBuf;
use std::env;

pub fn untildify(input_path: &str) -> String {
    if input_path.is_empty() {
        return String::from(input_path);
    }

    // Get home directory
    let home_dir = if cfg!(windows) {
        env::var("USERPROFILE").ok()
    } else {
        env::var("HOME").ok()
    }.map(PathBuf::from);

    if let Some(home_dir) = home_dir {
        if let Some(home_str) = home_dir.to_str() {
            // Handle different tilde patterns
            if input_path == "~" {
                return home_str.to_string();
            } else if input_path.starts_with("~/") || input_path.starts_with("~\\") {
                // Replace tilde with home directory
                let normalized = input_path.replacen('~', home_str, 1);

                // Normalize path separators for Windows
                if cfg!(windows) {
                    return normalized.replace('/', "\\");
                }
                return normalized;
            }
        }
    }

    input_path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_returns_untildfyed_string() {
        // Set up test environment
        let home = if cfg!(windows) {
            Path::new("C:\\Users\\testuser")
        } else {
            Path::new("/home/testuser")
        };

        let home_env = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        env::set_var(home_env, home.as_os_str());

        // Test cases
        assert_eq!(
            untildify("~/Desktop"),
            if cfg!(windows) {
                "C:\\Users\\testuser\\Desktop"
            } else {
                "/home/testuser/Desktop"
            }
        );

        assert_eq!(
            untildify("~/a/b/c/d/e"),
            if cfg!(windows) {
                "C:\\Users\\testuser\\a\\b\\c\\d\\e"
            } else {
                "/home/testuser/a/b/c/d/e"
            }
        );

        assert_eq!(
            untildify("~/"),
            if cfg!(windows) {
                "C:\\Users\\testuser\\"
            } else {
                "/home/testuser/"
            }
        );

        assert_eq!(
            untildify("~"),
            if cfg!(windows) {
                "C:\\Users\\testuser"
            } else {
                "/home/testuser"
            }
        );
    }

    #[test]
    fn test_handles_windows_paths() {
        if !cfg!(windows) { return; }

        env::set_var("USERPROFILE", "C:\\Users\\testuser");

        assert_eq!(untildify("~\\Documents"), "C:\\Users\\testuser\\Documents");
        assert_eq!(untildify("~/Documents"), "C:\\Users\\testuser\\Documents");
        assert_eq!(untildify("~"), "C:\\Users\\testuser");
        assert_eq!(untildify("~/"), "C:\\Users\\testuser\\");
    }

    #[test]
    fn test_returns_original_path() {
        assert_eq!(untildify("Desktop"), "Desktop");
        assert_eq!(untildify(""), "");
        assert_eq!(untildify("/"), "/");
        assert_eq!(untildify("C:\\Windows"), "C:\\Windows");
        assert_eq!(untildify("~unknown"), "~unknown");
    }
}
