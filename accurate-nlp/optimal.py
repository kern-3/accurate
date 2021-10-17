import os
import warnings
import json
import re

from termcolor import colored
import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split




from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense, LSTM, Embedding
from tensorflow.keras.preprocessing.sequence import pad_sequences
from tensorflow.keras.preprocessing.text import one_hot


warnings.filterwarnings("ignore")
os.environ['TF_CPP_MIN_LOG_LEVEL'] = '3' 


print(colored("\n\nImported Tensorflow Packages", "green"))



import nltk
from nltk.corpus import stopwords
from nltk.stem.porter import PorterStemmer

#nltk.download('stopwords')




print(colored("Imported NLTK Packages", "green"))




VOCAB = 10000
PAD_LENGTH=25
EMBEDDING_OUTPUT_DIM=50

df = pd.read_csv("data/train.csv")


print(colored("Finished reading data into dataframe\n", "green"))
df = df.dropna()


X = df.drop(["label"],1)

ys = df["label"]


X = X.reset_index()


ps = PorterStemmer()
corpus = []
for i in range(0, len(X)):

    review = re.sub('\W', ' ', str(X["title"][i]))
    review = ' '.join([ps.stem(word) for word in review.lower().split() if not word in stopwords.words('english')])
    corpus.append(review)

    
    if(i%1000 == 0):
        print ("\033[A                             \033[A")
        print(colored("Train Corpus Progress: " + str(i) + "/" + str(len(X)), "green"))

print ("\033[A                             \033[A")
print(colored("Finished stemming train corpus", "green"))



one_hot_data=[one_hot(words,VOCAB) for words in corpus]


xs=pad_sequences(one_hot_data,padding='pre',maxlen=PAD_LENGTH)

print(colored("One hot encoded and padded all the sequences", "green"))

model=Sequential()
model.add(Embedding(input_dim=VOCAB,output_dim=EMBEDDING_OUTPUT_DIM,input_length=PAD_LENGTH))
model.add(LSTM(100))
model.add(Dense(128, activation="relu"))
model.add(Dense(64, activation="relu"))
model.add(Dense(1, activation='sigmoid'))
model.compile(loss='binary_crossentropy',optimizer='adam',metrics=['accuracy'])

print(colored("Model created", "green"))

xs=np.array(xs)
ys=np.array(ys)



train_xs, test_xs, train_ys, test_ys = train_test_split(xs, ys, test_size=0.3)


history = model.fit(train_xs,train_ys,epochs=3,batch_size=64, verbose=2)

print(colored("Finished training", "green"))

results = model.evaluate(test_xs, test_ys)

print(colored("Finished testing model on test data", "green"))

print(results)

model.save("results/model.h5")
