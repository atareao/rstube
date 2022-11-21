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
        for user in self.users{
            if username == user.username{
                return Some(&user);
            }
        }
        None
    }
}
