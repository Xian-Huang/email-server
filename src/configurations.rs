use config::ConfigError;
use serde;



#[derive(serde::Deserialize)]
pub struct Settings{
    pub database:DatabaseSettings,
    pub application_port:u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username:String,
    pub password:String,
    pub port:u16,
    pub host:String,
    pub dbname:String
}


pub fn get_config()->Result<Settings,ConfigError>{
    let settings = config::Config::builder().add_source(config::File::new("configurations.yaml",config::FileFormat::Yaml)).build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connext_string(&self)->String{
        format!("mysql://{}:{}@{}:{}/{}",self.username,self.password,self.host,self.port,self.dbname)
    }
}
