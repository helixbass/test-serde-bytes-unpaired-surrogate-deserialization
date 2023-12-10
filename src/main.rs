use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Inner {
    pub whee: serde_bytes::ByteBuf,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum TaggedEnum {
    Inner(Inner),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum UntaggedEnum {
    Inner(Inner),
}

#[derive(Debug, Deserialize)]
struct OuterStruct {
    pub inner: Inner,
}

fn main() {
    let just_inner_parsed = serde_json::from_str::<Inner>(r#"{
        "whee": "\ud83d"
    }"#);
    println!("just_inner_parsed: {just_inner_parsed:#?}");

    let tagged_parsed = serde_json::from_str::<TaggedEnum>(r#"{
        "type": "Inner",
        "whee": "\ud83d"
    }"#);
    println!("tagged_parsed: {tagged_parsed:#?}");

    let untagged_parsed = serde_json::from_str::<UntaggedEnum>(r#"{
        "whee": "\ud83d"
    }"#);
    println!("untagged_parsed: {untagged_parsed:#?}");

    let outer_struct_parsed = serde_json::from_str::<OuterStruct>(r#"{
        "inner": {
            "whee": "\ud83d"
        }
    }"#);
    println!("outer_struct_parsed: {outer_struct_parsed:#?}");

}
