use std::fs;
pub fn grab_apps() -> Vec<String> {
    // Read ./apps/ directory and creates a vector with all files in it
    let paths = fs::read_dir("/home/blub/Projects/rust/salad/src/apps/")
       .unwrap()
       .filter_map(|e| e.ok())
       .map(|e| e.path())
       .collect::<Vec<_>>();
    
    // Filter out and clean up vector so we can return just the app names
    let mut app_vec = Vec::new();
    for i in &paths {
        app_vec.push(i.clone().into_os_string().into_string().unwrap());
    }
    println!("{:?}", &app_vec);
    for i in &mut app_vec {
        let stripped = i.split('/').last().unwrap().to_string();
        *i = stripped.split('.').next().unwrap().to_string();
    }
    // debug statement 
    println!("{:?}", &app_vec);
    return app_vec;
}

