import numpy as np
import re
import os
import socket
import traceback
import logging

import tensorflow
from tensorflow.keras.preprocessing.sequence import pad_sequences
from tensorflow.keras.preprocessing.text import one_hot

import nltk
from nltk.corpus import stopwords
from nltk.stem.porter import PorterStemmer

def recvall(sock):
    BUFF_SIZE = 4096 # 4 KiB
    data = b''
    while True:
        part = sock.recv(BUFF_SIZE)
        data += part
        if len(part) < BUFF_SIZE:
            # either 0 or end of data
            break
    return data

nltk.download("stopwords")

VOCAB = 10000
PAD_LENGTH=25

model = tensorflow.keras.models.load_model("model.h5")

if os.path.exists("/tmp/tensortalk-accurate.s"):
  os.remove("/tmp/tensortalk-accurate.s")

server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
server.bind("/tmp/tensortalk-accurate.s")

try:
    while True:
        server.listen(1)
        conn, addr = server.accept()
        text = str(recvall(conn))

        print("text: " + text)

        ps = PorterStemmer()

        review = re.sub('\W', ' ', text)
        review = ' '.join([ps.stem(word) for word in review.lower().split() if not word in stopwords.words('english')])
        corpus = [review]

        one_hot_data = [one_hot(words,VOCAB) for words in corpus] 
        xs = pad_sequences(one_hot_data,padding='pre', maxlen=PAD_LENGTH)
        xs = np.array(xs)

        result = model.predict(xs)

        print("result: " + str(result))
        conn.send(str.encode(str(result).replace("[", "").replace("]", "")))
        conn.close()
except Exception as e:
    print("exception occured, continuing as normal")
    print(traceback.format_exc())

