use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::{Serialize, Deserialize};
use serde_yaml::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub username: String,
    pub password: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    pub log_level: String,
    pub port: String,
    pub users: Vec<User>,
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }

    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_port(&self) -> &str{
        &self.port
    }

    pub fn get_user(&self, username: &str) -> Option<&User>{
        for user in self.users.as_slice(){
            if username == user.username{
                return Some(&user);
            }
        }
        None
    }

    pub fn check_basic_auth(&self, basic_auth: &BasicAuth) -> bool{
        let username = basic_auth.user_id();
        match basic_auth.password(){
            Some(p) => self.check_user(username, p),
            None => false,
        }
    }

    pub fn check_user(&self, username: &str, password: &str) -> bool{
        for user in self.users.as_slice(){
            if username == user.username &&
                    md5_hash(password) == user.password{
                return true;
            }
        }
        false
    }
}

fn md5_hash(to_hash: &str) -> String{
    let digest = md5::compute(to_hash);
    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use crate::config::md5_hash;

    #[test]
    fn it_works() {
        let sample = "sample to digest";
        let hash = md5_hash(sample);
        println!("{}", hash);
        assert_eq!(hash, "c0a5b64fb550f35d255f2033fdc57419");
    }
}
