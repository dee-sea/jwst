/// This function takes no arguments, and returns a vector of the urls of the files to be
/// downloaded.
pub fn get_links() -> Vec<String> {
    vec![
        "https://dl.dropboxusercontent.com/s/pc9vqyqm38361r5/Carena-composite.png".to_string(),
        "https://dl.dropboxusercontent.com/s/zyhqvrim8la8xcx/Carena-Nircam.png".to_string(),
        "https://dl.dropboxusercontent.com/s/yo4sb5wgazonssg/Deep_Field-Nircam.png".to_string(),
        "https://dl.dropboxusercontent.com/s/ayhybca9zfxj4rs/Southern_Ring-Nircam.png".to_string(),
        "https://dl.dropboxusercontent.com/s/d02stvegqppm16o/Stephan_Quintet-composite.png"
            .to_string(),
        "https://dl.dropboxusercontent.com/s/rfnpu8az41ypobf/Stephan_Quintet-Miri.png".to_string(),
    ]
}

/// This functions takes no arguments and returns a vector of the filenames of the files to be
/// saved loacally.
pub fn get_names() -> Vec<String> {
    vec![
        "Carena-composite.png".to_string(),
        "Carena-Nircam.png".to_string(),
        "Deep_Field-Nircam.png".to_string(),
        "Southern_Ring-Nircam.png".to_string(),
        "Stephan_Quintet-composite.png".to_string(),
        "Stephan_Quintet-Miri.png".to_string(),
    ]
}
