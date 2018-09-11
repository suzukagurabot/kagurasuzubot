library("twitteR")
args <- commandArgs(trailingOnly = TRUE)
print(args)
args <- c("./data/authorization.csv","./data/tweets.csv")
auth <- read.csv(args[1],header = TRUE,stringsAsFactors=FALSE)
tweets <- scan(file=args[2],what = character(),sep='\n')
setup_twitter_oauth(auth$consumer_key,
                    auth$consumer_secret,
                    auth$access_key,
                    auth$access_secret)
max_error <- 5
attempts <- 0
r <- NULL
while (is.null(r) && attempts<max_error){
    Sys.sleep(10)
    attempts <- attempts + 1
    try(
        r <- tweet(sample(tweets,1))
    )
}
ifelse(is.null(r),
       print(paste0("POST failed after ",max_error,"attempts")),
       1)
