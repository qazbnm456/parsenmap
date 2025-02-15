use crate::models::helpers::get_attr_value_by_name;
use xml::attribute::OwnedAttribute;

Initializer! {
#[derive(Debug, Clone)]
    pub struct Service {
        pub name: String,
        pub product: String,
        pub version: String,
        pub extrainfo: String,
        pub servicefp: String,
        pub hostname: String,
        pub ostype: String,
        pub tunnel: String,
        pub conf: String,
    }
}

impl From<&Vec<OwnedAttribute>> for Service {
    fn from(attr: &Vec<OwnedAttribute>) -> Self {
        let mut new_item = Self::new();
        new_item.name = get_attr_value_by_name(attr, "name");
        new_item.product = get_attr_value_by_name(attr, "product");
        new_item.version = get_attr_value_by_name(attr, "version");
        new_item.extrainfo = get_attr_value_by_name(attr, "extrainfo");
        new_item.servicefp = get_attr_value_by_name(attr, "servicefp");
        new_item.hostname = get_attr_value_by_name(attr, "hostname");
        new_item.ostype = get_attr_value_by_name(attr, "ostype");
        new_item.tunnel = get_attr_value_by_name(attr, "tunnel");
        new_item.conf = get_attr_value_by_name(attr, "conf");
        new_item
    }
}
