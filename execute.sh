#!/bin/bash
## Post a tweet in 30 min interval
# while true
# do
    # sleep 30m
    cargo run --release -- ./data/
    Rscript --vanilla ./src/post_random_tweet.R ./data/authorization.csv ./data/tweets.csv
# done                             
