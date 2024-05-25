cargo +stable-i686-pc-windows-msvc build --release
mkdir bin
copy target\release\samp_logger.dll bin\samp_logger.dll