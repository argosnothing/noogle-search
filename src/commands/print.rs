use crate::data::NoogleResponse;

pub fn execute(response: &NoogleResponse) {
    for doc in &response.data {
        for name in doc.all_names() {
            println!("{}", name);
        }
    }
}
