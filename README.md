![image](https://user-images.githubusercontent.com/6343630/228673532-1777e0bf-eb28-4381-8ae8-093a1688b524.png)

### Description
OctoScraper is a web multithread web scraper tool implemented in Rust. Currenlty it clones images from websites (but it is configurable).

### Run it
Allow reqwest crate to work propertly:
```
sudo apt install libssl-dev
```

Compile and run application with default configuration (for testing purposes):
```
cargo run
```

or run application with your configuration:
```
cargo run -- -w dodu.it -e png,PNG -d DIRECTORY_NAME -s 100 -t 90000 -i true -l 3
```
where 
|argument|meaning|example|
--------------------------
|-w|website - without http and www prefix. | dodu.it|
|-e|list of extensions separated by comma. | .png,.PNG,.jpg,.JPG,.jpeg,.JPEG|
|-d|directory where files will be saved. | Images|
|-s|sleep time in millis before making the request. | 1000|
|-t|download timeout. | 90000|
|-i|insistent mode (it retries until download succed). | true|
|-l|download limit (by default it makes as much requests as possibile). | 3|


```
    ___     _        __                                
    /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
   //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
  / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
  \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                       |_|              
```    


Tested on Linux and MacOS.
