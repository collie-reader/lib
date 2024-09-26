pub mod service {
    pub mod feed;
    pub mod item;
}

pub mod model {
    pub mod feed;
    pub mod item;
    pub mod syndication;
}

pub mod repository {
    pub mod database;
    pub mod feed;
    pub mod item;
}

pub mod util {
    pub mod fetcher;
}

pub mod worker;

pub mod error;

#[cfg(test)]
mod tests {
    mod syndication;
}
