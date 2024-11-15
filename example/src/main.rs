use injecto::module;

struct Client;

impl Client {
    fn new() -> Self {
        Self {}
    }

    pub fn get_name(&self) -> String {
        String::from("Client")
    }
}

struct Repositiry {
    client: Client,
}

impl Repositiry {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn get_name(&self) -> String {
        self.client.get_name()
    }
}

struct Service {
    repo: Repositiry,
}

impl Service {
    fn new(repo: Repositiry) -> Self {
        Self { repo }
    }

    fn get_name(&self) -> String {
        self.repo.get_name()
    }
}

pub trait Testt {
    fn test(&self) -> u32;
}

module! {
    pub AppModule {
        #[injectable]
        fn client() -> Client {
            Client::new()
        }

        #[injectable]
        fn repository(client: Client) -> Repositiry {
            Repositiry::new(client)
        }

        #[injectable]
        fn service(repository: Repositiry) -> Service {
            Service::new(repository)
        }
    }
}

fn main() {
    let app = AppModule::new();

    // app.test();

    println!("{:?}", app,);
}
