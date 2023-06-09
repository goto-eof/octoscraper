```
        ___     _        __                                
        /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
       //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
      / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
      \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                           |_|              
```    
### Description
OctoScraper is a multithread web scraper tool implemented in Rust.

### Execute it
Download the executable from [here](https://github.com/goto-eof/octoscraper/releases) and run it.
```
./octoscraper -w http://dodu.it -e .png,.PNG -d DIRECTORY_NAME -s 100 -t 90000 -i true -l 3 -a OctoScraper
```

### Examples
Download midi and mp3 files, no same domain, scan all website
```
./octoscraper -w http://audiomidimania.com  -oa true -sd false -r false
```
Download midi and mp3 files, same domain, only from the page passed as parameter 
```
./octoscraper -w http://ininternet.org/midi_file.htm -oa true -sd true -r true
```
Download image files, no same domain
```
./octoscraper -w https://wallpaper.mob.org/ -oi true -sd false -si 1000000
```
Download video, process only the page passed as parameter
```
./octoscraper -w http://www.w3schools.com/html/html5_video.asp -ov true -r true
```

where 

 | argument | meaning                                                            | value example                  |
 -----------|--------------------------------------------------------------------|--------------------------|
 | -h       |Help                                                                |
 | -w       |website - with http/https                     |  http://dodu.it or http://audiomidimania.com or http://wallpaper.mob.org or http://www.w3schools.com/html/html5_video.asp|
 | -sd      |same domain                                                         |  true|
 |   -oi    | enable image extractor                                            | true |
 |   -ov    | enable video extractor                                            | true |
 |   -oa    |  enable audio extractor                                            | true |
 |   -oo    |  enable other file extractor                                            | true |
 |  -si     |   minimum image file size (in bytes)                                    |1000000                   |
 |  -sv     |   minimum video file size (in bytes)                                    |1000000                   |
 |  -sa     |   minimum audio file size (in bytes)                                    |1000000                   |
 |  -so     |   minimum other file size (in bytes)                                    |1000000                   |
 |   -ei	|list of image extensions separated by comma                        | .jpg,.JPG,.png,.PNG |
 |   -ev	| list of video extensions separated by comma                       | .ogg,.OGG,.MP4,.mp4 |
 |   -ea	| list of audio extensions separated by comma                      | .mp3,.MP3,.midi,.MIDI |
 |   -eo	| list of other file extensions separated by comma                      | .zip,.ZIP,.exe,.EXE,.pdf,.PDF |
 | -d       |directory where files will be saved                                 | Images|
 | -s       |sleep time in millis before making the request                      | 1000|
 | -t       |download timeout                                                    | 90000|
 | -i       |insistent mode (it retries until download succeed)                  | true|
 | -l       |download limit (by default it makes as much requests as possibile)  | 3|
 | -a       |user agent                                                          | OctoScraper|
 | -c       |enables downloaded file hash check for avoiding duplicate downloads | true|
 |-r        | process only the root link (process only one page)                                         | false|
 | -u       | consider unique resources by filename (1) or by link (2). Allowed values: 1 or 2| 1 |


### For developers

Allow reqwest crate to work properly:
```
sudo apt install libssl-dev
```
Run application with your configuration:
```
cargo run -- -w http://dodu.it -e .png,.PNG -d DIRECTORY_NAME -s 100 -t 90000 -i true -l 3 -a OctoScraper
```

### Tests
```
cargo test
```

### Screenshot
![image](https://user-images.githubusercontent.com/6343630/229301705-de5ac86a-d44e-4e8b-99eb-99e30bc17296.png)





Tested on Linux, MacOS and Windows.

if any problems arise, feel free to [contact me](http://andre-i.eu/#contactme).
