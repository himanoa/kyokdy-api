pub struct Config {
    pub db_url: &String
}

pub trait Application<T> where T: Self::Sized + Clone + Send + Sync,
{
    
}
