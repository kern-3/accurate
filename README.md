<h1 align="center">
    <img src="https://user-images.githubusercontent.com/63599413/137497571-fe3c2fb7-c2a6-43dc-8dcd-942f6844a187.png" alt="Logo" width="220![image](https://user-images.githubusercontent.com/63599413/137502029-e7c6c8a1-f6c0-4877-9ae6-2404788dc3f1.png)
" height="155">
    <br>
    Accurate
  <br>
</h1>
<p align="center">A tool to try and tell the difference between what is trustworthy, and what is not. Using a combination of Natural Language Processing and basic human psychology, now you have a truth machine at your fingertips!
<br>
</p>


# Building
This project is made of a few different components:
1. Crawler
2. NLP
3. Backend
4. Frontend

## Crawler
This is the peice that grabs social media posts and sources, and bundles them into a file that the NLP software. The crawler is built in Rust, so you can just use the `cargo` utility that we all know and love :)
* Rust
* Asynchronous processing (Tokio + Hyper)
* Web scraping Twitter
* JSON

```bash
cd crawler
cargo build --release

# Find the binary where it normally is (target/release/accurate-crawler)!
# View the help message for invocation.
```

## NLP (natural language processing)
The Natural Language Processing aspect of this project is one of the most critical parts. It should only be needed to run once per group of crawler output data. This is built in Python, so you can run it as normal (but download the dependencies first!).
* Natural Language Sentiment Analysis 
* Python
* Keras
* Lots of homegrown data
```bash
sudo pip3 -r requirements.txt
python3 accurate-nlp.py

# View the help message for invocation.
```

## Backend
The view from the outside world! This peice assembles crawler and NLP data into a final product, which is then accessible via a REST HTTP server. This is also built in Rust, so build it the same way as the crawler!
* Rust
* Asynchronous processing (Tokio + Hyper)
* Data aggregation
* REST architecture
```bash
cd backend
cargo build --release

# Find the binary where it normally is (target/release/accurate-backend)
# View the help message for invocation.
```

## Frontend
Whats the point of a backend without a front end‽ This is hosted on a server that the [accuracy.news](accuracy.news) domain points to. To host it on your local computer, do this!
* HTML
* CSS
* Javascript
* JSON

```bash
<TODO>
```
# Hackathon
This project was created with ❤️ for the _2021 Treasure Hacks Hackathon_.
