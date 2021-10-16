use super::source::Source;

pub struct Post {
    pub url: String,
    pub contense: String,
    pub author: String,
    pub hashtags: Vec<String>,
    pub comments: u32,
    pub reposts: u32,
    pub likes: u32,

    pub associated_sources: Vec<Source>,
    pub associated_posts: Vec<Post>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            url: "".to_owned(),
            contense: "".to_owned(),
            author: "".to_owned(),
            hashtags: vec![],
            comments: 0,
            reposts: 0,
            likes: 0,
            
            associated_sources: vec![],
            associated_posts: vec![],
        }
    }
}

/// Processes a post. Returns the contense of the post, along with
/// related posts
pub async fn process_post(url: String) -> Post {
    let body = super::download_body(url).await.unwrap();
    let document = select::document::Document::from(body.as_str());

    for post in document.find(select::predicate::Class("css-901oao css-16my406 r-poiln3 r-bcqeeo r-qvutc0")) {
        println!("{:?}", post);
    }

    Post::default()
}

