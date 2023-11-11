
fn main() {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
    {


        if !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("libopus-dev") // the apt package that a dependency of my project needs to compile
            .arg("ffmpeg")
            .arg("curl")
            // can add more here
            .status()
            .expect("failed to run apt")
            .success()
        {
            panic!("failed to install dependencies")
        }

        if !std::process::Command::new("curl")
            .arg("-L")
            .arg("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp")
            .arg("-o")
            .arg("/usr/local/bin/yt-dlp").status().expect("Failed to run curl").success() {

            panic!("Failed to download YT-dlp");
        }

        if !std::process::Command::new("chmod")
            .arg("a+rx")
            .arg("/usr/local/bin/yt-dlp").status().expect("Failed to run chmod").success() {

            panic!("Failed to make yt-dlp executable");
        }
    }
}
