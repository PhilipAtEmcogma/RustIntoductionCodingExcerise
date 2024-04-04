

#[cfg(test)]
mod test{
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::fs::{OpenOptions, File};
    use std::io::prelude::*;
    use std::thread::{JoinHandle, spawn};
    
    #[test]
    fn tests_concurrency(){
        //this excerise is for the premises that we need to have acces to multiple thread or reading multiple files concurrently
        //i.e. multiple threads needs to read / write / mutate to the same file at the same time, concurrency is use to protect that
        // from happening so it won't corrupt the file.  thus when a thread is accessing a file, it'll lock that file, until it has
        // finish whatever it's doing, and other thread has to wait
        let file_mutex = Arc::new(Mutex::new(OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("increments.txt")
        .unwrap())); //open a file to write, if file didn't exist, allows rust to create it on the user's behalf
        //Mutex::new() prevents multiple threats trying to access the file at the same time.  it will look
        // the file when an threat is accessing it
        // Arc() allows rust to make multiple refference to the file, even though it's locked, to use for multi-threading
        
        let mut handles: Vec<JoinHandle<()>> = vec![];

        //create 10 threats
        for i in 0..10{
            let file_mutex = Arc::clone(&file_mutex);
            let handle:JoinHandle<()> = spawn(move ||{
                let mut file = file_mutex.lock().unwrap();
                writeln!(file, "{}", i).unwrap();

            });

            handles.push(handle);
        }

        //tell rust to wait for the spawned threads to finish successfully before main thread continues
        for handle in handles{
            handle.join().unwrap()
        }


    }
}