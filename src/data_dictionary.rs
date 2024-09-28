use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs;
pub mod data_dictionary {
    #[derive(Debug, Deserialize)]
    struct DataDictionary {
        #[serde(rename = "Header")]
        header: Header,
        #[serde(rename = "Body")]
        body: Body,
        #[serde(rename = "Trailer")]
        trailer: Trailer,
    }

    #[derive(Debug, Deserialize)]
    struct Header {
        // Define fields in the Header part of the XML
        // Match these to the actual XML file
    }

    #[derive(Debug, Deserialize)]
    struct Body {
        // Define fields in the Body part of the XML
        // Match these to the actual XML file
    }

    #[derive(Debug, Deserialize)]
    struct Trailer {
        // Define fields in the Trailer part of the XML
        // Match these to the actual XML file
    }
}