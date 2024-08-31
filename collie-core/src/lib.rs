pub mod model {
    pub mod database;
    pub mod feed;
    pub mod item;
}

pub mod producer {
    pub mod syndication;
    pub mod worker;
}

pub mod error;

#[cfg(test)]
mod tests {
    mod syndication;
}
