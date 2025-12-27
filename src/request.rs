struct Member {
    uid: String
}

mod request {

    pub struct DraftedRequest {
        pub name: String,
        pub description: String,
    }

    impl DraftedRequest {
        fn new(name: String, description: String) -> DraftedRequest {
            DraftedRequest {
                name,
                description
            }
        }

        fn advance(&self, uid: String) -> Request {
            Request {
                name: self.name,
                description: self.description,
                uid: uid
            }
        }
    }

    pub struct Request {
        pub name: String,
        pub description: String,
        pub uid: String
    }

    
}