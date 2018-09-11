# Kagura_suzu_bot: a tiny program to post a tween in specified interval

## Synopsis

- Prepare ./data/authorization.csv which has a header for four column like this.

consumer_key | consumer_secret | access_token | access_secret |
-------------|-----------------|--------------|---------------|
*********    |***********      |***********   |********       |

- Prepare ./data/SOME_FILE_NAME.toml which is like following:


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

- Execute `bash execute.sh`

