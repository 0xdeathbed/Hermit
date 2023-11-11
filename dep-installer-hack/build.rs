
fn main() {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {
        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("software-properties-common").status().expect("Failed to install apt tools").success() {
        }

        if !std::process::Command::new("add-apt-repository")
            .arg("ppa:tomtomtom/yt-dlp").status().expect("Failed to add repo").success() {
        }

        if !std::process::Command::new("apt")
            .arg("update").status().expect("Failed to update package list").success() {
        }

        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("libopus-dev") // the apt package that a dependency of my project needs to compile
            .arg("ffmpeg")
            .arg("yt-dlp")
            // can add more here
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }
    }
}
