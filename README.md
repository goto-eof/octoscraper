```
    ___     _        __                                
    /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
   //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
  / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
  \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                       |_|              
```    
### Description
OctoScraper is a multithread web scraper tool implemented in Rust. It clones images from websites.

### Execute it
Download the executable from [here](https://github.com/goto-eof/octoscraper/releases) and run it.
```
./octoscraper -w dodu.it -e .png,.PNG -d DIRECTORY_NAME -s 100 -t 90000 -i true -l 3 -a OctoScraper
```



### For developers

Allow reqwest crate to work properly:
```
sudo apt install libssl-dev
```
Run application with your configuration:
```
cargo run -- -w dodu.it -e .png,.PNG -d DIRECTORY_NAME -s 100 -t 90000 -i true -l 3 -a OctoScraper
```
where 

 | argument | meaning                                                            | value example                  |
 -----------|--------------------------------------------------------------------|--------------------------|
 | -h       |Help                                                                |
 | -w       |website - without http and www prefix                               |  dodu.it|
 | -e       |list of extensions separated by comma (only image extentions allowed)                               | .png,.PNG,.jpg,.JPG,.jpeg,.JPEG|
 | -d       |directory where files will be saved                                 | Images|
 | -s       |sleep time in millis before making the request                      | 1000|
 | -t       |download timeout                                                    | 90000|
 | -i       |insistent mode (it retries until download succeed)                  | true|
 | -l       |download limit (by default it makes as much requests as possibile)  | 3|
 | -a       |user agent                                                          | OctoScraper|
 | -c       |enables downloaded file hash check for avoiding duplicate downloads | true|


### Screenshot
![image](https://user-images.githubusercontent.com/6343630/228950041-c8621873-7757-4c68-953b-7a0fbe4b9389.png)





Tested on Linux and MacOS.

if any problems arise, feel free to [contact me](https://andre-i.dev/#contactme).
