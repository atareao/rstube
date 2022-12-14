use tokio::process::Command;

pub struct Ytdlp{
    path: String,
    cookies: String,
}

impl Ytdlp {
    pub fn new(path: &str, cookies: &str) -> Self{
        Self{
            path: path.to_string(),
            cookies: cookies.to_string(),
        }
    }
    pub async fn download(&self, id: &str, output: &str) -> std::process::ExitStatus{
        let url = format!("https://www.youtube.com/watch?v={}", id);
        let mut args = vec!["-f", "ba", "-x", "--audio-format", "mp3", 
            "-o", output];
        if &self.cookies != ""{
            args.push("--cookies");
            args.push(&self.cookies);
        }
        args.push(&url);
        Command::new(&self.path)
            .args(&args)
            .spawn()
            .expect("ytdlp can not start")
            .wait()
            .await
            .expect("ytdlp failed to run")
    }
}

#[tokio::test]
async fn test_ytdlp(){
    let ytdlp = Ytdlp::new("yt-dlp", "cookies.txt");
    let salida = ytdlp.download("mWoJw5qD0eI", "/tmp/test.mp3").await;
    println!("{:?}", salida);
    assert!(true);
}



