use std::thread::spawn;
use std::sync::{Mutex, RwLock};
use anyhow::Result;

#[derive(Debug)]
struct User {
    name: RwLock<String>,
}

fn main() -> Result<()> {
    let user1 = User { name: RwLock::new("dogs".to_string()) };
    
    println!("Hello {:?}", &user1.name);
    let t1 = spawn(move || -> Result<User> {
        let mut user_name = user1.name
            .write()
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        user_name.push_str(" and fish");        
        println!("{:?}", user_name);
        let user = User { name: RwLock::new(user_name.to_string()) };
        Ok::<User, anyhow::Error>(user)
    });
    
    println!("Hello {:?}", t1.join().unwrap()?.name.read());
    // t1.join().unwrap()?;
    Ok(())
}