use comfy_table::{ContentArrangement, Table};

#[derive(Serialize, Deserialize)]
pub struct DeviceRootJson {
    #[serde(rename = "Items")]
    pub items: Vec<DeviceDetails>,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceDetails {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LastUserName")]
    pub lastusername: String,
    #[serde(rename = "DateLastActivity")]
    pub lastactivity: String,
}

impl DeviceDetails {
    pub fn new(
        id: String,
        name: String,
        lastusername: String,
        lastactivity: String,
    ) -> DeviceDetails {
        DeviceDetails {
            id,
            name,
            lastusername,
            lastactivity,
        }
    }

    pub fn csv_print(devices: &[DeviceDetails]) {
        for device in devices {
            println!("{}, {}, {}", &device.id, &device.name, &device.lastusername);
        }
    }

    pub fn json_print(devices: &[DeviceDetails]) {
        println!("{}", serde_json::to_string_pretty(&devices).unwrap());
    }

    pub fn table_print(devices: Vec<DeviceDetails>) {
        let mut table = Table::new();
        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(120)
            .set_header(vec!["Device Id", "Device Name", "Last Used By"]);
        for device in devices {
            table.add_row(vec![&device.id, &device.name, &device.lastusername]);
        }
        println!("{table}");
    }
}
