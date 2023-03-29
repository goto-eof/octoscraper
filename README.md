![image](https://user-images.githubusercontent.com/6343630/228604051-053b9cbf-6d3d-4882-b041-fceb17250bae.png)

### Description
OctoScraper is a web multithread web scraper tool implemented in Rust. Currenlty it clones images from websites (but it is configurable).

### Run it
Allow reqwest crate to work propertly:
```
sudo apt install libssl-dev
```

Compile and run application:
```
cargo run
```

### Future release
Take a look at [this project](https://github.com/goto-eof/rust-multithread-sample-tokio) that will be included in the release 0.2.0 of OctoScraper in order to improve application performance (multithreading). 

```
    ___     _        __                                
    /___\___| |_ ___ / _\ ___ _ __ __ _ _ __   ___ _ __ 
   //  // __| __/ _ \\ \ / __| '__/ _` | '_ \ / _ \ '__|
  / \_// (__| || (_) |\ \ (__| | | (_| | |_) |  __/ |   
  \___/ \___|\__\___/\__/\___|_|  \__,_| .__/ \___|_|   
                                       |_|              
```    


Tested on Linux and MacOS.
