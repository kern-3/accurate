<h1 align="center">
    <img src="https://user-images.githubusercontent.com/63599413/137497571-fe3c2fb7-c2a6-43dc-8dcd-942f6844a187.png" alt="Logo" width="220![image](https://user-images.githubusercontent.com/63599413/137502029-e7c6c8a1-f6c0-4877-9ae6-2404788dc3f1.png)
" height="155">
    <br>
    Accurate
  <br>
</h1>

No longer will you be forced to discern truth from fiction! Thanks to NLP and Psychology, now you can sit back, relax, and rest easy knowing <b>Accurate</b>, a tool which is keeping you correctly informed by assisting you in determining what is trustworthy and what is not.
<br>
<p align="center">
  <a href="#inspiration">Inspiration</a> •
  <a href="#what-it-does">What it does</a> •
  <a href="#how-we-built-it">How we built it</a> •
  <a href="#challenges-we-ran-into">Challenges we ran into</a> •
  <a href="#accomplishments-that-were-proud-of">Accomplishments</a> •
  <a href="#what-we-learned">What we learned</a> 
</p>

# Inspiration
We were motivated to create a tool to try to discern the difference between what is trustworthy and what is not after observing the widespread misuse of social media and the spread of fake news via social media. Your Aunt Velma will no longer be able to claim that Bonobos are using 5G to spread poisons into millions of people's drinking water! Doesn't it sound fantastic...?

# What it does
Accurate is a tool that can help you figure out which online sources are the least trustworthy. This is accomplished by gathering information on which sources are related to which social media posts (in this case, Twitter). After this information is gathered, NLP is used to identify basic intent, and the result is skewed by the popularity of the source. Following that, the data would be traced back to the social media site, and some factors might be linked to identifiers (e.g. hashtags).
<!-- ### • Data Collection
A bot will be used to start with one post and then go on to others, collecting information along the way. After that, sources are gathered, and all data is converted to a format that the Data Analyzing engine can understand.
### • Data Analysing
Using Natural Language Processing to first determine a source's purpose, then biassing it with metadata acquired during data collection. Data is looped back to social media, where identifiers (hashtags) are assigned a trustworthiness grade.
### • Data Visualisation
We've also built a web application to enable users to engage with our tool, which provides the user with accuracy of specific posts and hashtags, as well as posts related to the user's post in a visually appealing manner. -->


- **Data Collection:** A bot will be used to start with one post and then go on to others, collecting information along the way. After that, sources are gathered, and all data is converted to a format that the Data Analyzing engine can understand.


- **Data Analysing:** Using Natural Language Processing to first determine a source's purpose, then biassing it with metadata acquired during data collection. Data is looped back to social media, where identifiers (hashtags) are assigned a trustworthiness grade.


- **Data Visualisation:** We've also built a web application to enable users to engage with our tool, which provides the user with accuracy of specific posts and hashtags, as well as posts related to the user's post in a visually appealing manner.

# How we built it
This project is made of a few different components:

- ***Crawler:*** Rust, Asynchronous processing (Tokio + Hyper), Web scraping Twitter, JSON
- ***AI & NLP:*** Natural Language Sentiment Analysis, Python (Tensorflow, Pandas, NLTK), R (ggplot), Lots of homegrown Data
- ***Backend:*** Rust, Asynchronous processing (Tokio + Hyper), Data aggregation, REST Arch.
- ***Frontend:*** HTML, CSS, Javascript, JSON
- ***Pitch Deck:*** Figma

## Cloning repo

```
git clone https://github.com/kern-3/accurate.git
cd accurate/
```

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

* Natural Language Processing 
* Python (Tensorflow, Pandas, NLTK)
* R (ggplot)
* Lots of homegrown data


First, the raw data is analyzed using data science approaches and cleaned so that it can be processed by the model. Then, all the titles are converted using a PorterStemmer and Stopwords to ensure only the most meaningful words are there. The data are one hot encoded and then trained through a Deep Learning model with an Embedding, LSTM, and numerous Dense layers.

```bash
pip install tensorflow termcolor pandas sklearn nltk
py optimal.py

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
Whats the point of a backend without a front end‽ This is hosted on a server that the [accuracy.netlify.app](https://accuracy.netlify.app) domain points to. To host it on your local computer, do this!
* HTML
* CSS
* Javascript
* JSON
* Figma

```bash
python -m pip install tensorflow nltk --user
python backend/server.py
# Open the URL where it's running.
```
# Challenges we ran into
* One of the most significant challenges we faced during the hackathon was a *13-hour power outage* at one of our teammates' location, which prevented us from progressing because we needed data that he had to provide for further predicting the outcomes from that data and aggregating it, as well as connecting the frontend, backend, and model, but it's something no one could avoid.
* Also one of the issues we encountered was obtaining the enormous amount of data we required. We concluded that using already gathered data would be against the spirit of the competition, so we opted to collect our own. We discovered that Web Scraping Twitter was the ideal tool for the job because of a combination of incredibly fast networking rates and a highly parallel processor.

# Accomplishments that we're proud of
* This is our ([kern3’s](https://github.com/kern-3)) first project ! As a result, we're pleased to be capable of doing anything like this. We're also pleased with how we divided the tasks among ourselves which turned out to be very efficient.
* We successfully finished with the model and were able to achieve 92% model accuracy on test data, which we were pleased with given the obstacles and time constraints.
* Pleased with the website's visual design.
* We were ecstatic since we were able to work with a variety of languages and technologies from various fields and link them together to create Accurate.

# What we learned
* We got a lot of experience in how to aggregate data, and how to connect different systems together, especially with so many different languages interacting with each other.
* We all learned a lot about how to interact in reasonably large and complicated projects because this was our first/second hackathon, and we had a great time working in our team.

# What's next for Accurate 
* Given the time (and possible financial backing), we would love to package this technology into a browser extension, for easier, inline, and informed social media interaction.
* We'd also like to expand our reach to various social media sites and check for the accuracy of the information shared there too.


# Hackathon
This project was created with ❤️ for the _2021 Treasure Hacks Hackathon_.
