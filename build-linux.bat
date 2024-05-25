docker build -t samp_logger .
docker run --rm --name samp_logger -it -d samp_logger
mkdir bin
docker cp samp_logger:/app/samp_logger.so bin/samp_logger.so
docker stop samp_logger
docker image rm samp_logger