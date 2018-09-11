# Kagura_suzu_bot: a tiny program to post a tween in specified interval

Supported OS
- Ubuntu 18.40
- OSX high sierra 10.13.6
- Also checked on bash on windows

This project aims to create a tiny program to post a tweet in a specified interval via twitter API. Current implementation depends on R and Rust language. If interested in running your own "suzu-kagura-bot," please install R and Rust firstly.


## How To Use

First of all, one should install "twitteR" package for R. After that, prepare an authentification file to use Twitter API correctly.

Specifically, prepare a UTF-8 encoded file named ./data/authorization.csv which has a header for four column like the following:

consumer_key | consumer_secret | access_token | access_secret |
-------------|-----------------|--------------|---------------|
*********    |***********      |***********   |********       |


and make sure that
- Rust language has been installed on the envirnment,
- R langauge and "twitteR" package have been installed,
- Authentification in Twitter development site has been done correctly, and
- the ./data/authorization.csv file has been created with nessesary fields.

Then, now just type

```
bash execute.sh &
```

and this script sent a tweet to your account in 30 min interval as a background process.



## How to add a tweet.

There are already some files for tweet in ./data/, such as "Mison_de_Maoh_1.toml".
Of course, any support for these annotation would be welcome.

However, there are some rules for annotation so that
- a irrelevant post would never be created,
- convertion from toml file to tweet be easy.

The syntax is as follows:


First, the filename should end with ".toml".
In the toml file, two parameters should be put in the first and second lines.
More specifically, one should write `title = TITLE` and `url = URL` in the beginning of the file,
where TITLE and URL are the title of the video and the url(not ur**I**) of the video, respectively.
```toml
title = TITLE
url = URL
```

In the following section, one can write scene description as much as one wants.
```toml
[[scenes]]
script = SCRIPT
start_time = START_TIME
```
where SCRIPT is the script Suzu-san said, and START_TIME is a string
to identify when Suzu-san said that script.


Here is a example,
```toml
title = "【メゾンド魔王】生活をかけた世界征服雑談【アイドル部】"
url = "https://youtu.be/gQ0GaN5zI4k"

[[scenes]]
script = "「愛を語らないでくださいね、戦争中、ですから」"
start_time = "26m42s"

[[scenes]]
script = "「キヒヒヒッ」"
start_time = "13m37s"
```
Please be aware that one single file corresponds to one single YouTube video.

If one need to see more examples, please `cat ./data/*.toml`.


