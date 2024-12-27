#!/bin/bash
BACKTESTER_PATH=Backtester/examples/data
curl -L -o ${BACKTESTER_PATH}/bitcoin-historical-data.zip\
  https://www.kaggle.com/api/v1/datasets/download/mczielinski/bitcoin-historical-data

unzip ${BACKTESTER_PATH}/bitcoin-historical-data.zip -d ${BACKTESTER_PATH}
rm ${BACKTESTER_PATH}/bitcoin-historical-data.zip
