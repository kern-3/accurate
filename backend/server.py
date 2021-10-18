from flask import render_template, jsonify, Flask, request


import numpy as np
import re

import tensorflow
from tensorflow.keras.preprocessing.sequence import pad_sequences
from tensorflow.keras.preprocessing.text import one_hot

import nltk
from nltk.corpus import stopwords
from nltk.stem.porter import PorterStemmer


nltk.download("stopwords")

VOCAB = 10000
PAD_LENGTH=25


model = tensorflow.keras.models.load_model("backend/model.h5")



app = Flask(__name__)



@app.route('/')
def main():
    return render_template("index.html")


@app.route("/predict")
def predict():

    text = request.args.get("text")\

    ps = PorterStemmer()

    review = re.sub('\W', ' ', text)
    review = ' '.join([ps.stem(word) for word in review.lower().split() if not word in stopwords.words('english')])
    corpus = [review]


    
    one_hot_data=[one_hot(words,VOCAB) for words in corpus] 
    xs=pad_sequences(one_hot_data,padding='pre',maxlen=PAD_LENGTH)
    xs=np.array(xs)

    result = model.predict(xs)

    return jsonify(str(result[0][0]))
if __name__ == '__main__':
    app.run(host='0.0.0.0', port=105)