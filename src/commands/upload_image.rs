use crate::api::{get_client, Api, Clients, ImageUploadData};
use std::borrow::Cow;

use fs_err as fs;

use crate::options::{GlobalOptions, UploadImageOptions};

pub fn upload_image(global: GlobalOptions, options: UploadImageOptions) {
    let image_data = fs::read(options.path).expect("couldn't read input file");

    let mut client = get_client(global);

    let upload_data = ImageUploadData {
        image_data: Cow::Owned(image_data),
        name: &options.name,
        description: &options.description,
        group_id: None,
    };

    let response = match client {
        Clients::OpenCloud(ref mut open_cloud) => open_cloud
            .upload_image(upload_data)
            .expect("OpenCloud request failed"),
        Clients::RobloxApi(ref mut roblox_api) => roblox_api
            .upload_image(upload_data)
            .expect("Roblox Api request failed"),
    };

    eprintln!("Image uploaded successfully!");
    println!("{}", response.backing_asset_id);
}
